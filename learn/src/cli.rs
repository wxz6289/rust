pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "Brand";
    let status = "100%";
    if command == "Hello" {
        println!(" Hi {}, How are you? ", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else { 
        println!("This is not a valid command");
    }
    println!("Args: {:?}", args);
    println!("Command: {}", command)
}