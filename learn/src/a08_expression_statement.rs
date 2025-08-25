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

#[test]
fn test_while() {
    let mut n = 5;
    while n > 0 {
        println!("n > {}", n);
        n -= 1;
    }
}

#[test]
fn test_loop() {
    let mut n = 5;
    loop {
        if n == 0 {
            break;
        }
        println!("n > {}", n);
        n -= 1;
    }
}

#[test]
fn test_while_let() {
    let mut n = 5;
    while let Some(x) = Some(n) {
        if x == 0 {
            break;
        }
        println!("n > {}", x);
        n -= 1;
    }
}

#[test]
fn test_for_in() {
    for n in 1..=5 {
        println!("n > {}", n);
    }
}

#[test]
fn test_for_in_string() {
    let strs = ["one".to_string(), "two".to_string(), "three".to_string()];
    for s in strs.iter() {
        println!("s > {} {:p}", *s, s);
    }
    println!("strs > {:?}", strs);
}

#[test]
fn test_fn_generic_param() {
    // let mut v = Vec::<u8>::with_capacity(10);
    let mut v = Vec::with_capacity(10);
    for i in (0..10).rev() {
        v.push(i);
    }
    println!("v > {:?}", v);

    // let gv = (0..7).collect::<Vec<_>>();
    // let gv = (0..7).collect::<Vec<u8>>();
    let gv: Vec<u8> = (0..7).collect();
    println!("gv > {:?}", gv);
}