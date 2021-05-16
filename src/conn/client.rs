use crate::utils::{
    cst::{Result, CONNECTION},
    rd::rand_id,
};
// use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use futures_channel::mpsc::UnboundedSender;
use futures_util::{future, pin_mut, stream::StreamExt};

extern crate serde_json;
use serde_json::Value;

use std::error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    // pub sender:
    //     Option<futures_channel::mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            topics: vec![],
            user_id: rand_id(),
        }
    }

    pub async fn connect(&mut self) -> Result<String> {
        use JsonValue::*;
        let url = url::Url::parse(&CONNECTION).unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("Cannot connect");
        let (write, read) = ws_stream.split();
        // println!("WebSocket handshake has been successfully completed");
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        // tokio::spawn(self.read_stdin(stdin_tx));

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let handleReading = {
            read.for_each(|message| async {
                // let data = message.unwrap().into_data();
                let data: String = message.unwrap().into_text().unwrap();
                println!("Receive msg: {:?}", data);

                // let expected: JsonValue = Array(vec![Num(42.0), Str("x".to_string())]);
                if data != "" {
                    let expected: Value = serde_json::from_str(data.as_str()).unwrap();
                    match expected {
                        // Shape::Circle(_, value) => println!("value: {}", value),
                        Value::Array(value) => {
                            println!("SignalID: {:?}, StreamID {:?}", value.get(0), value.get(1));
                        }
                        _ => println!("Something else"),
                    }
                }

                // let parsed: serde_json::Value =
                //     serde_json::from_str(&data).expect("Can't parse to JSON");
                // serde_json::from_str(&data).unwrap();
                // tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, handleReading);
        future::select(stdin_to_ws, handleReading).await;
        Ok(String::from(
            "WebSocket handshake has been successfully completed",
        ))
    }

    async fn handleData() -> Result<String> {
        Ok(String::from("HandleData Ok"))
    }

    // Our helper method which will read data from stdin and send it along the
    // sender provided.
    // async fn read_stdin(&mut self, ctx: futures_channel::mpsc::UnboundedSender<Message>) {
    //     let mut stdin = tokio::io::stdin();
    //     loop {
    //         let mut buf = vec![0; 1024];
    //         let n = match stdin.read(&mut buf).await {
    //             Err(_) | Ok(0) => break,
    //             Ok(n) => n,
    //         };
    //         buf.truncate(n);
    //         tx.unbounded_send(Message::binary(buf)).unwrap();
    //     }
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
}
