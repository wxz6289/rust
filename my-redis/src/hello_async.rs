use crate::hello::hello;

pub async fn main_async() {
    let op = hello();
    println!("op: {:?}", "world");
    op.await;
}
