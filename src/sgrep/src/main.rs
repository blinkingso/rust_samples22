use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).map_err(|error| {
        println!("{}", error);
        return;
    }).unwrap();

    println!("在文件{}中搜索: {}", &config.query, &config.filename);

    let contents = fs::read_to_string(&config.filename)
        .expect("文件读取失败!");

    println!("读取到文本: {}", &contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("命令行参数个数错误!!!, 需要{}个, 传入个数: {}", 2, args.len() - 1));
        }

        // clone new arguments
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}