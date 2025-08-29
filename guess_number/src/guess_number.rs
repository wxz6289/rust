use rand::seq::SliceRandom;
use rand::{ random_range, random, rng };
use std::cmp::Ordering::{ Less, Greater, Equal };
use std::io::stdin;

pub fn guess_number() {
    println!("Guess the number");
    let secret_number = random_range(1..101);
    // println!("The secret number is {}", secret_number);
    loop {
        let mut number: String = String::new();
        println!("Please input your number");
        stdin().read_line(&mut number).expect("Failed to read line");
        println!("You guessed: {}", &number);
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        // .expect("Please type a number");
        match number.cmp(&secret_number) {
            Less => println!("Too small"),
            Greater => println!("Too larger"),
            Equal => {
                break println!("You win!");
            }
        }
        // println!("You guessed {}", number);
    }
}

pub fn learn_rand() {
    if random() {
        println!("char: {}", random::<char>());
    }

    let mut rng = rng();
    let y: f64 = random();

    let mut nums: Vec<i32> = (1..10).collect();
    nums.shuffle(&mut rng);
    println!("{} {:?}", y, nums)
}
