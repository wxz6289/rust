/* fn main() {
    let data = "\
    common name, length(cm)
    Little penguin, 33
    Fiordland penguin, 65
    Invalid, data
    ";
    let records = data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|filed| {filed.trim()})
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
} */

/* #[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice, Rye, Spelt, Wheat
}
fn main() {
    let mut grains = vec![];
    grains.push(Cereal::Rye);
    drop(grains);
    println!("{:?}", grains);
}
 */

/*  use std::thread;

 fn main() {
    let mut data = 100;
    thread::spawn(move|| { data = 200; });
    thread::spawn(move|| { data = 300; });
    println!("{}", data);
 } */

/*   fn main() {
    let mut letters = vec!["a", "b", "c"];
    for letter in letters {
        println!("{}", letter);
        // letters.push(letter.clone());
    }
  } */

 use std::rc::Rc;
 use std::sync::{Arc, Mutex};

 fn main() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
 }