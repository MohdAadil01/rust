use core::num;
//QUESTION:- function to print even/odd number
use std::io::{self, Read};
fn main() {
    println!("Enter the number!");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the line.");
    let num: i32 = number.trim().parse().expect("Not a valid number");
    let ans = is_even(num);
    println!("{}", ans);
}
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
