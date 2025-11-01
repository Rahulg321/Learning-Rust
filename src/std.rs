use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
enum IpAdrressKind {
    V4,
    V5,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn send_message(&self) {}
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAdrressKind,
    address: String,
}

impl IpAddress {
    fn print_something(&self) {
        println!("value of kind is {:?}", self.kind);
    }
}

fn route(kind: IpAdrressKind) {
    println!("kind is {:?}", kind);
}

fn take_ownership(s: String) {}
fn take_copy(n: i32) {}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}

#[derive(Debug)]
enum State {
    California,
    NewYork,
    Alaska,
    Hawaii,
}

impl State {
    fn existed_in(&self, year: u32) -> bool {
        match self {
            State::California | State::NewYork => year >= 1788,
            State::Alaska => year >= 1959,
            State::Hawaii => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

struct BookSummary<'a> {
    title: &'a str,
    author: &'a str,
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn some_other_main() {
    let mut counter = 3;

    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }

    for i in 1..5 {
        println!("{}", i)
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Rahul"), 12);
    scores.insert(String::from("Surbhi"), 29);
    scores.insert(String::from("juliaane"), 69);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let found_score = scores.get("Surbhi");

    if let Some(&str) = found_score {
        println!("the value of score does exist {:?}", str);
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let row1 = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(12.21),
        SpreadsheetCell::Text(String::from("hello world")),
    ];

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v1 = vec![1, 2, 3];

    let v2: Vec<i32> = Vec::new();

    v1.push(4);

    let s1 = "abcd".to_string();
    let s2 = "xyz".to_string();
    let res = longest(&s1, &s2);
    println!("longest: {}", res);

    let b1 = BookSummary {
        title: "hello world",
        author: "is this real",
    };
}
