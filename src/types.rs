pub fn run() {
    let x = 1;
    let y = 2.5;

    let z:i64 = 444234234234;

    // find max size

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;

    println!("{:?}",(x,y,z,is_active));

    let is_greater = 10 > 5;

    println!("{}", is_greater);

    let a1 = 'a';

    println!("{}", a1);

    let face = '\u{1F600}';

    println!("{}", face);
}