use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("a.txt").unwrap();
    let reader = BufReader::new(file);

    // 줄 단위로 읽기
    for line in reader.lines() {
        match line {
            Err(e) => {
                eprintln!("error: {}", e);
            }
            Ok(line) => {
                println!("{}", line);
            }
        }
    }
}
