// mod hello;
// mod hello_async;
// mod transform_main;
// mod test_redis_connect;
mod redis_client;

fn main() {
    // transform_main::transform_main();
    // test_redis_connect::main().unwrap();
    redis_client::main();
}
