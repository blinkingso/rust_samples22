//!# lib crate
//! 查询命令行指令
//! 执行测试
use std::env;
use std::error::Error;
// run 函数定义
// use 语句
// Config定义
// Config::new函数的定义
use std::fs;

///
/// 运行
/// # 执行查询的逻辑部分, 一下为部分代码片段的注释
/// ```
/// let a: u32= 10;
/// let b : u32 = 20;
/// let b = a * b;
/// println!("now b is {}", b);
/// ```
///
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?将Error网上抛出, 将由上层调用者处理错误的部分
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    // 是否忽略大小写
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("没有查询的字符串")
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("缺少查询的文件名")
        };

        // 是否设置了此环境变量, 如果存在,则认为大小写不敏感
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
        Ok(Config { query, filename, case_insensitive })
    }
}

// 'a 标记返回的数组的生命周期和contents一致
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .into_iter()
        .filter(|line| line.contains(query))
        .collect()
}

// 大小写不敏感的查询
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

// 测试驱动开发
// test-driven development
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn one_result_case_insensitive() {
        let query = "DUCT";
        let contents = "\
        Rust:
safe, fast, Productive.
        Pick three.";


        assert_eq!(vec!["safe, fast, Productive."], search_case_insensitive(query, contents))
    }
}