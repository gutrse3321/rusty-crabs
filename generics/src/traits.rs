use std::fmt::Display;

/// trait
/// 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
/// 可以通过 trait 以一种抽象的方式定义共享的行为。
/// 可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
/// 类似其他语言的接口(interface)的功能，但有些不同

pub trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn other_summarize(&self) -> String {
        "default impl".to_string()
    }
}

/// 为类型实现trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", &self.headline, &self.author, &self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}

/// trait作为参数
/// 传入实现这个trait的结构体
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// trait bound 语法
pub fn notify_gen<T: Summary>(item: T) {
    println!("Breaking tweet! {}", item.summarize());
}

/// 使用 + 指定多个trait
/// 即传入实现Summary和std::fmt::Display的结构体
pub fn notify_plus_impl(item: impl Summary + Display) {
}

pub fn notify_plus_gen<T: Summary + Display>(item: T) {
}

/// 使用 where 简化trait bound（多个的情况）
pub fn notify_gen_manyT<T, U>(t: T, u: U) -> i32 where T: Display + Summary, U: Summary {
    1
}

/// 返回实现了trait的类型
pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: "yuki".to_string(),
        content: "mememe".to_string(),
        reply: false,
        retweet: false
    }
}
