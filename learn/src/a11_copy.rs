#[derive(Copy, Clone)]
struct Label {
  number: i32
}

fn print_label(l: Label){
  println!("{}", l.number)
}

#[test]
fn test() {
  let l = Label {
    number: 20
  };
  print_label(l);
  println!("{}", l.number)
}