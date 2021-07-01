use std::ptr::eq;
use std::thread;
use std::time::Duration;

use rand;
use rand::Rng;

use crate::test_closure::CacheClosure;
use rust_samples::{mix, PrimaryColor};
use std::borrow::Borrow;
use std::fmt::Pointer;

fn main() {
    println!("主线程!");
    let simulated_user_specified_value = rand::thread_rng().gen_range(1..30);
    let simulated_random_number = rand::thread_rng().gen_range(1..10);

    // generate_workout(simulated_user_specified_value, simulated_random_number);
    // test_closure::test_closure()
    // closure_context();

    test_iter::test_it();

    mix(PrimaryColor::Blue, PrimaryColor::Yellow);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("慢操作....");
    thread::sleep(Duration::from_secs(3));
    println!("线程: {}", thread::current().name().unwrap());
    intensity
}

fn generate_workout3(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("今天做了{}个俯卧撑", simulated_expensive_calculation(intensity));
        println!("接下来, 做了{}个仰卧起坐", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息一下!!!");
        } else {
            println!("今天跑步{}分钟!", simulated_expensive_calculation(intensity));
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num| {
    //     println!("closure的慢操作...");
    //     thread::sleep(Duration::from_secs(2));
    //     println!("线程: {}", thread::current().name().unwrap());
    //     num
    // };
    //
    // let expensive_result = expensive_closure(intensity);

    // 基于closure的模型
    let mut expensive_result = CacheClosure::new(|num| {
        println!("cached closure的慢操作...");
        thread::sleep(Duration::from_secs(2));
        println!("线程: {}", thread::current().name().unwrap());
        num
    });
    let expensive_result0 = expensive_result.value(intensity);
    let expensive_result1 = expensive_result.value(10);

    println!("e0 is {}, e1 is {}", expensive_result0, expensive_result1);

    let expensive_result = expensive_result0;

    if intensity < 25 {
        println!("今天做了{}个俯卧撑", expensive_result);
        println!("接下来, 做了{}个仰卧起坐", expensive_result);
    } else {
        if random_number == 3 {
            println!("今天休息一下!!!");
        } else {
            println!("今天跑步{}分钟!", expensive_result);
        }
    }
}

//上下文闭包定义
fn closure_context() {
    let x = vec![10, 20, 30, 40];

    // x 被move了
    // let equal_to_x = move |z| z == x;

    // 编译报错 move关键字
    println!("x在这里无法使用: {:?}", x);

    let y = vec![10, 20, 30, 40];

    // assert!(equal_to_x(y));
}

//闭包
pub mod test_closure {
    use std::collections::HashMap;

    pub fn add_one_v1(x: u32) -> u32 { x + 1 }

    pub fn test_closure() {
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| { x + 1 };
        let add_one_v4 = |x| x + 1;

        let i = add_one_v2(10);
        let i = add_one_v3(i);
        let i = add_one_v4(i);
        println!("i is : {}", i);
    }

    // 结构体存储closure
    pub struct CacheClosure<T> where T: Fn(u32) -> u32 {
        calculation: T,
        values: Option<HashMap<u32, u32>>,
    }

    impl<T> CacheClosure<T> where T: Fn(u32) -> u32 {
        pub fn new(calculation: T) -> CacheClosure<T> {
            CacheClosure {
                calculation,
                values: None,
            }
        }

        pub fn value(&mut self, arg: u32) -> u32 {
            match &mut self.values {
                Some(v) => {
                    match v.get(&arg) {
                        // * 解引用
                        Some(value) => *value,
                        None => {
                            let v32 = (self.calculation)(arg);
                            v.insert(arg, v32);
                            v32
                        }
                    }
                }
                None => {
                    let v = (self.calculation)(arg);
                    let mut map = HashMap::new();
                    map.insert(arg, v);
                    self.values = Some(map);
                    v
                }
            }
        }
    }
}


// 迭代器
pub mod test_iter {
    use std::ops::Rem;

    pub fn test_it() {
        let v1 = vec![1, 2, 3];
        // 创建了一个迭代器, 但是在没有主动调用方法来消耗并使用迭代器前,不会产生任何的实际效果.
        let mut v1_iter = v1.iter();
        // 遍历迭代器
        for val in v1_iter {
            println!("got: {}", val);
        }

        // 元组的定义dictionary
        let m = (1, 2, 3, 4);
        println!("{}-{}-{}-{}", m.0, m.1, m.2, m.3);
        // Iterator trait和next方法
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // 消耗迭代器的方法
        let mut v1_iter = v1.iter();
        // 汇总求和, 在调用sum方法的过程中获取了v1_iter的所有权
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
        // 生成其他迭代器的方法, 迭代器适配器方法
        // 求平方, 创建了新的迭代器, 对迭代器进行修改.
        let total: i32 = v1.iter().map(|x| x * x).sum();
        assert_eq!(total, 14);

        // 使用闭包捕获环境
        v1.iter().filter(|i| i.rem(2) == 1)
            .for_each(|i| println!("filtered i is : {}", i));

        // shoes
        let mut shoes: Vec<Shoe> = Vec::new();
        shoes.push(Shoe { size: 20, style: String::from("s1") });
        shoes.push(Shoe { size: 21, style: String::from("21码的鞋") });
        shoes.push(Shoe { size: 20, style: String::from("20码的鞋") });
        shoes.push(Shoe { size: 19, style: String::from("19码") });

        let my_shoes = shoes_in_my_size(shoes, 20);
        my_shoes.iter().for_each(|s| println!("我的鞋子: {}", s.style));

        // 使用Iterator trait来创建自定义迭代器
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);

        // zip
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);

        // 迭代器
        let buffer: &mut [i32] = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let coefficients: [i64; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let qlp_shift: i16 = 8;

        for i in 12..buffer.len() {
            let prediction = coefficients.iter()
                .zip(&buffer[i - 12..i])
                .map(|(&c, &s)| c * s as i64)
                .sum::<i64>() >> qlp_shift;

            let delta = buffer[i];
            buffer[i] = prediction as i32 + delta;
        }

        print!("buffer is : {:?}", buffer);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                count: 0,
            }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
}