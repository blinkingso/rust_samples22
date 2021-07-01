use std::env;
use std::process;

use sgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run(config).unwrap_or_else(|err| {
    //         // Error 实现了Display trait
    //         println!("查询错误: {}", err);
    //         process::exit(1);
    //     });
    if let Err(e) = sgrep::run(config) {
        eprintln!("查询错误: {}", e);

        process::exit(1);
    }
}

///
/// 基本注释
///# examples 文档注释
///
/// ```
/// let a: u32 = 12;
/// ```
///
///
pub fn add_one(x: i32) -> i32 {
    x * 32
}