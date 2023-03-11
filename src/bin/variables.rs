fn main() {
    let is_member: bool = false; // data type explicitly given
    let is_member = false; // data type automatically inferred
    let user_age: i32 = 33; // data type explicitly given
    let user_age = 33; // data type automatically inferred
    let single_letter = 'a';
    let amount = 37.65;

    let my_age = user_age; // assignment of value of one variable to another
    let mut vesting_days = 10; // mutable variable, its value can be changed later
    vesting_days = 9; // give vesting_days a new value
    let message = "Hello Customer"; // this is a string slice, covered later
    message = "Hi Customer"; // error: cannot assign twice to immutable variable
}
