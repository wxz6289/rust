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
            break counter;
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

/* #[test]
fn test_while_return_value() {
    let mut counter = 0;
    let result = while counter < 3 {
        println!("while again ...");
        counter += 1;
        if counter == 2 {
            break counter; // while 语句中不能使用break
        }
    };
    println!("result is {}", counter);
} */

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
