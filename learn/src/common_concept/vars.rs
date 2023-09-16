pub fn var() {
    let x = 5;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
}

pub fn mut_var() {
    let mut x = 15;
    println!("{}", x);
    x = x + 1;
    println!("{}", x);
}

pub fn shade_var() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner {}", x);
    }
    println!("out {}", x);
}

pub fn repeat_var() {
    let spaces = "     ";
    let spaces = spaces.len();
    println!("repeat {}", spaces);
}
