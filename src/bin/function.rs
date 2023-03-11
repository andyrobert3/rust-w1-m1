fn say_something() {   // this function does not return anything!
    println!("Hello");
 }
 fn calc_amount(number: f32, rate: f32) -> f32 {  // this function returns an float value
    number * rate             
 }
 fn main() {
    say_something();    // prints "Hello"  
    let amount1 = calc_amount(2.0, 5.5);       // 'amount1' stores the return value from calc_amount() 
    let amount2 = calc_amount(amount1, 2.0);   // 'amount2' stores the return value
    println!("{}", amount2);  // Can you guess what value this prints?
 }
 