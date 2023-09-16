pub fn loop_num() {
    let mut num = 0;

    let result = loop {
        num += 1;
        println!("current loop: {}", num);
        if num >= 10 {
            break num * 2;
        }
    };
    println!("final result: {}", result);
}
