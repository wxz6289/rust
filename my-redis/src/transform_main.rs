use tokio;
use crate::hello::hello;

pub fn transform_main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        hello().await;
        println!("{}", "world");
    });
}