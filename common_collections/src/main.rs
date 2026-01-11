// enum Hp {
//     Int(u32,i32,String),
//     Text(String),
// }

use std::collections::HashMap;

struct Hp {
    address: String,
    ip: i32,
}

fn main() {
    let mut a: Vec<u32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // println!("the one value is: {}",v[0]);
    // println!("Hello, world!");
    // let check = &a[2];
    a.push(10);
    a.push(9);
    a.push(8);
    a.push(7);

    // println!("value of check is : {}",check);

    let third: &u32 = &a[3];
    println!("the fourth item is : {}", third);

    let thd = v.get(2);
    match thd {
        Some(thd) => println!("the fourth item of secound is: {}", thd),
        None => println!("Not available"),
    };

    for i in &v {
        println!("{i}");
    }

    let mut test = vec![100, 200, 350];

    for i in &mut test {
        *i += 50;
    }

    for j in &test {
        println!("value: {}", j);
    }

    // let etest = Hp::Int(12, 8, String::from("hello"));

    // println!("first value is: {}",etest);

    let a = vec![
        Hp {
            address: String::from("India"),
            ip: 124,
        },
        Hp {
            address: String::from("USA"),
            ip: 125,
        },
        Hp {
            address: String::from("China"),
            ip: 126,
        },
    ];

    match a.get(0) {
        Some(hello) => {
            println!("value of 1 is: {}", hello.address);
            println!("value of 1 is: {}", hello.ip);
        }
        None => println!("Not available"),
    }
    match a.get(1) {
        Some(hello) => {
            println!("value of 2 is: {}", hello.address);
            println!("value of 2 is: {}", hello.ip);
        }
        None => println!("Not available"),
    }
    match a.get(2) {
        Some(hello) => {
            println!("value of 3 is: {}", hello.address);
            println!("value of 3 is: {}", hello.ip);
        }
        None => println!("Not available"),
    }
    // let ve: Vec<i32> = Vec::new();
    let mut hello = String::from("你好");
    hello.push_str(" hello");
    println!("value of string is : {}", &hello);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    // let b = format!("{s1}-{s2}-{s3}");
    println!("value of s is : {s}");
    println!("value of s is : {s}");
    // println!("value of s is : {b}");
    let hello_in_russian = String::from("Здравствуйте");
    let length_of_r = hello_in_russian.len();
    println!("the length of word is: {}", length_of_r);
    let first_chr = &hello_in_russian.as_bytes()[0];
    println!("{first_chr}");

    //NOTE: HASHMAP

    let mut score = HashMap::new();

    score.insert(String::from("Yellow"), 12);
    score.insert(String::from("Purple"), 50);
    let selected_team = String::from("Purple");
    println!(
        "the score of yellow team is: {}",
        score.get(&selected_team).copied().unwrap_or(0)
    );

    for (key, value) in &score {
        println!("{key} : {value}");
    }

    score.insert(String::from("Yellow"), 20);
    println!("{:?}", score);
    score.entry(String::from("Black")).or_insert(100);
    score.entry(String::from("Yellow")).or_insert(100);

    println!("{:?}", score);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        println!("word is: {word}");
        println!("count before is: {count}");
        *count += 1;
        println!("count after is: {count}");
    }

    println!("{:?}", map);

  
}

// #![allow(unused)]
// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//
//     assert_eq!(vec.len(), 2);
//     assert_eq!(vec[0], 1);
//
//     assert_eq!(vec.pop(), Some(2));
//     assert_eq!(vec.len(), 1);
//
//     vec[0] = 7;
//     assert_eq!(vec[0], 7);
//
//     vec.extend([1, 2, 3]);
//
//     for x in &vec {
//         println!("{x}");
//     }
//     assert_eq!(vec, [7, 1, 2, 3]);
// }
