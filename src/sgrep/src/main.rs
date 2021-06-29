use std::borrow::Borrow;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("在文件{}中搜索: {}", filename, query);

    let contents = fs::read_to_string(filename)
        .expect("文件读取失败!");

    println!("读取到文本: {}", &contents);
}
