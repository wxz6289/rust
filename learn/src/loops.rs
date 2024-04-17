pub fn run() {
    /*  let mut count = 0;
    loop {
        count += 1;
        println!("{}", count);
        if(count == 20) {
            break;
        }
    }

    while count <= 100 {
        if count % 15 == 0 {
            println!("{} % 15", count);
        } else if count % 3 == 0 {
            println!("{} % 3", count);
        }

        count += 1;
    } */

    for x in 0..10 {
        println!("{}", x);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iter() {
        let nums = [1, 2, 3, 4, 5];
        for i in nums.iter() {
            println!("{}", i);
        }
        run()
    }
}
