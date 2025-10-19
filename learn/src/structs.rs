use core::str;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
/* 元组结构体 只有字段类型,没有字段名 */
struct TColor(u8, u8);

#[test]
pub fn test_color() {
    let mut c = Color {
        red: 255,
        green: 255,
        blue: 253,
    };
    c.red = 120;
    println!("{} {} {}", c.red, c.green, c.blue);

    let mut c2 = TColor(110, 20);
    c2.0 = 21;
    println!("{} {}", c2.0, c2.1);
}

struct Point2(u32, u32);
#[test]
fn test_point() {
    let point = Point2(12, 32);
    println!("{} {}", point.0, point.1);
    let p = Point2(10, 20);
    println!("{} {}", p.0, p.1);
}

/* 类单元结构体 在类上实现trait但不需要在类中存储数据 */
#[derive(Debug)]
struct AlwaysEqual; // 没有()和{}

#[test]
fn test_always_equal() {
    let _subject = AlwaysEqual; // 创建类单元结构体实例时不需要()和{}
    println!("AlwaysEqual struct created {:?}", _subject);
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(name: String, email: String) -> User {
        User {
            name,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
}

#[test]
pub fn show_user() {
    let mut user = User {
        name: String::from("wxz"),
        email: String::from("wxz@gamil.com"),
        active: true,
        sign_in_count: 20000,
    };
    user.active = false;
    println!(
        "{:?}",
        (user.name, user.email, user.sign_in_count, user.active)
    );

    user = User::new(String::from("king"), String::from("king@gamil.com"));
    println!(
        "{:?}",
        (
            user.name.clone(),
            user.email.clone(),
            user.sign_in_count,
            user.active
        )
    );

    let user2 = User {
        name: String::from("dreamer"),
        email: String::from("dreamer@gamil.com"),
        ..user
    };
    println!(
        "user {:?}",
        (user.name, user.email, user.sign_in_count, user.active)
    );

    println!(
        "user2 {:?}",
        (user2.name, user2.email, user2.sign_in_count, user2.active)
    );

    let user3 = User::new(String::from("dreamer"), String::from("dreamer@gamil.com"));
    println!(
        "user3 {:?}",
        (user3.name, user3.email, user3.sign_in_count, user3.active)
    );

    let user4 = build_user(String::from("builder"), String::from("builder@gamil.com"));
    println!(
        "user4 {:?}",
        (user4.name, user4.email, user4.sign_in_count, user4.active)
    );
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        active: false,
        sign_in_count: 0,
    }
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

#[test]
pub fn test_person() {
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    p.set_first_name("Wang");
    println!("Person Tuple {:?}", p.to_tuple());
}

struct Rectangle(u32, u32);

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle(size, size)
    }
    fn area(&self) -> u32 {
        self.0 * self.1
    }
}

#[test]
pub fn test_rectangle() {
    let rect = Rectangle(30, 50);
    println!(
        "The rectangle width is {} and height is {} ",
        rect.0, rect.1
    );
    println!("The area of the rectangle is {}", rect.area());

    let square = Rectangle::square(40);
    println!(
        "The square width is {} and height is {} ",
        square.0, square.1
    );
    println!("The area of the square is {}", square.area());
}

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

trait Shape {
    fn area(&self) -> u32;
}

impl Rectangle2 {
    fn new(width: u32, height: u32) -> Rectangle2 {
        Rectangle2 { width, height }
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Shape for Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[test]
pub fn test_rectangle2() {
    let scale = 2;
    let rect = Rectangle2::new(40, dbg!(30 * scale));
    dbg!(&rect);
    let rect2 = Rectangle2::new(30, 50);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!(
        "The rectangle2 width is {} and height is {}",
        rect.width,
        rect.height()
    );
    println!("Rectangle2 struct {rect:?}");
    println!("Rectangle2 struct {rect:#?}");
    println!(
        "The rectangle2 width is {} and height is {} ",
        rect.width(),
        rect.height()
    );
    println!("The area of the rectangle2 is {}", rect.area());
}
