use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

struct CustomStartPointer {
    data: String,
}

impl Drop for CustomStartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomStartPointer with data {} !", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#box::List::{Cons, Nil};
    use std::rc::Rc;

    #[test]
    fn test_box() {
        let b = Box::new(6);
        println!("{}", *b);
        println!("{}", b);
    }

    #[test]
    fn test_list() {
        let list = Rc::new(Cons(
            1,
            Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil))))))),
        ));
        println!("{:#?}", list);
    }

    #[test]
    fn deref() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_box() {
        let x = 6;
        let y = Box::new(x);
        assert_eq!(6, *y);
        println!("{}", *y);
    }

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, *y);
        println!("{}", *y);
    }

    #[test]
    fn test_hello() {
        let m = MyBox::new(String::from("King"));
        hello(&(*m)[..]);
        hello(&m);
    }

    #[test]
    fn test_drop() {
        let c = CustomStartPointer {
            data: String::from("hello c"),
        };
        drop(c); // 提前丢弃值
        let d = CustomStartPointer {
            data: String::from("hi d"),
        };
        println!("CustomStartPointer created");
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("1 a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
        println!("2 a = {}", Rc::strong_count(&a));
        {
            let d = Cons(6, Rc::clone(&a));
            println!("3 a = {}", Rc::strong_count(&a));
        }
        println!("4 a = {}", Rc::strong_count(&a));

        /*    println!("a: {:#?}", a);
        println!("b: {:#?}", b);
        println!("c: {:#?}", c); */
    }
}
