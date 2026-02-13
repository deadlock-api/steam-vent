use crate::net::NetworkError;
use bytes::{Bytes, BytesMut};
use futures_util::{Sink, SinkExt, StreamExt, TryStreamExt};
use reqwest::{Client, Proxy};
use reqwest_websocket::{Message as ReqwestWsMessage, RequestBuilderExt};
use rustls::{ClientConfig, KeyLogFile, RootCertStore};
use std::future::ready;
use std::sync::Arc;
use tokio_stream::Stream;
use tokio_tungstenite::tungstenite::{Message as WsMessage, Message};
use tokio_tungstenite::{Connector, connect_async_tls_with_config};
use tracing::{debug, instrument};

type Result<T, E = NetworkError> = std::result::Result<T, E>;

#[instrument]
pub async fn connect(
    addr: &str,
) -> Result<(
    impl Sink<BytesMut, Error = NetworkError> + use<>,
    impl Stream<Item = Result<BytesMut>> + use<>,
)> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok(); // can only be once called
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let mut tls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    tls_config.key_log = Arc::new(KeyLogFile::new());
    let tls_config = Connector::Rustls(Arc::new(tls_config));
    let (stream, _) = connect_async_tls_with_config(addr, None, false, Some(tls_config)).await?;
    debug!("connected to websocket server");
    let (raw_write, raw_read) = stream.split();

    Ok((
        raw_write.with(|msg: BytesMut| ready(Ok(WsMessage::binary(msg)))),
        raw_read
            .map_err(NetworkError::from)
            .map_ok(Message::into_data)
            .map_ok(Bytes::from)
            .map_ok(BytesMut::from),
    ))
}

#[instrument]
pub async fn connect_with_proxy(
    addr: &str,
    proxy_url: Option<String>,
) -> Result<(
    impl Sink<BytesMut, Error = NetworkError> + use<>,
    impl Stream<Item = Result<BytesMut>> + use<>,
)> {
    // If no proxy, delegate to the standard connect
    if proxy_url.is_none() {
        let (sink, stream) = connect(addr).await?;
        // We need to erase the types to match the return type
        let sink =
            Box::pin(sink) as std::pin::Pin<Box<dyn Sink<BytesMut, Error = NetworkError> + Send>>;
        let stream =
            Box::pin(stream) as std::pin::Pin<Box<dyn Stream<Item = Result<BytesMut>> + Send>>;
        return Ok((sink, stream));
    }

    // Build the client with proxy support
    let mut client_builder = Client::builder();

    if let Some(proxy) = proxy_url {
        client_builder = client_builder.proxy(Proxy::all(&proxy)?);
    }

    let client = client_builder.build()?;

    // Connect to websocket using the upgrade flow
    let response = client.get(addr).upgrade().send().await?;
    let websocket = response.into_websocket().await?;

    debug!("connected to websocket server via proxy");
    let (raw_write, raw_read) = websocket.split();

    let sink = raw_write.with(|msg: BytesMut| ready(Ok(ReqwestWsMessage::Binary(msg.to_vec()))));
    let stream = raw_read
        .map_err(NetworkError::from)
        .map_ok(|msg| match msg {
            ReqwestWsMessage::Binary(data) => Bytes::from(data),
            _ => Bytes::new(),
        })
        .map_ok(BytesMut::from);

    let sink =
        Box::pin(sink) as std::pin::Pin<Box<dyn Sink<BytesMut, Error = NetworkError> + Send>>;
    let stream = Box::pin(stream) as std::pin::Pin<Box<dyn Stream<Item = Result<BytesMut>> + Send>>;
    Ok((sink, stream))
}
