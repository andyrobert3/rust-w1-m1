fn reduce_ten_times (mut balance: i32) {
    //TODO: Create a for loop that runs 10 times, Each iteration should:      
        // TODO: Reduce the balance by 1.0
        // TODO: Use an If Else to
        //       Print the balance to terminal if balance is less or equal to 3,
        //       Print "Balance still above 3" if the balance is more than 3
        //       Break out of the loop if balance is less than 0       
    for _ in 1..10 {
        balance -= 1;    

        if balance < 0 {
            break;
        } else if balance > 3 {
            println!("Balance still above 3");
        } else {
            println!("{}", balance);
        }
    }
    
 }
 
 fn main() {
    // TODO: Create a variable representing the users starting balance, give it any value
    // TODO: Call the function you created and pass in the users starting balance   

    let mut start_balance: i32 = 8;
    reduce_ten_times(start_balance);
 }
 
 