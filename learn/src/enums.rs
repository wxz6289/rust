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

pub fn run() {
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Left;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}