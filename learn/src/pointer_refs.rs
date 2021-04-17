pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    let v1 = vec![1, 2, 3];
    let v2 = &v1;
    println!("Values: {:?}", (&v1, v2));
}