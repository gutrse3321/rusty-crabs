use std::{env, process};

use io_toy::{Config};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // (1) 直接传vector
    // let config = Config::new(&args)
    //     .unwrap_or_else(| err | {
    //         eprintln!("参数有误：{}", err);
    //         process::exit(1);
    //     });

    // （2）传一个迭代器
    // 我们直接将 env::args 返回的迭代器的所有权传递给 Config::newWithIter
    // 这样就无需在去args值的时候无法取值(clone)
    let config = Config::new_with_iter(env::args())
        .unwrap_or_else(|err| {
            eprintln!("（2）参数有误：{}", err);
            process::exit(1);
        });
    println!("searching for: {}", config.query);
    println!("filename: {}", config.filename);

    if let Err(e) = io_toy::run(config) {
        eprintln!("app错误：{}", e);
        process::exit(1);
    }
}
