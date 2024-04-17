#[derive(Debug)]
enum UsState {
  Alaska,
  NewYork
}
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn show_coin_value(coin: Coin) -> usize {
    match coin {
      Coin::Penny => 1,
      Coin::Dime => 10,
      Coin::Nickel => 5,
      Coin::Quarter(state) => {
        println!("{:?}", state);
        25
      },
    }
  }

  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(x) => Some(x +1)
    }
  }

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_coin() {
    let q = Coin::Quarter(UsState::Alaska);
    let v = show_coin_value(q);
    println!("{:?}", v);
  }
  
  #[test]
  fn test_option() {
    let s = Some(5);
    let v = plus_one(s);
    println!("{:?}", v);
  }
}