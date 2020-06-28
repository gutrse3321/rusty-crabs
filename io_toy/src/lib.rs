use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}

/// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
/// dyn: dynamic 动态的 的缩写
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 会从函数中返回错误值并让调用者来处理它
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("catch: {}", line)
    }

    Ok(())
}

/// 定义一个显式生命周期 'a 并用于 contents 参数和返回值
/// Rust 不可能知道我们需要的是哪一个参数，所以需要告诉它
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    //遍历每行
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // //从 new 中返回 Result 而不是调用 panic!
        // if args.len() < 3 {
        //     panic!("没有传入足够的参数")
        // }

        if args.len() < 3 {
            return Err("没有传入足够的参数");
        }

        // 使用clone()解决变量所有权
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

