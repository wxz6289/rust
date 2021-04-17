pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("{:?}", numbers);
    for x in numbers.iter() {
        println!("{:?}", x);
    }

    for x in numbers.iter_mut() {
        *x * = 2;
    }
    println!("{:?}", numbers);
}