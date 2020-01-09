pub fn run() {
    // str - immutable
    let hello = "Hello";
    let mut hello2 = String::from("Hello ");

    
    
    
    // Get length
    println!("Length: {}", hello.len());
    
    hello2.push('W');
    
    hello2.push_str("orld!");
    
    println!("{}", hello2.capacity());

    println!("Is Empty: {}", hello2.is_empty() );

    println!("Containes 'World' {}", hello2.contains("World"));

    println!("Replace: {}", hello2.replace("World", "There"));

    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    println!("{:?}",(hello,hello2));

    let mut s = String::with_capacity(10);
    s.push('1');
    s.push('2');

    // assertion testing - nothing happens, it passed
    assert_eq!(2, s.len());
    // gives error
    // assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());


}