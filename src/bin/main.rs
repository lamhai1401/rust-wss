use std::io::{self, Write};

use futures::sink::SinkExt;
use futures::stream::StreamExt;
use structopt::StructOpt;
use url::Url;
use websocket_lite::{ClientBuilder, Message, Opcode, Result};

const CONNECTION: &'static str = "wss://signal-conference-staging.quickom.com";

// fn main() {
//     println!("Connecting to {}", CONNECTION);
// }

#[tokio::main]
async fn main() {
    println!("Hello async")
}
