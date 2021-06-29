// use crate::front_of_house;

fn fix_incorrect_order() {
    cook_order();
    // 通过super关键字调用相对路径的module的函数
    super::server_order();
}

fn cook_order() {}