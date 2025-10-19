#[test]
fn print_padova() {
    let mut padova = vec![1, 1, 1];
    for i in 3..10 {
        let next = padova[i - 3] + padova[i - 2];
        padova.push(next);
    }
    println!("{:?}", padova);
}
#[test]
fn print_struct() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}

struct Person {
    name: String,
    birth: i32,
}
#[test]
fn test_person() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: "King".to_string(),
        birth: 1990,
    });
    composers.push(Person {
        name: "Dreamer".to_string(),
        birth: 2026,
    });
    composers.push(Person {
        name: "Panpan".to_string(),
        birth: 2000,
    });
    for composer in composers {
        println!("{} -> {}", composer.name, composer.birth);
    }
}

#[test]
fn transform() {
    let s = vec!["udon".to_string(), "King".to_string(), "wxz".to_string()];
    let t = s;
    // let u = s;
    println!("{:?}", t);
}

#[test]
fn transform1() {
    let s = vec!["udon".to_string(), "King".to_string(), "wxz".to_string()];
    let t = s.clone();
    let u = s.clone();
    println!("{:?} {:?} {:?}", t, s, u);
}

#[test]
pub fn test_own() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s);

    let s2 = String::from("world");
    take_ownership(s2.clone());
    println!("s2 = {}", s2);

    let x = 5;
    let y = x;
    make_copy(x);
    println!("x = {}, y = {}", x, y);
}

fn take_ownership(s: String) {
    println!("{s}");
}

fn make_copy(x: i32) {
    println!("{x}");
}

#[test]
fn test_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

#[test]
fn test_mut_reference() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &s1;
    s2.push_str(", world");
    println!("{}", s2);
}

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 42;
        // r = &x;
    }
    // println!("r: {}", r);
}

/*
#[test]
fn test_dangling_reference2() {
    let r = dangle();
    println!("r: {}", r);
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

#[test]
fn test_slice() {
    let mut s = String::from("hello World");
    let word = first_word(&s);
    // s.clear();
    println!("slice: {}", word);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
