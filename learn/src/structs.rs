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

struct Point2(u32, u32);


struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn show_user() {
    let mut user = User {
        name: String::from("wxz"),
        email: String::from("wxz@gamil.com"),
        active: true,
        sign_in_count: 20000
    };
    user.active = false;
    println!("{:?}", (user.name, user.email, user.sign_in_count, user.active));

    user = build_user(String::from("king"), String::from("king@gamil.com"));
    // println!("{:?}", (user.name, user.email, user.sign_in_count, user.active));

    let user2 = User {
        name: String::from("dreamer"),
        ..user
    };
    println!("{:?}", (user2.name, user2.email, user2.sign_in_count, user2.active));
      
    let point = Point2(12, 32);
    println!("{:?}", (point.0, point.1));
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        active: false,
        sign_in_count: 0,
    }
}
