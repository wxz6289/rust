pub fn run() {
    let age  = 22;
    let check_id: bool = true;
    if age > 21 && check_id {
        println!("{} > 21", age);
    } else {
        println!("{} < 21", age);
    }

    let is_of_age = if age > 21 { true } else { false };
    println!("{}", is_of_age);
}