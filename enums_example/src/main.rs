// #![allow(unused)] //----> with this we don't get warning message such as unused etc..
// enum IpAddr {
//     V4(u8, u8, u8),
//     V6(String),
// }
//
// impl IpAddr {
//     fn call(&self) {
//         println!("method call is used");
//     }
// }
//
//     enum Option<T> {
//         None,
//         Some(T),
//     }
//
// // enum Option<T> {
// //     Some(T),
// //     None,
// // }
// fn main() {
//     let home = IpAddr::V4(127, 0, 0);
//
//     let personal = IpAddr::V6(String::from("::1"));
//
//     home.call();
//
//
//
//     let a = Some(5);
//     let b = Some("hello");
//     let c:Option<i32> =None;
//
//     // println!(a);
//     // println!(b);
//     // println!(c);
// }

//
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
//
// struct IpAddr  {
//    kind : IpAddrKind,
//    address: String,
// }
//
//
// fn main() {
//    let home = IpAddr{
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0"),
//     } ;
//
//     let personal = IpAddr{
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//
//    // println!("the home ip struct is : {home:#?}");
//     // println!("the personal ip struct is : {personal:#?}");
//
//  }

//NOTE: IT IS OPTION EXAMPLE

// #[derive(Debug)]
// enum IpAdd {
//     V1(String),
//     V2(String),
// }
//
// impl IpAdd {
//     fn print_hello(&self) {
//         println!("this is example of enum method. and data is {self:#?}");
//     }
// }
//
// fn main() {
//     let home = IpAdd::V1(String::from("127.0.0.1"));
//     let loopback = IpAdd::V2(String::from("::1"));
//
//     let m: Option<i32> = Option::Some(12);
//     let n: Option<i32> = Option::None;
//
//     println!("home is :{home:#?}");
//     println!("loopback is : {loopback:#?}");
//
//     println!("m is : {m:#?}");
//     println!("n is : {n:#?}");
//     home.print_hello();
// }

//NOTE: EXAMPLE OF MATCH EXPRESSION

#[derive(Debug)]
#[allow(unused)]
enum Coin {
    Pani,
    Nikal,
    Dime,
    Quarter,
}
//
// fn value_in_coin(coin: Coin) -> u8 {
//     match coin {
//         Coin::Pani => 1,
//         Coin::Dime => 10,
//         Coin::Nikal => 5,
//         Coin::Quarter => 25,
//     }
// }
//
// fn dice_roll() {
//     let number = 6;
//     match number {
//         6 => {
//             println!("value is given.");
//             6;
//         }
//         8 => {
//             println!("value is taken.");
//             8;
//         }
//         _other => {
//             println!("this is other called.");
//         }
//     }
// }
//
// fn main() {
//     println!("the value of coin is : {}", value_in_coin(Coin::Dime));
//     dice_roll();
//
//
// NOTE: IF LET AND LET ELSE CONTROL FLOW



fn main() {
    let max_config = Some(5u8);
    if let Some(max)= max_config{
        println!("the max config is : {max}");
    };

    
    let value_of_check: Option<u8> = Some(10);

    let Some(_max)= value_of_check  else {
        println!("not avalible.");
        return;
    };
}


