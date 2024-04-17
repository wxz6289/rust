#[test]
pub fn tuple() {
    let tup = (-1, 20.2);
    println!("{}:{}", tup.0, tup.1);

    let (x, y) = tup;
    println!("{x},{y}");
    println!("{}", tup.1);
}
