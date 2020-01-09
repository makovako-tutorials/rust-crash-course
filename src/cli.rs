use std::env;

pub fn run() {

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Lala";
    let status = "100%";
    // println!("{:?}", command);
    if command == "hello" {
        println!("Hi {}", name);
    } else if command == "status" {
        println!("Status {}", status);
    } else {
        println!("That is not a valid command", )
    }
}