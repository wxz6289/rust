#[test]
fn print_padova(){
  let mut padova = vec![1, 1, 1];
  for i in  3..10 {
    let next = padova[i-3] + padova[i-2];
    padova.push(next);
  }
  println!("{:?}", padova);
}
#[test]
fn print_struct(){
  let point = Box::new((0.625, 0.5));
  let label = format!("{:?}", point);
  assert_eq!(label, "(0.625, 0.5)");
}

struct Person {
  name: String,
  birth: i32
}
#[test]
fn test_person(){
  let mut composers =  Vec::new();
  composers.push(Person { name: "King".to_string(), birth: 1990 });
  composers.push(Person { name: "Dreamer".to_string(), birth: 2026 });
  composers.push(Person { name: "Panpan".to_string(), birth: 2000 });
  for composer in composers {
    println!("{} -> {}", composer.name, composer.birth);
  }
}

#[test]
fn transform(){
  let s = vec!["udon".to_string(), "King".to_string(), "wxz".to_string()];
  let t = s;
  // let u = s;
  println!("{:?}",t);
}

#[test]
fn transform1(){
  let s = vec!["udon".to_string(), "King".to_string(), "wxz".to_string()];
  let t = s.clone();
  let u = s.clone();
  println!("{:?} {:?} {:?}",t, s, u);
}
