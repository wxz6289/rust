pub fn run(){
    let mut c = Color {
        red: 255,
        green: 255,
        blue: 253,
    };
    c.red = 120;
    println!("{:?}", (c.red, c.green, c.blue));

    // Tuple Struct
    let mut c2 = Color2(110, 20);
    c2.0 = 21;
    println!("{:?}", (c2.0, c2.1));
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Color2(u8, u8);
