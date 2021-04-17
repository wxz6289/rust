pub fn run() {
   greet("Hello", "King");
   let get_sum = add(23, 32);
   println!("{}", get_sum);
   let add_sum = | n1: i32, n2: i32 | n1 + n2;
   println!("{}", add_sum(1, 2));
}

fn greet (greet: &str, name: &str) {
    println!("{} {}, Nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}