use num::complex::Complex;

#[test]
pub fn test_complex(){
  let a  = Complex {
    re: 23,
    im: 3
  };
  let b = Complex::new(11, 12);
  let c =  a + b;
  println!("{}", c);
}

