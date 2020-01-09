pub fn run(){
    let age = 18;
    let check_id = false;
    let knows_person_of_age = true;
    if age >= 21 && check_id || knows_person_of_age{
        println!("Drinking allowed", );
    } else if age < 21 && check_id{
        println!("Drinking not allowed", );
    } else {
        println!("Give me id", )
    }

    //shorthand

    let is_of_age = if age >= 21 {true} else {false};
    println!("{}", is_of_age)
}