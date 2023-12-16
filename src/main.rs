use std::sync::Arc;
use tokio::runtime::Runtime;

mod broker;
mod control_plane;
mod info;
mod model;
mod position;
mod strategy;
mod subscription;
mod transaction;

fn main() {
    let runtime = Arc::new(Runtime::new().unwrap());
    println!("Hello, world!");
}
