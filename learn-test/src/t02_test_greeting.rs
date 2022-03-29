pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("King");
        assert!(result.contains("King"), "Greeting did not contain name, value was ` {} `", result);
    }
}