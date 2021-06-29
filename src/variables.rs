use std::env;

use std::ops::*;
use std::ops::Mul;

fn main() {
    let x = 5;
    println!("x is {}", x);
    // compile error
    // x = 6;
    let mut y: u32 = 5;
    y = 6;

    let x = x + 10;
    let x = x + 100;
    // 同一个字面量来创建不同类型的变量
    let x = "hello";


    println!("max point is : {}, x is : {}", MAX_POINT, x);


    let name = env::args().skip(1).next();

    match name {
        Some(s) => println!("名称: {}", s),
        None => panic!("参数缺失了...?")
    }

    let spaces = "  ";
    let spaces = spaces.len();
    let mut spaces = "  ";
    // compile error
    // spaces = spaces.len();
    // 数值
    let guess: u32 = "42".parse().expect("不是一个数字!");
    // scalar types: integers, floating-point numbers, Booleans and characters.
    // integers: signed and unsigned integers. from 8bit->128bit(位)
    let u8: u8 = 255;
    let i8: i8 = 127;
    // binary
    let b: u8 = 0b1111_0000;
    let b: i8 = -0b0111_0000;
    // literal out of range
    //hex 0xff=255
    // let b : i8 = 0xff;
    let b: u8 = 0xff;
    // octal 63=7*8^1 + 7* 8^0
    let b: u8 = 0o77;
    // byte 65
    let b: u8 = b'A';
    // decimal
    let b: u32 = 98_222;
    println!("u8 is {}, i8 is {}, b is {}", u8, i8, b);
    let x = 2.1;
    let y: f32 = 3.0;
    let z: f64 = 3.01;
    let z = x * z;
    println!("z is {}", z);

    let t = true;
    let t: bool = false;
    let cat = ' ';
    let heart_eyed_cat = '😻';
    let heart_eyed_cat = '中';
    println!("heart is {}", heart_eyed_cat);

    // 复合数的表示
    let tum: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tum;
    println!("value is : {}, z is {}", y, tum.2);

    // 数组
    let a: [f32; 6] = [1.1, 2.1, 3.1, 4.1, 5.1, 4.1];
    let a = [10.0; 6];
    let first = a[0];
    let second = a[1];

    let m: usize = 10;
    let m: isize = -109999999999999999;

    println!("max value between 10 and 100 is {}", operation(max, 10, 100));
    println!("min value between 10 * 2 and 100 is {}", operation(min_between2y, 10, 100));
    print_max(1, 2, false);
}


fn max(x: i32, y: i32) -> i32 {
    return if x > y { x } else { y };
}

// not doc type comments
fn min_between2y(x: i32, y: i32) -> i32 {
    let x = {
        let x1 = x;
        let x1 = 2 * x1;
        x1
    };
    return if x < y { x } else { y };
}

fn print_max(x: u32, y: u32, z: bool) {
    if x > y && z {
        println!("yes !!!");
    } else {
        println!("No .........");
    }

    let value = if z { "world" } else { "hello" };
    println!("value is : {}", value);

    let mut i = 0;
    loop {
        println!("again");
        i += 1;
        if i > 10 {
            println!("over now!");
            break;
        }
    }

    while i < 20 {
        i = i + 1;
        println!("while loop again.");
    }

    for i in 0..10 {
        self::operation(max, i, 5);
    }

    let m = [10; 5];
    for e in m {
        let e = e.mul(20);
        println!("array value is : {}", e);
    }


    for e in m.iter() {
        // mul
        let e = e.mul(10);
        println!("array in iter value is : {}", e);
    }

    for number in(1..4).rev() {
        println!("{}!", number)
    }
}

/**
*functional programing
*/
fn operation(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    return f(x, y);
}

const MAX_POINT: u32 = 100_000_1000;