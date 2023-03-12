struct User {
    // TODO: create 1 field called name (String)
    // TODO: create 1 field called balance (Tuple) that contains the balance (f32) and currency (String)
    name: String,
    balance: (f32, String),
}

impl User {
    // TODO: Create 1 method for User struct called print_name, that prints the name
    fn print_ln(&self) {
        println!("{}", self.name);
    }
}

fn main() {
    // TODO: create a variable user of type User and provide values into its fields.
    // TODO: call print_name on variable user. Then, print the individual values within the user.balance tuple.

    let user = User {
        name: "Matthew".to_owned(),
        balance: (100.50, "USDC".to_owned())
    };

    user.print_ln();
    let (amount, currency) = user.balance;
    println!("User '{}' has {} {}", user.name, currency, amount);
}
