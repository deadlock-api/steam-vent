use crate::message::flatten_multi;
use crate::net::{NetworkError, RawNetMessage};
use futures_util::{Sink, SinkExt, Stream, StreamExt, TryStreamExt};
use reqwest::{Client, Proxy};
use reqwest_websocket::{Message as WsMessage, RequestBuilderExt};
use std::future::ready;
use tracing::{debug, instrument};

type Result<T, E = NetworkError> = std::result::Result<T, E>;

#[instrument]
pub async fn connect(
    addr: &str,
) -> Result<(
    impl Sink<RawNetMessage, Error = NetworkError>,
    impl Stream<Item = Result<RawNetMessage>>,
)> {
    connect_with_proxy(addr, None).await
}

#[instrument]
pub async fn connect_with_proxy(
    addr: &str,
    proxy_url: Option<String>,
) -> Result<(
    impl Sink<RawNetMessage, Error = NetworkError>,
    impl Stream<Item = Result<RawNetMessage>>,
)> {
    // Build the client with optional proxy support
    let mut client_builder = Client::builder();

    // Configure proxy if provided
    if let Some(proxy) = proxy_url {
        client_builder = client_builder.proxy(Proxy::all(&proxy)?);
    }

    let client = client_builder.build()?;

    // Connect to websocket using the upgrade flow
    let response = client.get(addr).upgrade().send().await?;

    let websocket = response.into_websocket().await?;

    debug!("connected to websocket server");
    let (raw_write, raw_read) = websocket.split();

    Ok((
        raw_write.with(|msg: RawNetMessage| {
            let mut body = msg.header_buffer;
            body.unsplit(msg.data);
            ready(Ok(WsMessage::Binary(body.to_vec())))
        }),
        flatten_multi(
            raw_read
                .map_err(NetworkError::from)
                .map_ok(|msg| match msg {
                    WsMessage::Binary(data) => data,
                    _ => vec![], // Handle other message types as needed
                })
                .map(|res| res.and_then(RawNetMessage::read)),
        ),
    ))
}
