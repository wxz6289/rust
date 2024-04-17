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
