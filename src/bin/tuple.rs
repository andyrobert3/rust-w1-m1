fn user_balance() -> (String, f32) {
    ("John".to_owned(), 99.99) // return multiple values
}
fn main() {
    let test_tuple = (24, 120);
    println!("{}, {}", test_tuple.0, test_tuple.1); // prints 24, 120

    let user_bal = user_balance();
    println!("{}, {}", user_bal.0, user_bal.1); // prints John, 99.99

    let (x, y) = user_balance(); // destructuring
    println!("{}, {}", x, y); // x = John, y = 99.99 -> prints John, 99.99
}
