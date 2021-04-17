pub fn run() {
    let mut hello = String::from("hello King");

    println!("{}", hello);
    println!("Length: {}", hello.len());
    hello.push('W');
    println!("{}", hello);
    println!("{}", hello.capacity());
    println!("{}", hello.is_empty());
    println!("{}", hello.contains("hello"));
    println!("{}", hello.replace("hello", "world"));

    for word in  hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
    println!("{}", s);
}