mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
        pub fn seat_at_table() {
            println!("4");
        }
    }

    mod serving {
        fn take_order() {
            println!("order taken");
        }
        fn serve_order() {
            println!("order served");
        }
        fn take_payment() {
            println!("payment taken");
        }
    }
}

pub fn eat_at_resturant() {
    front_of_house::hosting::seat_at_table();
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();
}

fn delivery_order() {
    println!("Food is delivered");
}

mod back_of_house {
    pub fn fix_inccorect_orer() {
        println!("fixed");
        cook_food();
        super::delivery_order();
    }
    pub fn cook_food() {
        println!("food is cooked");
    }
}

