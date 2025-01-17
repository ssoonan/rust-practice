mod basics;
mod ipc;

use std::sync::Arc;
use std::thread;

struct Rectangle {
    width: u32,
    height: u32,
}
// impl로 구조체를 쓴 이후에 메서드를 개별로 fn으로 쓰기
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 모든 경우를 처리하지 않으면 컴파일 에러 발생
fn value_in_cents(coin: Coin) -> u8 {
    // match에는 coin, 변수가 들어가야 하는군
    match coin {
        Coin::Dime => 1,
        Coin::Nickel => 1,
        Coin::Penny => 1,
        Coin::Quarter => 1,
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
// 이렇게 tuple struct, tuple에도 이름을 매길 수 있다.

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // println!("Hello, world!");
    // let user1 = User {
    //     active: true,
    //     username: String::from("user1"),
    //     email: String::from("ssoonan0770@gmail.com"),
    // };
    // // 여기도 마찬가지로 mut을 붙이면 변경 가능, 아니면 변경 불가능
    // Rectangle::square(10);

    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for _ in 0..3 {
        // let data_clone = Arc::clone(&data);
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("{:?}", data_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // let data = 42; // i32는 Copy 트레잇을 구현
    // for _ in 0..3 {
    //     let handle = thread::spawn( || {
    //         println!("{}", data); // 각 스레드가 `data`의 복사본을 사용
    //     });
    // }

    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", data);
    });
    for _ in 0..3 {
        let handle = thread::spawn(move || {
            println!("{:?}", data);
        });
    }

}
