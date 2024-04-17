#[test]
fn test_expression() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x = {}, y = {}", x, y);
}

fn five() -> i32 {
    5
}

#[test]
fn test_five() {
    let x = five();
    println!("five = {}", x);
}
