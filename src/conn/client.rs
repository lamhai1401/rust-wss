use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::connect_async;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

pub struct WssClient {
    pub user_id: usize,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
    // conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

const CONNECTION: &'static str = "wss://signal-conference-prod.quickom.com/?id=hailam";

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users =
    Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>>>;

#[allow(dead_code)]
// #[derive(Debug)]
impl WssClient {
    pub async fn new() {
        let url = url::Url::parse(&CONNECTION).unwrap();

        // let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        // // tokio::spawn(read_stdin(stdin_tx));

        // let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        // WssClient {}
    }
}
