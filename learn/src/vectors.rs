struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    /* let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("{:?}", numbers);
    for x in numbers.iter() {
        println!("{:?}", x);
    }

    for x in numbers.iter_mut() {
        *x * = 2;
    }
    println!("{:?}", numbers); */
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    p.set_first_name("Wang");
    println!("Person Tuple {:?}", p.to_tuple());
}