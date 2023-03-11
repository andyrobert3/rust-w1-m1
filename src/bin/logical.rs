fn main() {
    let feeling_thrifty = true;
    let my_balance = 30.00;

    // Logical AND
    if (feeling_thrifty == true) && (my_balance < 50.0) {
        println!("I will not go shopping today");
    }

    let am_thirsty = true;
    let am_hungry = false;

    // Logical OR
    if (am_thirsty == true) || (am_hungry == true) {
        println!("What's in the fridge?");
    }
}
