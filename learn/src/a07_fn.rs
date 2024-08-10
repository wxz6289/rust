use std::cmp::PartialOrd;

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

fn general_max<T>(list: &[T]) -> T {
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

