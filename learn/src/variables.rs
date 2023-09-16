pub fn vars() {
    let x = 5;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
}

pub fn vars2() {
    let mut x = 5;
    println!("{}", x);
    x = x + 1;
    println!("{}", x);
}

pub fn vars3() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);
}

pub fn run() {
    vars();
    vars2();
    vars3();
}
