pub fn run() {
   if_run();
   if_else_run();
   let_if();
}

fn if_run() {
     let number = 7;
    if number > 3 {
        println!("greater 3");
    } else {
        println!("less or equal 3");
    }
}

fn if_else_run() {
    let number = 12;
    // if else 会且仅会执行匹配的第一个条件中的语句，不会再检查剩余的其他语句
    if number % 4 == 0 {
        println!("{} / 4 = 0 ", number);
    } else if number % 3 == 0 {
        println!("{} / 3 = 0 ", number);
    } else {
        println!("{}", number)
    }
}

fn let_if() {
    let condition = true;
    let number = if condition {
        3
    } else { 
        2
    };
    println!("The number is {}", number);
}