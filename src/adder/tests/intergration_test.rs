use adder::*;
use adder::common;

#[test]
fn it_add_works() {
    assert_eq!(4, add_two(2))
}

#[test]
fn it_add_works2() {
    // 函数执行
    common::common_test();

    println!("你好, 集合测试中...");
    assert_eq!(4, add_two(2))
}