pub fn print_vars() {
    let name = "Brand";
    let mut age = 36;
    println!("Name is {} age is {}", name, age);
    age = 20;
    println!("Name is {} age is {}", name, age);

    const ID: i32 = 0001;
    println!("ID is {}", ID);
    let (name, age) = (name, age);
    println!("Name is {} age is {}", name, age);
}