fn main() {
    let my_vector = vec![1, 2, 3];

    for element in my_vector {
        println!("I like {}.", element);
    }

    // The range expression 1..3 will yield elements 1 and 2
    for element in 1..3 {
        println!("I like {}.", element);
    }

    // The inclusive range expression 1..=3 will yield elements 1, 2 and 3
    for element in 1..=3 {
        println!("I like {}.", element);
    }

    let mut counter = 3;

    for _ in 1..=5 {
        counter = counter - 1;

        if counter < 0 {
            // Use break, to exit a loop and stop iterating
            break;
        }

        println!("{}", counter);
    }
}
