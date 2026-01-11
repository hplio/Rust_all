use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is now: {x}");

    let space = "   ";

    let space = space.len();

    let tup = (500, 100, -100);

    println!("The value of x is: {}", tup.2);

    println!("Space is {space}");

    let a = [1, 5, 10, 20, 40, 80];

    println!("Please enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = input.trim().parse().expect("Please enter a valid number");

    let element = a[index];

    dog_print(index as u8, element);

    divisible_by();

    looping_somthing();

    break_loop_with_lable_example();

    for element in a {
        println!("Element: {}", element);
    }

    for number in 1..8 {
        println!("Number: {}", number);
    }
}

fn dog_print(index: u8, element: u32) {
    println!("The value of the element at index {index} is: {element}");
}

fn divisible_by() {
    let a = 8;
    if a % 5 == 0 {
        println!("{} is divisible by 2", a);
    } else if a % 3 == 0 {
        println!("{} is divisible by 3", a);
    } else if a % 4 == 0 {
        println!("{} is divisible by 4", a);
    } else {
        println!("Not divisible by any");
    }
}

fn looping_somthing() {
    let mut a = 0;

    let result = loop {
        a += 1;
        if a == 10 {
            break a * 2;
        }
    };

    println!("The result is: {}", result);
}

fn break_loop_with_lable_example() {
    let mut c = 0;
    'outer: loop {
        println!("c is: {}", c);
        let mut r = 10;
        println!("r is: {}", r);
        loop {
            if r == 9 {
                break;
            }
            if c == 2 {
                break 'outer;
            }
            r -= 1;
            println!("r is now: {}", r);
        }
        c += 1;
    }

    println!("It's all done!");
}
