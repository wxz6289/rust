#[test]
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 255,
        blue: 253,
    };
    c.red = 120;
    println!("{:?}", (c.red, c.green, c.blue));

    // Tuple Struct
    let mut c2 = TColor(110, 20);
    c2.0 = 21;
    println!("{:?}", (c2.0, c2.1));
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct TColor(u8, u8);

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
        sign_in_count: 20000,
    };
    user.active = false;
    println!(
        "{:?}",
        (user.name, user.email, user.sign_in_count, user.active)
    );

    user = build_user(String::from("king"), String::from("king@gamil.com"));
    // println!("{:?}", (user.name, user.email, user.sign_in_count, user.active));

    let user2 = User {
        name: String::from("dreamer"),
        ..user
    };
    println!(
        "{:?}",
        (user2.name, user2.email, user2.sign_in_count, user2.active)
    );

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
pub fn run2() {
    /* let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("{:?}", numbers);
    for x in numbers.iter() {
        println!("{:?}", x);
    }

    for x in numbers.iter_mut() {
        *x * = 2;
    }
    println!("{:?}", numbers); */
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    p.set_first_name("Wang");
    println!("Person Tuple {:?}", p.to_tuple());
}
