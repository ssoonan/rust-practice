use core::error;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::iter::Sum;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {}

trait Summary {
    // struct와 같은 계층으로 쓰는 것으로 보임. 일단 trait으로 정의한 이후에 fn으로 ... 들어감
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let mut v = Vec::new();
        v.push(5);

        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {}
}

fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

impl Tweet {
    fn init(&self) -> Self {
        Tweet {}
    }
}

// 일단 선언한 trait에서 어떤 연관 관계는 없지만.. 이를 for로 구현해서 추가 기능 구현이 가능.

fn file() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file1 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    let greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

enum MyError {
    Io(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::Io(error)
    }
}

fn read_username_from_file() -> Result<String, MyError> {
    let mut username_file = File::open("hello.txt")?; // std::io::Error -> MyError로 변환
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // std::io::Error -> MyError로 변환
    Ok(username)
}

enum List {
    Cons(i32, Box<List>),
    Nil, // emum안에 들어갔으니 Cons, Nil은 문제가 없는데 이 List를 안에 또 넣어서 컴파일이 안 되는군
}

use crate::List::{Cons, Nil};

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} // 이렇게 enum 안에 구조체를 바로 넣을 수 있나

fn make_message() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    Message::Move { x: 5, y: 10 };

}
