pub fn truple() {
    let tup: (i8, u8) = (-1, 20);
    println!("{}:{}", tup.0, tup.1);

    let (x, y) = tup;
    println!("{},{}", x, y);
}