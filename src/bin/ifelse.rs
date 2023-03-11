fn main() {
    let price = 40.00;
    let my_balance = 80.00;
    let feeling_thrifty = true;

    // if example
    if feeling_thrifty == true {
        println!("I will not purchase today");
    }

    // if else example
    if price <= my_balance {
        // comparison operator
        println!("I can afford this purchase");
    } else {
        println!("I cannot afford this purchase");
    }

    //if else if example
    if feeling_thrifty == true {
        println!("I will not purchase today");
    } else if price <= my_balance {
        println!("I can afford this purchase");
    } else {
        println!("I cannot afford this purchase");
    }
}
