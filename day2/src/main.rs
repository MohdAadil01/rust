fn main() {
    let mut str: String = String::from("Hi there");
    // the str is stored in stack, however the "hi there" is stored in heap as it is dynamic
    str.insert(6, 'c');
    println!("{}", str);
}
