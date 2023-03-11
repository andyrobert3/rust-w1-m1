struct User {
    name: String,
    balance: f32,
    active: bool,
}

impl User {
    fn shout(phrase: String) {
        println!("{}!", phrase);
    }
    fn print_name(&self) {
        println!("{}", self.name);
    }
    fn print_balance(&self) {
        println!("{}", self.balance);
    }
}
fn main() {
    // Calling the associated function, does not require an instance
    User::shout("Hello World".to_owned());
    // declare a var of type User, set its field values
    let user = User {
        // to_owned() will be explained in Chapter "Strings"
        name: "John".to_owned(),
        balance: 25.00,
        active: true,
    };
    user.print_name(); // Method: access instance’s name
    user.print_balance(); // Method: access instance’s balance
    User::print_balance(&user); // Method: Calling as an associated function
}
