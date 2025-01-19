// // // // // use core::num;
// // // // // //QUESTION:- function to print even/odd number
// // // // // use std::io::{self, Read};
// // // // // fn main() {
// // // // //     println!("Enter the number!");
// // // // //     let mut number = String::new();
// // // // //     io::stdin()
// // // // //         .read_line(&mut number)
// // // // //         .expect("Failed to read the line.");
// // // // //     let num: i32 = number.trim().parse().expect("Not a valid number");
// // // // //     let ans = is_even(num);
// // // // //     println!("{}", ans);
// // // // // }

// // // // // fn is_even(num: i32) -> bool {
// // // // //     if num % 2 == 0 {
// // // // //         return true;
// // // // //     }
// // // // //     return false;
// // // // // }

// // // // // !QUESTION:- fibbonacci number
// // // // use std::io::{self, Read};
// // // // fn main() {
// // // //     println!("Enter number!");
// // // //     let mut input: String = String::new();
// // // //     io::stdin()
// // // //         .read_line(&mut input)
// // // //         .expect("Failed to read the line");

// // // //     let num: i32 = input.trim().parse().expect("Not a valid input");

// // // //     let ans: i32 = func(num);
// // // //     println!("Fibonacci of number {} is {}", num, ans);
// // // // }
// // // // fn func(num: i32) -> i32 {
// // // //     if num <= 1 {
// // // //         return num;
// // // //     }
// // // //     return func(num - 1) + func(num - 2);
// // // // }
// // // // fn func(num: i32) -> i32 {
// // // //     let mut first: i32 = 0;
// // // //     let mut second: i32 = 1;
// // // //     if num <= 1 {
// // // //         return num;
// // // //     }
// // // //     for _ in 1..num {
// // // //         let temp: i32 = second;
// // // //         second = second + first;
// // // //         first = temp;
// // // //     }
// // // //     return second;
// // // // }

// // // use std::io;

// // // // !QUESTION 3:- function to return string length
// // // fn main() {
// // //     println!("Enter the string. ");
// // //     let mut str: String = String::new();
// // //     io::stdin()
// // //         .read_line(&mut str)
// // //         .expect("Unable to read input from the line.");
// // //     let ans: usize = get_length(str);
// // //     println!("The size of the input string is {}", ans);
// // // }
// // // fn get_length(str: String) -> usize {
// // //     return str.trim().chars().count();
// // // }

// // //  STRUCT:- lets you create user defined data type, means you can structure data together

// // use std::{fs::read_link, io};

// // struct User {
// //     username: String,
// //     password: String,
// //     active: bool,
// // }
// // fn main() {
// //     let mut username: String = String::new();
// //     let mut password: String = String::new();
// //     let mut active: String = String::new();
// //     println!("Enter username");
// //     io::stdin()
// //         .read_line(&mut username)
// //         .expect(("Unable to read input"));
// //     println!("Enter password");
// //     io::stdin()
// //         .read_line(&mut password)
// //         .expect("Unable to read passsword input");
// //     println!("Active/Inactive");
// //     io::stdin()
// //         .read_line(&mut active)
// //         .expect("Unable to read input");

// //     username = username.trim().to_string();
// //     password = password.trim().to_string();

// //     let is_active: bool = match active.trim().to_lowercase().as_str() {
// //         "active" => true,
// //         "inactive" => false,
// //         _ => {
// //             eprintln!("Invalid input for active/inactive. Defaulting to inactive.");
// //             false
// //         }
// //     };
// //     let user1: User = User {
// //         username: username,
// //         password: password,
// //         active: is_active,
// //     };
// //     println!("{}", user1.username);
// // }

// // !OPTIONS;

// use std::io;

// fn main() {
//     let mut s: String = String::new();
//     println!("Enter string");
//     io::stdin()
//         .read_line(&mut s)
//         .expect("Unable to read input from the string.");
//     match find_first_a(s) {
//         Some(index) => println!("{} is the index of a", index),
//         None => println!("Not found a!"),
//     }
// }

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// !RESULT
use std::fs;
fn main() {
    let greet_file = fs::read_to_string("a.txt");

    match greet_file {
        Ok(data) => println!("{}", data),
        Err(error) => println!("Failed to read file {}", error),
    }
}
