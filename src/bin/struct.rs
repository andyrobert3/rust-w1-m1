struct UserRecord {
    name: String,
    balance: f32,
    active: bool,
 }
 
 fn main() {
    let user = UserRecord {           // declare a var of type Employee, set its field values
        name: "John".to_owned(),     // to_owned() will be explained in Chapter "Strings"
        balance: 25.00,
        active: true,
    };
    let bal = user.balance;
    println!("{} has a balance of {} dollars", user.name, bal); // guess what this prints?
 }
 