#[test]
pub fn print() {
    println!("Hello, world!");
    println!("{} from {}", 1, "Number");
    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "brand", "Mass", "Code"
    );
    // Named Arguments
    println!(
        "{name} likes to paly {activity}",
        name = "Joe",
        activity = "Baseball"
    );
    // Placeholder trait
    println!("Binary: {:b}; Hex: {:x}; Octal: {:o}", 10, 10, 10);
    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));
    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
    let (name, age) = ("King", 20);
    println!("Name is {name} age is {age}");

    let x = 23;
    println!("x is {x}")
}
