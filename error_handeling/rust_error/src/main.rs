use rand::Rng;
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, Read},
};
// use std::{fs::File, io::ErrorKind};

fn main() {
    // panic!("Crash and burn");
    // let v = vec![1,2,3,4];
    // v[20];

    //NOTE: RESULT EXAMPLE

    // let greeting_file_example = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_example{
    //     Ok(file)=> file,
    //     Err(err)=> match err.kind() {
    //         ErrorKind::NotFound=> match File::create("hello.txt") {
    //             Ok(file)=>file,
    //             Err(err)=> panic!("error occure while creating file: {err:?}"),
    //         },
    //         _=>{
    //             panic!("Problem while opening file");
    //         }
    //     },
    //     // Err(err)=> panic!("Problem opening file : {err:?}"),

    //NOTE: EXCEPT METHOD

    let value = Some(32);
    let ex = value.expect("Invalid value");
    println!("The value is: {ex}");
    // let file = File::open("dr.txt").expect("not found file");

    //NOTE: ? USE

    let a = match read_username_from_file() {
        Ok(value) => println!("value: {value}"),
        Err(err) => println!("error: {err}"),
    }; // let test_file = File::open("hp.txt")?;
       // println!("{a:?}");

    // let last_chr = match last_char_of_first_line("hello from earth") {
    //     Some(value) => println!("the value is : {value}"),
    //     None => println!("No value found."),
    // };

    // println!("the last chr: {last_chr:?}");
    // INFO: GUESS GAME:

    println!("Guess the number game:-------------");
    let system_number = rand::thread_rng().gen_range(1..=100);
    let mut user_try = 1;

    loop {
        println!("Enter your input between 1 to 100:  ");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Please enter valid format");

        let guess: u32 = match user_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                user_try += 1;
                continue;
            }
        };
        if guess < 1 || guess > 100 {
            println!("Please enter between 1 to 100");
            user_try += 1;
            continue;
        }
        match guess.cmp(&system_number) {
            Ordering::Less => {
                println!("small number!!");
                user_try += 1;
            }
            Ordering::Greater => {
                println!("greater number!!");
                user_try += 1;
            }
            Ordering::Equal => {
                println!("YOU HAVE WON!! in {user_try} try.");
                break;
            }
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    // println!("user name is : {username}");
    // Ok("".to_string())
}
//
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }
