use std::sync::Arc;
use tokio::runtime::Runtime;

mod info;
mod model;

fn main() {
    let runtime = Arc::new(Runtime::new().unwrap());
    println!("Hello, world!");
}
