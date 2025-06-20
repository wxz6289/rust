#[allow(arithmetic_overflow)]
pub fn test_overflow(){
let (a, b) = (200, 200);
let c:u8 = a + b;
println!("{} + {} = {}", a, b, c);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero() {
    1 / 0;
}

use std::num::ParseIntError;

#[test]
fn explicit_radix() -> Result<(), ParseIntError> {
  i32::from_str_radix("1024", 10)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  fn roughly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
  }

  #[test]
  fn test_works(){
    use std::f64::consts::PI;
    assert!(roughly_equal(PI.sin(), 0.0));
  }
}


