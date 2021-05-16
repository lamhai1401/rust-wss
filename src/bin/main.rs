extern crate wss;
use wss::conn::client::Client;

// use std::sync::{
//     atomic::{AtomicUsize, Ordering},
//     Arc,
// };

// static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

// use std::collections::HashMap;
// use tokio::sync::{mpsc, RwLock};

// type Clients = Arc<RwLock<HashMap<String, Client>>>;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut client = Client::new();
    let result = client.connect().await.expect("Cannot connect");

    println!("{:?}", result);
    // tokio::spawn(read_stdin(stdin_tx));

    // let (write, read) = ws_stream.split();

    // let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    // let ws_to_stdout = {
    //     read.for_each(|message| async {
    //         let data = message.unwrap().into_data();
    //         println!("Receive msg: {:?}", data);
    //         // tokio::io::stdout().write_all(&data).await.unwrap();
    //     })
    // };
    // pin_mut!(stdin_to_ws, ws_to_stdout);
    // future::select(stdin_to_ws, ws_to_stdout).await;

    // println!("{}", y);
    // loop {
    //     thread::sleep(Duration::from_millis(4000))
    // }

    // let a1: [u32; 3] = serde_json::from_str("[\"hai\", 2, 3]").unwrap();

    // println!("{:?}", a1);

    // json_array();
    Ok(())
}

fn own(charrrr: String) {
    println!("{:?}", charrrr);
}

fn json_array() {
    use JsonValue::*;
    let input = r#"[42,"x"]"#;
    let expected: JsonValue = Array(vec![Num(42.0), Str("x".to_string())]);

    // let sub_vec: Vec<JsonValue> = expected.iter().map(|v| v[0]).collect();

    match expected {
        // Shape::Circle(_, value) => println!("value: {}", value),
        JsonValue::Num(value) => println!("{}", value),
        JsonValue::Str(value) => println!("{}", value),
        JsonValue::Array(value) => {
            println!("{:?}", value.get(0))
        }
        _ => println!("Something else"),
    }

    // println!("{:?}", expected);
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
