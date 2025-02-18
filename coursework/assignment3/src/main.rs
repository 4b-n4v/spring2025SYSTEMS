/*
Create a simple number guessing game in Rust. The program should:

    Use a mutable variable to store a "secret" number (you can hard-code this).
    Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
        0 if the guess is correct
        1 if the guess is too high
        -1 if the guess is too low
    In the main function:
        Use a loop to repeatedly:
            Set a mutable guess variable to a number of your choice (simulating user input)
            Call the check_guess function
            Use an if-else expression to print whether the guess was correct, too high, or too low
            If the guess was correct, break the loop
        After the loop ends, print how many guesses it took (you'll need to track this in a variable)

These assignments strictly use only the concepts covered in the provided materials: variables, mutability, basic data types (integers, booleans), arrays, functions, if-else expressions, and the three types of loops (loop, while, for).
*/
// Step 2:
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        return -1; // Too low
    } else if guess > secret {
        return 1; // Too high
    } else {
        return 0; // Correct
    }
}
fn main() {
    // Step 1:
    let mut secret_num: i32 = 1001;
    let mut num_guess: i32 = 0;
    loop {
        let mut guess: i32 = 1001; // Simulate user input right, not actually use std::io?
        let ans = check_guess(guess, secret_num);
        if ans == 1 {
            println!("The guess, {guess}, was too high");
        } else if ans == -1 {
            println!("The guess, {guess}, was too low");
        } else {
            println!("The guess, {guess}, was correct");
            println!("The number of guesses it took you to get the write answer was {num_guess}.");
            break;
        }
        num_guess += 1;
    }
}
