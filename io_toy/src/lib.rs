use std::error::Error;
use std::{fs, env};

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

    /// 查询关键词 大小写不区分
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}

/// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
/// dyn: dynamic 动态的 的缩写
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 会从函数中返回错误值并让调用者来处理它
    let contents = fs::read_to_string(config.filename)?;

    // 使用迭代器的适配器的方式
    let res = if config.case_sensitive {
        search_iter(&config.query, &contents)
    } else {
        search_case_insensitive_iter(&config.query, &contents)
    };

    // let res = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };

    // for line in search(&config.query, &contents) {
    //     println!("catch: {}", line)
    // }

    for line in res {
        println!("case catch: {}", line)
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

/// 大小写不区分的查询
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line)
        }
    }
    res
}

/// 使用迭代器适配器来使代码更简明
pub fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, // 是否大小写不区分
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
        // 检查环境变量CASE字段
        let case_sensitive = env::var("CASE").is_err();
        Ok(Config { query, filename, case_sensitive})
    }

    // 实现Iterator的std::env::Args结构体
    pub fn new_with_iter(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有取得搜索字段"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("没有取得文件名称"),
        };

        let case_sensitive = env::var("CASE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

