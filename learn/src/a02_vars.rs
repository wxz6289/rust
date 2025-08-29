#[test]
pub fn learn_vars() {
    let a;
    // println!("a is {}", a);
    a = 10;
    println!("a is {}", a);
}

#[test]
pub fn learn_const() {
    println!("PI is {}", PI);
}
const PI: f64 = 3.1415926;

#[test]
pub fn learn_shadow() {
    println!("PI is {}", PI);
}

#[test]
pub fn learn_mut() {
    let a = 2;
    println!("a is {}", a);
    let mut a = 10;
    println!("a is {}", a);
    a = 20;
    println!("a is {}", a);
}

#[test]
pub fn print_vars() {
    let name: &'static str = "Brand";
    let s = "hello".to_string();
    let mut age = 36;
    println!("Name is {} {} age is {}", name, s, age);
    age = 20;
    println!("Name is {} age is {}", name, age);

    const ID: i32 = 0001;
    println!("ID is {}", ID);
    let (name, s1, age) = (name, s, age);
    println!("Name is {} {s1} age is {}", name, age);
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
    let mut x = 15;
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
