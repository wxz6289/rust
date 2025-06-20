use std::cmp::PartialOrd;
use std::ops::Add;
use std::time::Duration;

#[test]
fn test() {
    println!("Testing");
    another_test(12);
}

fn another_test(x: i8) {
    println!("another {}", x);
}


fn max(list: &[i32]) -> i32 {
    let mut m = list[0];
    for &item in list.iter() {
        if m < item {
            m = item;
        }
    }
    return m;
}

fn general_max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut m = list[0];
    for &item in list.iter() {
        if m < item {
            m = item;
        }
    }
    return m;
}

#[test]
fn test_max() {
    let array = vec![23, 12, 43, 56, 2, 90, 32];
    let m = max(&array);
    println!("max is {} ", m);
    let mx = general_max(&array);
    println!("general_max is {}", mx);
}

fn add_with_lifetime<'a, 'b>(i: & 'a i32, j: &'b i32) -> i32 {
    *i + *j
}

#[test]
fn test_add_with_lifetime(){
    let a = 10;
    let b = 20;
    let res = add_with_lifetime(&a, &b);
    println!("output: {}", res);
}


fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

#[test]
fn test_add(){
    let i = 23;
    let j = 32;
    let res = add(i,j);
    println!(" -> {:?}", res);
    let floats = add(1.2, 3.9);
    println!("floats = {}", floats);
    let durations = add( Duration::new(5,0), Duration::new(12, 2));
    println!("durations: {:?} ", durations);
}


fn search(){
    let search_terms = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned -- in search of what?
    It is the same with books.
    What do we seek through millions of pages?
    ";
    for line in quote.lines() {
        if line.contains(search_terms) {
            println!("{}", line);
        }
    }
}

#[test]
fn test_search(){
    search();
}