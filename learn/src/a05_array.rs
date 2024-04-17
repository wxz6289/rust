#[test]
pub fn test_array() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    numbers[1] = 22;
    println!("{}", numbers[0]);
    println!("{}", numbers[1]);
    println!("{}", numbers.len());
    println!("{}", std::mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
    let a = [3; 5];
    let idx = 5;
    println!("{:?}", a);
    // println!("{:?}", a[idx]);
    let b: [f32; 3] = [1f32, 2f32, 3f32];
    println!("{:?}", b);
}
