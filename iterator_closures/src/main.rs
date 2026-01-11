// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }
//
// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }
// impl Inventory {
//     fn giveway(&self, user_prefrence: Option<ShirtColor>) -> ShirtColor {
//         user_prefrence.unwrap_or_else(|| self.most_stock())
//     }
//     fn most_stock(&self) -> ShirtColor {
//         let mut nub_red = 0;
//         let mut nub_blue = 0;
//
//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => nub_red += 1,
//                 ShirtColor::Blue => nub_blue += 1,
//             }
//         }
//
//         if nub_blue > nub_red {
//             ShirtColor::Blue
//         } else {
//             ShirtColor::Red
//         }
//     }
// }
//
// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
//     };
//
//     let user_1 = Some(ShirtColor::Red);
//     let giveway1 = store.giveway(user_1);
//     println!("The user with prefrence {:?} get {:?}", user_1, giveway1);
//
//     let user_2 = None;
//     let giveway2 = store.giveway(user_2);
//     println!("The user with prefrence {:?} get {:?}", user_2, giveway2);
// }

// use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];

    println!("before defining closure: {list:?}");

    let mut change_list = || list.push(10);

    change_list();
    println!("after calling closure:{list:?}");

    // thread::spawn(move||{
    //     list.push(10);
    //     println!("the list value is :{list:?}");
    // } );
    // thread::spawn(move || println!("from thread: {list:?}"))
    //     .join()
    //     .unwrap();
}
