use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    println!("Gusess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secrect number is {}", secret_number);
    loop {
        let mut number = String::new();
        println!("Please input your number");
        io::stdin().read_line(&mut number)
            .expect("Faild to read line");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
           // .expect("Please type a number");
        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too larger"),
            Ordering::Equal => { break println!("You win!")}
        }
        // println!("You guessed {}", number);
    }
   
    
}