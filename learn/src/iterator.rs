#[test]
fn log() {
    use std::iter::{once, repeat};

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizz_buzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fizz_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }
}

#[test]
fn test_count() {
    use std::io::prelude::*;
    let stdin = std::io::stdin();
    let count = stdin.lock().lines().count();
    assert_eq!(count, 0);
}

#[test]
fn test_sum() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    assert_eq!(sum, 15);
    println!("{}", (1..10).sum::<i16>());
    println!("{}", (1..=5).product::<i16>());
}

#[test]
fn test_collect() {
    let numbers = vec![1, 2, 3, 4, 5];
    let collected: Vec<i32> = numbers.iter().cloned().rev().collect();
    assert_eq!(collected, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_max_min() {
    let numbers = vec![1, 2, 3, 4, 5, 0, 8, 2, 6];
    let max = numbers.iter().max();
    let min = numbers.iter().min().unwrap();
    assert_eq!(max, Some(&8i32));
    assert_eq!(min, &0);
}

#[test]
fn test_max_by_or_min_by() {
    let numbers = vec![1.0, 2.2, 3.1, std::f64::NAN, 4.0, 5.0, 0.0, 8.0, 2.0, 6.0];
    let max = numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
    let min = numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(max, Some(&8.0));
    assert_eq!(min, Some(&0.0));
}

#[test]
fn test_max_by_key() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let max: Option<(&&str, &i32)> = map.iter().max_by_key(|&(_, v)| v);
    assert_eq!(max, Some((&"c", &3)));
    let min: Option<(&&str, &i32)> = map.iter().min_by_key(|&(_, v)| v);
    assert_eq!(min, Some((&"a", &1)));
}

#[test]
fn test_eq_lt_gt() {
    let packed = "Hello of Troy";
    let spaced = "Hello   of   Troy";
    let obscure = "Hello of Sandusky";
    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    assert!(packed.split_whitespace().gt(obscure.split_whitespace()));
}

#[test]
fn test_any() {
    let id = "Iterator";
    assert!(id.chars().any(|c| c.is_uppercase()));
    assert!(!id.chars().all(|c| c.is_lowercase()));
}
