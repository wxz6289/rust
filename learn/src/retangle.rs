/* pub fn show_tangle(){
  let width = 20;
  let height = 32;
  println!("this tangle area is {}", area(width, height));
}

fn area(width: usize, height: usize) -> usize {
  width * height
} */

/* pub fn show_tangle() {
  let tangle = (20, 32);
   println!("this tangle area is {}", area(tangle));
}

fn area(tangle: (usize, usize)) -> usize {
  tangle.0 * tangle.1
} */

/* #[derive(Debug)]
struct Tangle {
    width: usize,
    height: usize,
}

pub fn show_tangle() {
    let tangle = Tangle {
        height: 20,
        width: 32,
    };
    let x = 3.1415;
    println!("{}", x);
    println!("this tangle  is {:?}", tangle);
    println!("this tangle  is {:#?}", tangle);
    println!("this tangle area is {}", area(&tangle));
}

fn area(tangle: &Tangle) -> usize {
    tangle.width * tangle.height
}
*/

pub fn show_tangle() {
    let tangle = Tangle {
        height: 20,
        width: 32,
    };
    let x = 3.1415;
    println!("{}", x);
    println!("this tangle  is {:?}", tangle);
    println!("this tangle  is {:#?}", tangle);
    println!("this tangle area is {}", tangle.area());
}

#[derive(Debug)]
struct Tangle {
    width: usize,
    height: usize,
}

impl Tangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn can_hold(&self, other: &Tangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}

impl Tangle {
    fn square(size: usize) -> Tangle {
        Tangle {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tangle() {
        let tangle = Tangle {
            height: 20,
            width: 32,
        };
        println!("{}", tangle.area());
        assert_eq!(tangle.area(), 20 * 32)
    }
    #[test]
    fn test_can_hold() {
        let t1 = Tangle {
            width: 32,
            height: 32,
        };
        let t2 = Tangle {
            width: 20,
            height: 12,
        };
        let t3 = Tangle {
            width: 20,
            height: 42,
        };
        println!("{}", t1.can_hold(&t2));
        assert!(t1.can_hold(&t2));
        assert!(!t1.can_hold(&t3));
    }
    #[test]
    fn test_square() {
        let t1 = Tangle::square(36);
        println!("{}", t1.area());
    }
}

/* mod tests {
  use super::*;

  #[test]
  fn test_simple() {
      assert_eq!(reverse("racecar"), "racecar");
  }
} */
