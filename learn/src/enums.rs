enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar move up"),
        Movement::Down => println!("Avatar move Down"),
        Movement::Left => println!("Avatar move Left"),
        Movement::Right => println!("Avatar move Right"),
    }
}
#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum IpAddress2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    }, // 匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn run() {
        let avatar1 = Movement::Down;
        let avatar2 = Movement::Up;
        let avatar3 = Movement::Right;
        let avatar4 = Movement::Left;
        move_avatar(avatar1);
        move_avatar(avatar2);
        move_avatar(avatar3);
        move_avatar(avatar4);
    }

    #[test]
    fn test_ip_kind() {
        let home = IpAddress {
            kind: IpAddressKind::V4,
            address: String::from("127.0.0.1")
        };
        let s = IpAddressKind::V6;
        println!("{:?}", home);
        println!("{:?}", s);
    }

    #[test]
    fn test_enum_ip() {
        let lookup = IpAddress2::V6(String::from("::1"));
        let home = IpAddress2::V4(127, 0, 0, 1);
        println!("{:?}", lookup);
        println!("{:?}", home);
    }
    #[test]
    fn test_message() {
        let m = Message::Write(String::from("Test"));
        m.call();
    }
    #[test]
    fn test_option() {
        let t = Some(5);
        let t = t.unwrap() + 1;
        println!("{:?}", t);
    }
}
