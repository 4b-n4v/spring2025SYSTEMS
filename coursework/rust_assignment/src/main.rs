// fn main() {
//     // let x = true;
//
//     let x = 5;
//     let rem = x % 3;
//
//     let phrase = match rem {
//         0 => "The remainder is Zero",
//         1 => "The remainder is One",
//         2 => {
//             println!("This was amazing choice!");
//
//             // "The remainder is Two"
//         }
//         _ => "#",
//     };
//
//     println!("{}", phrase);
// }
//
//
//
//
//
// fn main() {
//     let num = 5;
//
//     if num % 3 == 0 && num % 5 == 0 {
//         println!("FizzBuzz");
//     } else if num % 3 == 0 {
//         println!("Buzz");
//     } else if num % 5 == 0 {
//         println!("Fizz");
//     } else {
//         println!("{}", num);
//     }
// }

// fn main() {
//     let number = 5;
//
//     // let divisible_by_two = if number % 2 == 0 {
//     //     "even" // no semicolon, because then it becomes an expression
//     // } else {
//     //     "odd" // both arms or branches should evaluate to the same type
//     // };
//     let mut divisible_by_two = "odd";
//     if number % 2 == 0 {
//         divisible_by_two = "even";
//     }
//
//     println!("Number {} is a type of {}", number, divisible_by_two);
// }

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];

    // for each loop
    for idx in 0..10 {
        nums.push(idx);
    }

    println!("{:?}", nums);
    // println!("{}", nums);
}
