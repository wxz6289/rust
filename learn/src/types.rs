pub fn run() {
    // Default is i32
    let _x = 1;
    // Default is f64
    let _y = 3.2;

    let _z: i64 = 34;

    println!("Max i32: {}", std::i32::MAX);
    println!("Min i64: {}", std::i64::MIN);

    let is_active: bool = true;
    let is_greeter = 10 > 5;
    let face = '\u{1f600}';
    println!("{:?}", (_x, _y, _z, is_active, face));
}

pub fn dtype() {
    let guess: u32 = "42".parse().expect("Invalid type");
    println!("{:?}", guess);
}
