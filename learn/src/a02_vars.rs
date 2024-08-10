#[test]
pub fn print_vars() {
    let name = "Brand";
    let s = "hello".to_string();
    let mut age = 36;
    println!("Name is {} age is {}", name, age);
    age = 20;
    println!("Name is {} age is {}", name, age);

    const ID: i32 = 0001;
    println!("ID is {}", ID);
    let (name, age) = (name, age);
    println!("Name is {} age is {}", name, age);
}

#[test]
pub fn test_var() {
    let mut x = 5;
    println!("{x}");
    x = x + 1;
    println!("{x}");
}

#[test]
pub fn var() {
    let x = 5;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
}

#[test]
pub fn mut_var() {
    let mut x= 15;
    println!("{}", x);
    x = x + 1;
    println!("{}", x);
}

#[test]
pub fn shade_var() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner {}", x);
    }
    println!("out {}", x);
}

#[test]
pub fn repeat_var() {
    let spaces = "     ";
    let spaces = spaces.len();
    println!("repeat {}", spaces);
}

#[test]
pub fn mut_const_var() {
    let x = 32;
    println!("{x}");
    let mut x = 23;
    x = 33;
    println!("{x}");
}
