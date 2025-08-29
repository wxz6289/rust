use std::time::{ Duration, Instant };

#[test]
fn test_if() {
    let num = 3;
    if num < 5 {
        println!("{} <= 5", num);
    } else {
        println!("5 <= {}", num);
    }
}

#[test]
fn test_if_else_if() {
    let num = 4;
    if num % 2 == 0 {
        println!("num is divisible by 2");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 4 == 0 {
        println!("num is divisible by 4");
    } else {
        println!("other");
    }
}

#[test]
fn test_if_let() {
    let condition = false;
    let num = if condition { 5 } else { 6 };
    println!("num is {}", num);
}

#[test]
fn test_loop() {
    let mut counter = 0;
    loop {
        println!("loop!");
        counter += 1;
        if counter >= 3 {
            break;
        }
    }
}

#[test]
fn test_loop_return_value() {
    let mut counter = 0;
    let result = loop {
        println!("loop!");
        counter += 1;
        if counter >= 3 {
            break counter * 2;
        }
    };
    println!("result is {}", result);
}

#[test]
fn test_while() {
    let mut counter = 0;
    while counter < 3 {
        println!("while again ...");
        counter += 1;
    }
    println!("result is {}", counter);
}

#[test]
fn test_while_return_value() {
    let mut counter = 0;
    let result = 'counter: while counter < 3 {
        println!("while again ...");
        counter += 1;
        if counter == 2 {
            break 'counter;
            // break 'counter 32; // 带返回值的break语句需要在内层循环或loop中使用
        }
    };
    println!("result is {:?}", result);
}

#[test]
fn test_loop_label() {
    let mut counter = 0;
    let result = 'outer: loop {
        println!("counter is {}", counter);
        let mut remaining = 10;
        loop {
            println!("remaining is {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'outer remaining;
            }
            remaining -= 1;
        }
        counter += 1;
    };
    println!("result is {:?}", result);
}

#[test]
fn test_iterate_array() {
    let a = [10, 20, 12, 32];
    let mut idx = 0;
    while idx < 3 {
        println!("The value is {} at {}", a[idx], idx);
        idx += 1;
    }
}

#[test]
fn test_iterate_for() {
    let a = [10, 20, 12, 32];
    for val in a.iter() {
        println!("The value is {} ", val);
    }
}

#[test]
fn test_range_for() {
    for num in (1..4).rev() {
        println!("The num is {} ", num);
    }
    println!("finished!");
}

#[test]
fn test_vec() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let third = &v[3];
    let fifth = &v[4];
    println!("{} {}", third, fifth);
}

#[test]
fn test_vec2() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let fifth = v.pop().expect("vector empty");
    assert_eq!(fifth, "105");
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    println!("{:?}", v);
    let third = std::mem::replace(&mut v[2], "king".to_string());
    assert_eq!(third, "103");
    println!("{:?}", v);
}

#[test]
fn print_string() {
    let mut s = vec!["King".to_string(), "Dreamer".to_string()];
    for mut t in s {
        t.push('!');
        println!("{}", t);
    }
}

#[test]
fn test_swap() {
    struct Person {
        name: Option<String>,
        birth: u32,
    }
    let mut composers = Vec::new();
    composers.push(Person { name: Some("King".to_string()), birth: 23 });
    // let name = composers[0].name;
    // let name = std::mem::replace(&mut composers[0].name, None);
    let name = composers[0].name.take();
    println!("{:?}", name);
    println!("{:?}", composers[0].name);
}

#[test]
pub fn test_time() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();
    while Instant::now() - start < time_limit {
        count += 1;
    }
    println!("{}", count);
}

#[test]
pub fn test_loop2() {
    let a = loop {
        break 123;
    };
    println!("{}", a);
}
