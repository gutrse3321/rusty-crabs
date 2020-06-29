use std::{env, process};

use io_toy::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args)
        .unwrap_or_else(| err | {
            eprintln!("参数有误：{}", err);
            process::exit(1);
        });
    println!("searching for: {}", config.query);
    println!("filename: {}", config.filename);

    if let Err(e) = io_toy::run(config) {
        eprintln!("app错误：{}", e);
        process::exit(1);
    }
}
