#[test]
fn create_vector() {
    let mut v: Vec<u32> = Vec::new();
    println!("creating vector with Vec: {:#?}", v);
    v.push(3);
    v.push(12);
    println!("{:#?}, {}", v, v.len());
}

#[test]
fn create_vector_with_vec_macro() {
    let v = vec![1, 2, 5];
    println!("{:#?}", v);
}

#[test]
fn visit_vector() {
    let v = vec![1, 2, 8, 2];
    let e = &v[2];
    println!("{}", e);

    let f = v[3];
    println!("with index {}", f);

    match v.get(4) {
        Some(x) => println!("{}", x),
        None => println!("no value found"),
    }
}

#[test]
fn test_vector() {
    let mut v = vec![1, 2, 3, 4];
    let first = &v[1];
    // v.push(6); // 可能会因为空间不够，重新分配新的内存空间，导致指向被释放的内存
    println!("{}", first);
}

#[test]
fn traverse_vector() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    for i in v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    for i in &mut v {
        *i *= 2;
    }
    println!("{:#?}", v);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[test]
fn save_different_values() {
    let row = vec![
        SpreadSheetCell::Text(String::from("Hello")),
        SpreadSheetCell::Int(23),
        SpreadSheetCell::Float(3.14156),
    ];
    println!("{:#?}", row);
}
