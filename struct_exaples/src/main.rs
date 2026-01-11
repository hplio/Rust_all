#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
    //This is associate function in which there is no self perameter
    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let ret = Rectangle {
        width: 10,
        height: 60,
    };

    let ret1 = Rectangle {
        width: 20,
        height: 20,
    };

    println!("The value of ret is : {ret:#?}");

    println!("The are of rect is: {} ", ret.area());

    println!(
        "can fit secound rectangle in first? :{}",
        ret.can_hold(&ret1)
    );

    let sq = Rectangle::square(32); //-----> this is how we call associate function using :: sing
    println!("the square is :  {sq:#?}")
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rec = Rectangle {
//         width: 30,
//         height: 60,
//     };
//     println!("Rectangle is: {rec:#?}");
//     println!("The area of the rectangle is: {}", area(&rec));
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     return rectangle.width * rectangle.height;
// }

// struc Use`` {
//     name: String,
//     email: String,
//     age: u32,
//     is_active: bool,
// }

// fn main() {

//     let user_data= User{
//         name: String::from("Alice"),
//         email: String::from("alice@gmail.com"),
//         age: 24,
//         is_active:true
//     };

//     println!("User Name: {}", user_data.name);
//     println!("User Email: {}", user_data.email);
//     println!("User Age: {}", user_data.age);
//     println!("Is User Active?: {}", user_data.is_active);

// }
