// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is block-scoped language

pub fn run() {
    let name = "Marek";
    let mut age = 37;
    
    //error
    // age = 38 - cannot assign twice to imutable variable
    println!("My name is {} and i am {}", name, age);
    age = 38;

    println!("My name is {} and i am {}", name, age);

    //define constant

    const ID:i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars

    let (my_name, my_age) = ("Brad",37);
    println!("{} is {}", my_name, my_age);
}