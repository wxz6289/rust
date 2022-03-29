pub fn function(x: i32) -> i32{
    println!("Hello Snake case {}", x);
    let y = {
        x + 1
    }; 
    y
   
   // x
}

pub fn return_number() -> i32 {
    32
}

pub fn run() {
    let n = return_number();
   println!("{:?}", n)
}