// use core::num;
// //QUESTION:- function to print even/odd number
// use std::io::{self, Read};
// fn main() {
//     println!("Enter the number!");
//     let mut number = String::new();
//     io::stdin()
//         .read_line(&mut number)
//         .expect("Failed to read the line.");
//     let num: i32 = number.trim().parse().expect("Not a valid number");
//     let ans = is_even(num);
//     println!("{}", ans);
// }

// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// !QUESTION:- fibbonacci number
use std::io::{self, Read};
fn main() {
    println!("Enter number!");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let num: i32 = input.trim().parse().expect("Not a valid input");

    let ans: i32 = func(num);
    println!("Fibonacci of number {} is {}", num, ans);
}
fn func(num: i32) -> i32 {
    if num <= 1 {
        return num;
    }
    return func(num - 1) + func(num - 2);
}
// fn func(num: i32) -> i32 {
//     let mut first: i32 = 0;
//     let mut second: i32 = 1;
//     if num <= 1 {
//         return num;
//     }
//     for _ in 1..num {
//         let temp: i32 = second;
//         second = second + first;
//         first = temp;
//     }
//     return second;
// }
