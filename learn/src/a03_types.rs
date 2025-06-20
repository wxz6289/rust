use num::integer::sqrt;

#[test]
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

    let x = 7_2_02i16;
    println!("{x}");
}

#[test]
pub fn dtype() {
    let guess: u32 = "42".parse().expect("Invalid type");
    println!("{:?}", guess);
    let guess: i32 = "xxx".parse().expect("Invalid type");
    println!("{:?}", guess);
}

#[test]
pub fn test_float() {
    println!("{}",(-1./ f32::INFINITY).is_sign_negative());
    println!("{}",-f32::MIN);
    println!("{}",-f32::MAX);
    println!("{}", 2f64.sqrt());
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    // 方法调用优先级高于后缀运算符
    println!("{}", (-1.01f64).floor());
}

#[test]
pub fn test_bool() {
    println!("{}", false as i8);
    println!("{}", true as i8);
    let x = 1;
    if x != 0 {
        println!("{}", "ok");
    }
}

// Rust中的字符类型char会以32位值表示单个Unicode字符。Rust会对单个字符使用char类型，但对字符串和文本流使用UTF-8编码。String会将起文本表示为UTF-8字节序列，而不是字符数组。

#[test]
pub fn test_different_type_add(){
    let a = 32u32;
    let b = 21i8;
    let c = a as i8 + b;
    println!("{}", c);
}

#[test]
pub fn test_number_array() {
    let var_name = [
        42.023,
        32f32,
        20.0_f32
    ];
    let arr = var_name;
    println!("{:02}", arr[0].round());
}