// // // ! LIFETIME
// // fn main() {
// //     // let ans;
// //     // let str1: String = String::from("longest");
// //     // {
// //     //     let str2: String = String::from("smallbiggerekdj");
// //     //     ans = longest(&str1, &str2);
// //     // !this code is problametic coz, ans might point to str2 or str1(borrowed from str1 or str2), but str2 score is limited, so after its scope ans will be pointing to dangling pointer., ans will be a string slice pointing to str2
// //     // }
// //     // println!("{ans}");

// //     let ans;
// //     let str1: String = String::from("longest");
// //     {
// //         let str2: String = String::from("small");
// //         ans = longest(&str1, &str2);
// //     }
// //     println!("{ans}");
// // }

// // // !how is the lifetime of the output is related to the output of the inputs str1 and str2, we have to specify that.
// // // fn longest(a: &str, b: &str) -> &str {
// // //     if a.len() > b.len() {
// // //         return a;
// // //     } else {
// // //         return b;
// // //     }
// // // }

// // !to solve the above problem
// fn main() {
//     let ans;
//     let str1 = String::from("small");
//     {
//         let str2 = String::from("longer");
//         ans = longest(&str1, &str2);
//         // here is ans points to str2, ans lifetime would in the next line, so we have to solve this problem, the intersection of life time of str1 and str2 whichever is smaller will be the life time of ans;
//         println!("{ans}");
//     }
// }
// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         return str1;
//     } else {
//         return str2;
//     }
// }

// // 'a generic lifetime annotation

struct User<'a> {
    username: &'a str,
}
fn main() {
    let user;
    {
        let username = String::from("Dark");
        user = User {
            username: &username,
        };
        println!("{}", user.username);
    }
}
