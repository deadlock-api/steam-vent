use super::Result;
use crate::message::EncodableMessage;
use crate::net::{NetMessageHeader, RawNetMessage};
use crate::session::{hello, Session};
use crate::{
    connection::{ConnectionImpl, MessageFilter, MessageSender},
    transport::websocket::connect_with_proxy,
};
use crate::{ConnectionError, ServerList};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;
use steam_vent_proto::{steammessages_clientserver_login::CMsgClientHeartBeat, MsgKind};
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::{select, spawn};
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{debug, error};

#[derive(Clone)]
pub(crate) struct RawConnection {
    pub session: Session,
    pub filter: MessageFilter,
    pub timeout: Duration,
    pub sender: MessageSender,
    heartbeat_cancellation_token: CancellationToken,
    _heartbeat_drop_guard: Arc<DropGuard>,
}

impl Debug for RawConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawConnection").finish_non_exhaustive()
    }
}

impl RawConnection {
    pub async fn connect(
        server_list: &ServerList,
        proxy: Option<String>,
    ) -> Result<Self, ConnectionError> {
        let (read, write) = connect_with_proxy(&server_list.pick_ws(), proxy).await?;
        let filter = MessageFilter::new(read);
        let heartbeat_cancellation_token = CancellationToken::new();
        let mut connection = RawConnection {
            session: Session::default(),
            filter,
            sender: MessageSender {
                write: Arc::new(Mutex::new(write)),
            },
            timeout: Duration::from_secs(10),
            heartbeat_cancellation_token: heartbeat_cancellation_token.clone(),
            // We just store a drop guard using an `Arc` here, so dropping the last clone of `Connection` will cancel the heartbeat task.
            _heartbeat_drop_guard: Arc::new(heartbeat_cancellation_token.drop_guard()),
        };
        hello(&mut connection).await?;
        Ok(connection)
    }

    pub fn setup_heartbeat(&self) {
        let sender = self.sender.clone();
        let interval = self.session.heartbeat_interval;
        let header = NetMessageHeader {
            session_id: self.session.session_id,
            steam_id: self.session().steam_id,
            ..NetMessageHeader::default()
        };
        debug!("Setting up heartbeat with interval {:?}", interval);
        let token = self.heartbeat_cancellation_token.clone();
        spawn(async move {
            loop {
                select! {
                    _ = sleep(interval) => {},
                    _ = token.cancelled() => {
                        break
                    }
                };
                debug!("Sending heartbeat message");
                match RawNetMessage::from_message(header.clone(), CMsgClientHeartBeat::default()) {
                    Ok(msg) => {
                        if let Err(e) = sender.send_raw(msg).await {
                            error!(error = ?e, "Failed to send heartbeat message");
                        }
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to prepare heartbeat message")
                    }
                }
            }
            debug!("Heartbeat task stopping");
        });
    }
}

impl ConnectionImpl for RawConnection {
    fn timeout(&self) -> Duration {
        self.timeout
    }

    fn filter(&self) -> &MessageFilter {
        &self.filter
    }

    fn session(&self) -> &Session {
        &self.session
    }

    async fn raw_send_with_kind<Msg: EncodableMessage>(
        &self,
        header: NetMessageHeader,
        msg: Msg,
        kind: MsgKind,
        is_protobuf: bool,
    ) -> Result<()> {
        let msg = RawNetMessage::from_message_with_kind(header, msg, kind, is_protobuf)?;
        self.sender.send_raw(msg).await
    }
}
