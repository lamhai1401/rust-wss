use std::error;

pub const CONNECTION: &'static str = "wss://signal-conference-prod.quickom.com/?id=hailam";
// Change the alias to `Box<dyn error::Error>`.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
