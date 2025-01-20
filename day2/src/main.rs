// // MEMORY MANAGEMENT AND MUTABILITY
// fn main() {
//     let mut str: String = String::from("Hi there");
//     // the str is stored in stack, however the "hi there" is stored in heap as it is dynamic
//     str.insert(6, 'c');
//     // all variables are immutable by default in rust;
//     println!("{}", str);
// }

// !OWNERSHIP
// every value in rust has an owner
// there can be only one owner at a time
// if the owner goes out of scope, the value gets dropped

// fn main() {
//     let s1: String = String::from("This is the string");
//     // ?THE OWNER OF STRING IS s1 here
//     // let s2: String = s1;
//     let s2: String = s1.clone();
//?In clone a new memory block on the heap is allocated with the contents of original string copied to this memory
//     //?HERE OWNER OF STRING IS s2 now, so if i try to access s2 here, i will be able to access, however if i try to access s1, i won't be able to acces;
//     println!("{}", s2);
//     println!("{}", s1);
// }

// fn main() {
//     let x: i32 = 5;
//     let y: i32 = x;

//     println!("x = {x}, y = {y}");
// }

// fn main() {
//     let mut s: String = String::from("Hi");
//     s = take_ownership(s);
//     // ? if i try to access s here, i wont be able to get because the string has been moved to str means the ownernhip of value hi is moved to str;
//     println!("{s}");
// }
// fn take_ownership(str: String) -> String {
//     println!("{str}");
//     return str;
// }

// fn main() {
//     let str: String = String::from("HI there");
//     let len: usize = get_length(&str);
//     println!("The length of the string {str} is {len}");
// }
// fn get_length(s: &String) -> usize {
//     return s.len();
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{s}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//     }
//     let r2 = &mut s;
//     // println!("{}, {}", s, r2);
// }

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn main() {
//     let mut a: Vec<i32> = Vec::new();
//     a.push(1);
//     a.push(2);
//     a.push(3);
//     println!("{:?}", a);
//     even(&mut a);
//     println!("Modified vector (only evens): {:?}", a);
// }

// fn even(a: &mut Vec<i32>) {
//     let mut i = 0;
//     while i < a.len() {
//         if a[i] % 2 != 0 {
//             a.remove(i);
//         } else {
//             i += 1;
//         }
//     }
// }

use std::{collections::HashMap, string};

// fn main() {
//     let mut users: HashMap<String, i32> = HashMap::new();
//     users.insert(String::from("user_1"), 21);
//     users.insert(String::from("user_2"), 43);

//     let first_user: Option<&i32> = users.get("user_1");

//     match first_user {
//         Some(age) => println!("{age}"),
//         None => println!("None"),
//     }
// }

fn main() {
    let mut users: Vec<(String, i32)> = vec![
        (String::from("user_1"), 21),
        (String::from("user_2"), 25),
        (String::from("user_3"), 30),
        (String::from("user_4"), 18),
        (String::from("user_5"), 22),
        (String::from("user_6"), 27),
    ];

    let users_details: HashMap<String, i32> = func(users);
    for (user, age) in users_details {
        println!("User: {user}, age: {age}");
    }
}

fn func(users: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut users_details: HashMap<String, i32> = HashMap::new();
    for (user, age) in users {
        users_details.insert(user, age);
    }
    return users_details;
}
