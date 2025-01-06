use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    // config, args의 에러 발생 가능성
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    // fs::read_to_string의 에러 발생 가능성
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
