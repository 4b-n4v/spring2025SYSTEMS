/*
Create a Rust program that analyzes a series of numbers. The program should:

1    Create an array of 10 integer numbers of your choice.
2    Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
3    Use a for loop to iterate through the array and for each number:
3.1       Print whether it's even or odd using your is_even function
3.2        If the number is divisible by 3, print "Fizz" instead
3.3      If the number is divisible by 5, print "Buzz" instead
3.4        If it's divisible by both 3 and 5, print "FizzBuzz"
4    Use a while loop to find and print the sum of all numbers in the array.
5    Use a loop to find and print the largest number in the array.
*/

// Step 2:
fn is_even(n: i32) -> bool {
    return (n % 2) == 0;
}
fn main() {
    // Step 1:
    let my_super_cool_arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 123123];

    // Step 3:
    for num in my_super_cool_arr {
        // Step 3.1
        if is_even(num) {
            println!("{} is even!", num)
        } else {
            println!("{} is odd!", num)
        }
        // Step 3.4
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        // Step 3.2
        } else if num % 3 == 0 {
            println!("Fizz");
        // Step 3.3
        } else if num % 5 == 0 {
            println!("Buzz");
        // Didn't specify but I assume we just keep going eh?
        } else {
            continue;
        }
    }
    // Step 4:
    let mut sum = 0;

    let mut index = 0;
    while index < my_super_cool_arr.len() {
        sum += my_super_cool_arr[index];
        index += 1;
    }
    println!("Sum of all these super cool numbers is {sum}");

    // Step 5:
    let mut max_num = my_super_cool_arr[0];
    for &num in my_super_cool_arr.iter() {
        if num > max_num {
            max_num = num;
        }
    }
    println!("Largest number in the array: {}", max_num);
}
