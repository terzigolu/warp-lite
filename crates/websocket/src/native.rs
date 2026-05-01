//! warp-lite stub for the native WebSocket connector.
//!
//! Original code dialed websockets (with optional HTTP proxy + TLS via
//! rustls-platform-verifier) to power Warp's session sharing and graphql
//! subscription transport. warp-lite never opens an outbound websocket, so
//! the connect path is now an error return; the type aliases remain so that
//! downstream callers (graphql subscriptions, sharer/viewer) still compile.
use async_tungstenite::{
    tokio::ClientStream, tungstenite::client::IntoClientRequest, WebSocketStream,
};
use futures::{Sink, Stream};
use futures_util::StreamExt as _;
use tokio::net::TcpStream;

use crate::WebsocketMessage;

pub use async_tungstenite::tungstenite::Message;

pub struct WebSocket(WebSocketStream<ClientStream<TcpStream>>);

pub async fn connect(_request: impl IntoClientRequest + Unpin) -> anyhow::Result<WebSocket> {
    Err(anyhow::anyhow!(
        "warp-lite: outbound websocket connections are disabled"
    ))
}

impl WebSocket {
    pub async fn split(
        self,
    ) -> (
        impl Sink<Message, Error = Error>,
        impl Stream<Item = Result<Message, Error>>,
    ) {
        self.0.split()
    }

    pub async fn into_graphql_client_builder(self) -> graphql_ws_client::ClientBuilder {
        graphql_ws_client::Client::build(self.0)
    }
}

pub type Error = async_tungstenite::tungstenite::Error;

impl WebsocketMessage for Message {
    fn new_binary(bytes: Vec<u8>) -> Self {
        Self::Binary(bytes)
    }

    fn binary(&self) -> Option<&[u8]> {
        match self {
            Message::Binary(bytes) => Some(bytes.as_ref()),
            _ => None,
        }
    }

    fn new_text(text: String) -> Self {
        Self::Text(text)
    }

    fn new(text: String) -> Self {
        Self::new_text(text)
    }

    fn text(&self) -> Option<&str> {
        match self {
            Message::Text(text) => Some(text),
            _ => None,
        }
    }
}
