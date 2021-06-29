use std::cmp::Ordering;
use std::collections;
use std::io;
use std::num::TryFromIntError;
use std::option::Option::Some;

use rand::Rng;

use t_c::test_vector;

use crate::structures::{Coin, State};
use std::borrow::Borrow;

mod structures;
mod variables;
mod lib;
mod front_of_house;
mod t_c;

fn main() {
    // guess_game();
    // place_holder();
    // if_let();
    // test_map();
    // test_vector::main();
    // test_ownership();
    test_vector();
}

fn test_vector() {
    let mut m = vec![1, 2, 3, 4, 5];
    // 修改vec中元素的值
    for mut i in &mut m {
        *i += 10;
    }

    match m.pop() {
        Some(io) => println!("{}", io),
        None => {println!("空数组")}
    }

    for i in m {
        println!("i is : {}", i);
    }

    let row = vec![
        SpreadSheetCell::Int(10),
        SpreadSheetCell::Float(10.1),
        SpreadSheetCell::Text("行".to_string()),
        SpreadSheetCell::Int(108)
    ];

    for rand in row {
        println!("row: {:?}", rand)
    }
}

// tests the place holder _
fn place_holder() {
    let some_u8_value = 0o01;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("others")
    }
}

fn test_map() {
    let mut map = collections::HashMap::new();
    map.insert("hello", "世界1");
    map.insert("hello1", "世界2");
    println!("key is {} and value is {}", "hello", match map.get_mut("hello") {
        Some(str) => &str,
        None => "key不存在",
    });
}

// tests if let
fn if_let() {
    let some_u8_value = Some(0x10);
    if let Some(16) = some_u8_value {
        println!("16");
    }

    if some_u8_value == Some(16) {
        println!("真16?");
    }

    // origin
    let mut count = 0;
    let coin = Coin::Quarter(State::M);

    let coin1 = &coin;
    let coin2 = &coin;
    // 拿到coin的引用
    match coin1 {
        Coin::Quarter(state) => {
            println!("State is : {}", state.get_u8());
        }
        _ => count += 1,
    }
    // replace via if let or else.
    if let Coin::Quarter(s) = coin2 {
        println!("状态为: {}", s.get_u8());
    } else {
        count += 1;
    }
}

fn guess_game() {
    println!("猜数字游戏:");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("生成的谜底数字为: {}", secret_number);
    println!("请你开始猜吧!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("读取数据失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("监控到异常: {}, 请输入正确的数字!", error);
                continue;
            }
        };

        println!("你猜的数字为: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("猜小了"); }
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
            Ordering::Greater => { println!("猜大了"); }
        }
    }
}

// 理解下ownership, borrowing, move, slice.
fn test_ownership() {
    fn test_inner(s: &str) {
        println!("test_inner s is : {}", s);
    }

    // 函数,将获取s的ownership. borrowing from s; will move s's ownership.
    fn test_inner_str(s: String) {
        println!("test_inner_str s is : {}", s);
    }

    // let s = "hello";
    let s = "hello".to_string();
    // let mut s2 = "hello";
    // let mut s3 = "hello".to_string();

    test_inner(&s);
    println!("now s is : {}", s);
    // 创建新的字符串
    let m = s;
    test_inner_str(m);
    // value moved, memory s already released after let m = s;
    // + +=的函数签名的&self, +=self模式,会获取数据的ownership
    // println!("now s is : {}", s);

    // test_outer(&s);
    // println!("now s is : {}", s);
    // test_outer_str(s);
    // println!("now s is : {}", s);
}

fn test_outer(s: &str) {
    println!("test_inner s is : {}", s);
}

fn test_outer_str(s: String) {
    println!("test_inner_str s is : {}", s);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),

}