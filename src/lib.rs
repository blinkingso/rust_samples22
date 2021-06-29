// 模块化 module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_table() {
        }
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

    pub mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            // 通过super关键字调用相对路径的module的函数
            super::server_order();
        }

        fn cook_order() {}


        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }

        pub fn create_breakfast(toast: &str, fruit: String) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: fruit,
            }
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    fn server_order() {}
}


// 通过use导入路径时, 必须在声明之后的部分
use front_of_house::{hosting, back_of_house};

pub fn eat_at_restaurant() {
    // 父模块不可以调用子模块的函数等, 除非子模块的函数等定义为pub来暴露路径(paths)
    hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 修改
    meal.toast = String::from("Wheat");
    println!("like {}, please.", meal.toast);

    // 编译err
    // meal.season_fruit = String::from("blueberries");

    let m = back_of_house::create_breakfast("tus", String::from("banana"));

    // all public
    let m = back_of_house::Appetizer::Salad;

    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}

use rand::Rng;
use std::{cmp::Ordering, io, collections};

fn test_rng() {
    let number = rand::thread_rng().gen_range(1..101);
    println!("生成的随机数为: {}", number);
}