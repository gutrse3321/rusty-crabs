use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

fn main() {
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     /// 匹配不同的错误
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Problem creating the file: {:?}", err),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error)
    //     }
    // };
    //
    // // 第二种方式
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    /// panic的简写：unwrap和expect
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    /// propagating error 传递异常
    let pro = read_username_from_file();
    println!("{:?}", pro);

    let homeAddr: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", homeAddr.to_string());
}

/// propagating error 传递异常
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
