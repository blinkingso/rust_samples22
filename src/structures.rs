#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    sex: String,
    age: u8,
}

struct Ipv4 {
    ip: String,
    ip_address: (u8, u8, u8, u8),
    host: String,
}

struct Ipv6 {
    ip: String,
    ip_address: (u8, u8, u8, u8, u8, u8),
    host: String,
}

enum Ip {
    V4(Ipv4),
    V6(Ipv6),
}

pub enum State {
    N,
    M,
}

impl State {
    pub fn get_u8(&self) -> u8 {
        match &self {
            State::N => 0,
            State::M => 1
        }
    }
}

pub enum Coin {
    Penny,
    Nickel,
    Quarter(State),
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 4,
        Coin::Quarter(s) => s.get_u8()
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let mut user = User {
        id: 10,
        name: String::from("yaphets"),
        sex: String::from("male"),
        age: 10,
    };

    println!("user.id is : {}", user.id);
    user.name = String::from("android");

    let val = value_in_coin(Coin::Quarter(State::M));
    println!("now result is {}", val);

    // tests plus_one
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("tests plus_one result is, six = {:#?}, none = {:#?}", six, none);
}

struct Rng {
    id: u32
}