use std::sync::Arc;
use tokio::runtime::Runtime;

mod broker;
mod info;
mod model;
mod subscription;

fn main() {
    let runtime = Arc::new(Runtime::new().unwrap());
    println!("Hello, world!");
}
