use std::rc::Rc;
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

#[test]
fn test_rc() {
    let r = Rc::new("Dreamer".to_string());
    let t = r.clone();
    let u = t.clone();
    assert!(r.contains("Dreamer"));
    assert_eq!(t.find("ream"), Some(1));
    println!("{} {} {}", r, t, u);
}

fn show(table: &Table){
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!("  {} ", work);
        }
    }
}

#[test]
fn test_show() {
    let mut table = Table::new();
    table.insert("King".to_string(), vec!["Python".to_string(), "React".to_string()]);
    table.insert("Dreamer".to_string(), vec!["JavaScript".to_string(), "vue".to_string()]);
    show(&table);
    assert_eq!(table["Dreamer"][1], "vue");
}

#[test]
fn test_ref(){
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);
    println!("{} {}", r, *r);
}


#[test]
fn test_mut_ref(){
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);
}

#[test]
fn test_ref2() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;
    if b {
        r = &y;
    }
    assert_eq!(*r, 20);
}

struct Point {
    x: i32,
    y: i32
}

#[test]
fn test_multi_ref_struct(){
    let x = Point { x: 12, y: 10};
    let r = &x;
    let rr = &r;
    let rrr = &rr;
    assert_eq!(rrr.x, 12);
}

#[test]
fn test_multi_ref(){
    let x = 12;
    let r = &x;
    let rr = &r;
    let rrr = &rr;
    assert_eq!(***rrr, 12);
    println!("{}", rrr);
}