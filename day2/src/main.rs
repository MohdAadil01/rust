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

fn main() {
    let s1: String = String::from("This is the string");
    // ?THE OWNER OF STRING IS s1 here
    let s2: String = s1;
    //?HERE OWNER OF STRING IS s2 now, so if i try to access s2 here, i will be able to access, however if i try to access s1, i won't be able to acces;
    println!("{}", s2);
    println!("{}", s1);
}
