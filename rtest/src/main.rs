/* mod t01_test_add_two;
mod t02_test_greeting;
mod t03_test_should_panic;
mod t04_test_result; */

struct ImportantExcept<'a> {
    part: &'a str
}

fn main() {
    fn longest<'a>(x: &'a str, _y: &'a str) -> &'a str {
        x
       /*  let result = String::from("test");
        result.as_str() */
    }

    println!(" {} ", longest("abc", "cds"));

    let novel = String::from("Call me Ishmael, Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Coulf not find a '.'");
    let i = ImportantExcept {
        part: first_sentence
    };

    println!("{} ", i.part);
}
