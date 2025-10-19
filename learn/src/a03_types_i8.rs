#[test]
fn test_from_str_radix() {
    assert_eq!(i8::from_str_radix("A", 16), Ok(10))
}

#[test]
fn test_count_ones() {
    let n = 0b100_0000i8;
    assert_eq!(n.count_ones(), 1);
    println!("{}", n.count_zeros())
}
#[test]
fn test_array() {
    let a = [1, 2, 3];
    assert_eq!(a.len(), 3);
    assert_eq!(a[0], 1);
}

#[test]
fn test_array_outflow() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Invalid input");
    assert_eq!(a.len(), 6);
    println!("out {}", a[index]);
}
