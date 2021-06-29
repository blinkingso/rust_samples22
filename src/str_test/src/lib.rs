#[cfg(test)]
mod tests {
    use core::fmt;
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter, Pointer};
    use std::fs::{File, read_to_string};
    use std::io::{ErrorKind, Read};
    use std::option::Option;
    use std::path::Path;
    use std::ptr::null;

    use crate::tests::trait_test_mod::{Summary, Tweet};

    #[test]
    fn test_error() {
        // panic!("测试panic宏的定义!");

        // panic!回溯
        // let v = vec![1,2,3];
        // index out of bound
        // println!("{}", &v[99]);
        let mut f = File::open(Path::new("/Users/andrew/nohup.out"));
        let mut json = String::new();
        match &mut f {
            Ok(f) => {
                let r = f.read_to_string(&mut json);
                match r {
                    Ok(size) => println!("值: {}, 长度为: {}字节", &json, size),
                    Err(_) => println!("文件不存在"),
                }
            }
            Err(_) => println!("io异常"),
        }
    }

    #[test]
    fn test_error_match() {
        let file_name = String::from("/Users/andrew/nohup2.out");
        let f = File::open(&file_name);
        match f {
            Ok(file) => {}
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        println!("文件不存在, 准备创建文件!");
                        match File::create(&file_name) {
                            Ok(f) => { println!("文件创建成功!") }
                            Err(err) => { println!("文件创建失败: {:?}", err) }
                        }
                    }
                    _ => {
                        println!("未知错误等!!!");
                    }
                }
            }
        };
    }

    // 简化match表达式
    #[test]
    fn test_error_match_simplify() {
        let file_name = String::from("/Users/andrew/nohup4.out");
        let mut file_data = String::new();

        let file = File::open(&file_name)
            // 闭包
            .map_err(|error| {
                if error.kind() == ErrorKind::NotFound {
                    // create the file
                    println!("文件{}不存在, 准备创建文件!", &file_name);
                    // 创建文件
                    let file = File::create(&file_name).map_err(|error| {
                        if error.kind() == ErrorKind::PermissionDenied {
                            panic!("您没有权限创建文件, 程序出错!");
                        } else {
                            panic!("未知错误!");
                        }
                    }).unwrap();
                    println!("文件{}创建成功!", &file_name);
                }
            }).unwrap().read_to_string(&mut file_data)
            .map_err(|error| {
                println!("文件读取异常: {:?}", error.to_string());
            });

        println!("从文件中读取到数据: {}", &file_data);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn str_test() {
        // 字符串的定义
        // slice, 字符串切片, &str. 字符串切片是一些指向存储在别处的UTF-8编码字符串的引用.
        let data = "initial contents";
        let s = data.to_string();
        // 宏定义的函数对于数据的引入是引用而不是直接拥有所有权
        println!("data is {}, and s is {}", data, s);
        // print s again.
        println!("s is {}", s);

        // str_test_move(s);
        // 编译报错: - move occurs because `s` has type `String`, which does not implement the `Copy` trait
        // println!("s 被移除(内存释放)了, 无法再次访问s: {}", s);
        str_test_borrow(s.borrow());
        println!("s 被借用(引用)了, 可以再次访问s: {}", s);


        // 字符串修改
        let mut s = s;
        s.push_str(" pushed data");
        println!("新增后的s为: {}", s);

        // 字符串拼接
        let s1 = "abc";
        let s2 = s + s1;
        let s3 = String::from(">cbd");
        // + 操作符: 等价于函数签名fn add(self, s: &str) -> String返回字符串并释出ownership.
        let s4 = s3 + &*String::from("_bcd");
        println!("s4= {}", s4);
        let s5 = s4 + String::from("_mdc").borrow();
        // 定义s6之后 s5被move, 之后对s5的引用会导致编译器报错. 可以通过to_string方法, 创建新的字符串进行add函数的处理
        // let s6 = s5 + "_move_s5";
        // to_string
        let s6 = s5.to_string() + "_none_move_s5";
        println!("s5= {}, s6 = {}", s5, s6);

        // s3 move after + return s4; so compile error;
        // -- move occurs because `s3` has type `String`, which does not implement the `Copy` trait
        // println!("s3 = {}", s3);

        // 使用format!宏拼接字符串
        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "c".to_string();
        // 通过s1.to_string()创建新的字符串.
        let s4 = s1.to_string() + s2.borrow() + s3.borrow();
        println!("s4 = {}", s4);
        println!("s4 = {}", s4);
        println!("使用format!宏");

        println!("s4 = {}", format!("{}-{}-{}", s1, s2, s3));

        // 字符串索引
        let s1 = String::from("ni在讲什么乸??");
        let len = s1.len();
        // 字符串切片, 字节索引. 必须可以UTF-8编码
        let ss1 = &s1[0..5];
        println!("len = {}, ss1 = {}", len, ss1);
        // chars遍历
        for core in s1.chars() {
            println!("char是: {}", core);
        }
    }

    fn str_test_move(s: String) {
        println!("被移除的数据: {}", s);
    }

    fn str_test_borrow(s: &str) {
        println!("被引用的数据: {}", s);
    }

    #[test]
    fn test_hash_map() {
        let teams = vec![String::from("blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter())
            .collect();

        for entry in &scores {
            println!("map: key = {}, value = {}", entry.0, entry.1);
        }

        let field_name = "color".to_string();
        let field_value = 101;

        scores.insert(field_name.borrow(), &field_value);

        scores.iter().for_each(|(k, v)| {
            println!("key = {}, value = {}", k, v);
        });

        println!("================");

        let fv = scores.get(&field_name);
        if let Some(s) = fv {
            println!("得到结果值: {}", s);
        }

        scores.insert(field_name.borrow(), &102);

        scores.iter().for_each(|(k, v)| {
            println!("key = {}, value = {}", k, v);
        });

        let mut empty_map = HashMap::new();
        empty_map.insert(String::from("B"), 123);
        empty_map.insert(String::from("C"), 67);
        let score = empty_map.get(&String::from("B"));
        match score {
            Some(v) => println!("找到值: {}", v),
            _ => println!("搜索的key不存在"),
        }

        // 检查key
        let e = empty_map.entry(String::from("B")).or_insert(50);
        // entry check.
        let text = "hello 你好 世界 never give u p o hello";
        let mut words = HashMap::new();
        text.split_whitespace().for_each(|string| {
            // 初始化或取出已经存在的个数
            let count = words.entry(string).or_insert(0);
            // 个数+1 *解引用, 计算值
            // * 解引用后才能对count变量进行赋值操作
            *count += 1;
        });

        println!("{:?}", words);
    }

    mod trait_test_mod {
        use std::fmt::{Display, Formatter, Pointer};
        use std::fmt;

        pub trait Summary {
            // 摘要汇总, 提供一个默认实现
            fn summarize(&self) -> String {
                String::from("Read more...")
            }
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        // 只能实现一次, 否则编译报错: conflicting implementations of trait `adder.tests::trait_test_mod::Summary` for type `adder.tests::trait_test_mod::NewsArticle`
        // impl Summary for NewsArticle{}

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        impl fmt::Display for Tweet {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "格式化后的数据: {{username:{},content:{}, replay:{}, retweet:{}}}",
                       self.username, self.content, self.reply, self.retweet)
            }
        }
    }

    fn notify(summary: &impl Summary) {
        println!("Breaking news: {}", summary.summarize());
    }

    fn notify2<T: Summary>(t: &T) {
        println!("Breaking news: {}", t.summarize());
    }

    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }

    impl<T, U> Point<T, U> {
        fn mix<V, M>(self, other: Point<V, M>) -> Point<T, M> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    impl<T: Display, U: Display> fmt::Display for Point<T, U> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Point{{\"x\":{}, \"y\":{}}}", self.x, self.y)
        }
    }

    fn notify_function<T: Summary + Display, U: Clone + Debug>(t: T, u: U) -> i32 {
        let r = t.summarize();
        println!("R is : {}", r);

        10
    }

    // where从句改写trait约束
    fn notify_function_01<T, U>(t: T, u: U) -> i32
        where T: Display + Summary, U: Clone + Debug {
        let r = t.summarize();
        println!("R is : {}", r);

        100
    }

    // 返回trait约束
    fn returns_summarize(username: String, content: String) -> impl Summary {
        Tweet {
            username,
            content,
            reply: false,
            retweet: false,
        }
    }

    fn largest<T>(arrays: &Vec<T>) -> &T where T: PartialOrd {
        let mut max = &arrays[0];
        for x in arrays.iter() {
            if x > max {
                max = &x;
            }
        }

        max
    }

    fn largest2<T>(arrays: &[T]) -> T where T: PartialOrd + Copy + Clone {
        // 直接取出结果, 实现了Copy trait
        let mut max = arrays[0];
        for &x in arrays.iter() {
            if x > max {
                max = x;
            }
        }

        max
    }

    // 通过调用clone方法返回一个T值的所有权的实例T
    fn largest3<T>(arrays: &[T]) -> T where T: PartialOrd + Clone {
        let mut max = &arrays[0];
        for x in arrays.iter() {
            if x > max {
                max = x
            }
        }
        max.clone()
    }

    #[test]
    fn test_largest() {
        let arrays = vec![10, 0, 100, 8, 99];
        println!("largest number is : {}", largest(&arrays));
        println!("第2个元素为: {}", arrays[1]);

        println!("largest2 number is : {}", largest2(&arrays));
        println!("largest3 number is : {}", largest3(&arrays));
    }

    #[test]
    fn test_point() {
        let mut p1 = Point {
            x: 10,
            y: 100.01,
        };
        let mut p2 = Point {
            x: "helen",
            y: 'c',
        };

        // p1,p2 moved
        let p3 = p1.mix(p2);
        println!("p3 is : {}", &p3);
    }

    // trait cmp_display
    pub struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T: PartialOrd + Display> Pair<T> {
        pub fn cmp_display(&self) {
            if self.x > self.y {
                println!("The largest member is x = {}", self.x.to_string());
            } else {
                println!("The largest member is y = {}", self.y.to_string());
            }
        }
    }

    #[test]
    fn test_cmp_display() {
        let pair = Pair { x: 10, y: 20 };
        pair.cmp_display();
    }

    #[test]
    fn test_trait() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("content of horse!!!"),
            reply: false,
            retweet: false,
        };

        println!("one new tweet : {}", tweet.summarize());
        println!("one new tweet : {}", tweet);

        notify(tweet.borrow());
        notify2(tweet.borrow());
    }

    // 使用声明周期保证引用的有效性.
    // 避免悬垂引用
    #[test]
    fn test_lifetime() {
        let r;
        {
            let x = 5;
            r = &x;
        }
        // x在}之后被释放: `x` does not live long enough
        // println!("R: {}", r);

        let x = String::from("abcs");
        let y;
        {
            // y 在执行第一个}之后生命周期还在!
            y = "abc";
            println!("longest: {}, and x is {}, y is {}", longest(x.as_str(), y), x, y);
            println!("longest2: {}, and x is {}, y is {}", longest2(x.as_str(), y), x, y);
        }
        println!("x is {}, y is {}", x, y);
    }

    #[test]
    fn test_lifetime2() {
        let string1 = String::from("longest string definition.");
        // let result;
        {
            let string2 = "short str".to_string();
            // result 的声明周期不能超过引用的参数的声明周期
            // 本例中result的生命周期>string2. 编译报错: `string2` does not live long enough
            // result = longest(string1.as_str(), string2.as_str());
        }

        // println!("longest str is : {}", result);
    }

    #[test]
    fn test_lifetime3() {
        let s1 = "longest string definition";
        let s2 = "short ones";
        // s1, s2, result在同一个生命周期
        let result;
        {
            result = longest(s1, s2);
            let r2 = longest(s1, s2);
            println!("b生命周期中最大值为: {}", r2);
        }
        println!("largest result is : {}", result);
    }

    #[test]
    fn test_lifetime4() {
        //result的生命周期> s1,s2: 编译报错:  `s1` does not live long enough
        // let result;
        {
            let s1 = "longest string definition".to_string();
            // s1, s2 在同一个生命周期
            let s2 = "short ones";
            // result = longest(s1.as_str(), s2);
            let r2 = longest(s1.as_str(), s2);
            println!("b生命周期中最大值为: {}", r2);
        }
        // println!("largest result is : {}", result);
    }


    // 'a 声明周期的名称, 被标注表示同一个生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 生命周期消息的引用
    fn longest2(x: &str, y: &str) -> String {
        if x.len() > y.len() {
            x.to_string()
        } else {
            y.to_string()
        }
    }

    pub struct Rectangle<T: Clone + PartialOrd> {
        pub x: T,
        pub y: T,
    }

    impl<T: Clone + PartialOrd> Rectangle<T> {
        pub fn can_hold(&self, other: &Rectangle<T>) -> bool {
            self.x > other.x && self.y > other.y
        }
    }

    impl<T: Clone + PartialOrd + Display> fmt::Display for Rectangle<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", format!("Rectangle {{ \"x\": {}, \"y\": {} }}", &self.x, &self.y))
        }
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }


    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, but got {}.", value);
            }

            Guess {
                value
            }
        }
    }

    fn internal_adder_test(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod new_test {
    use crate::tests::{Rectangle, add_two, Guess};
    use super::internal_adder;
    #[test]
    fn test_assert() {
        let a = Rectangle { x: 10, y: 20 };
        let b = Rectangle { x: 18, y: 20 };

        assert!(a.can_hold(&b), "can not hold the rect : `{}`", b);
    }

    #[test]
    fn test_assert_eq_and_ne() {
        let a = 10;
        let b = add_two(a);

        assert_eq!(12, b);
        assert_ne!(12, add_two(b));
    }

    #[test]
    #[should_panic(expected = "不能超过100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four!"))
        }
    }

    // 显示函数输出
    fn prints_and_returns(a: i32) -> i32 {
        println!("得到值: {}", a);
        10
    }

    #[test]
    fn test_pass() {
        let v = prints_and_returns(4);
        assert_eq!(10, v)
    }

    #[test]
    #[ignore] // cargo test时会忽略此测试
    fn test_pass_fail() {
        let v = prints_and_returns(10);
        assert_eq!(5, v)
    }

    // 测试私有函数
    #[test]
    fn test_private_fn() {
        assert_eq!(5, internal_adder(2,3));
    }
}