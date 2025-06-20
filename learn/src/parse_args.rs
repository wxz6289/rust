use std::env;
use std::str::FromStr;
use crate::gcd::*;

pub fn parse_arg(){
  let mut numbers: Vec<u64> = Vec::new();
  for arg in env::args().skip(1) {
    match u64::from_str(&arg) {
      Ok(num) => numbers.push(num),
      Err(_) => {
        eprintln!("Error: '{}' is not a valid positive integer.", arg);
        return;
      }
    }
  }
  if numbers.len() < 2 {
    eprintln!("Error: Please provide at least two positive integers.");
    return;
  }

  let mut d = numbers[0];
  for &num in &numbers[1..] {
    d = gcd(d, num);
  }
  println!("GCD of the provided numbers is: {}", d);
}