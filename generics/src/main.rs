use crate::traits::Summary;
use std::fmt::Display;

mod traits;

/// 泛型代码的性能
/// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
/// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("largest: {}",largest(&number_list));

    /// 这里只有一个泛型类型，所以x和y的类型要一致
    let integer = Point {x: 15, y: 30};
    let float = Point {x: 1.0, y: 2.0};
    println!("float getX={}", float.getX());
    // 这样两个泛型类型的话就可以
    let cus = CusPoint {x: 1, y: 2.0};
    let cus_1 = CusPoint {x: 3.4, y: 2};

    /// trait
    let tweet = traits::Tweet {
        username: "gutrse3321".to_string(),
        content: "fucker".to_string(),
        reply: false,
        retweet: false
    };
    println!("tweet: {}", tweet.summarize());
    println!("tweet other_summarize: {}", tweet.other_summarize());
    // trait参数
    let news = traits::NewsArticle {
        headline: "fake news!".to_string(),
        location: "USA".to_string(),
        author: "fucker".to_string(),
        content: "big fake news".to_string()
    };
    traits::notify(news);
    // trait bound语法方式，泛型
    traits::notify_gen(tweet);
    // 返回实现trait的tweet
    let tweet_2 = traits::return_summarizable();
    println!("tweet_2: {}", tweet_2.summarize());

    /// 使用泛型函数的比大小
    let number_list = vec![34, 50, 25, 100, 65];
    println!("largest_gen i32: {}", largest_gen(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("largest_gen char: {}", largest_gen(&char_list));
}

/// number > largest：binary operation `>` cannot be applied to type `T`
/// 如果我们想要使用大于号比较两个泛型T的值，需要使用std::cmp::PartialOrd trait的一个默认方法
/// 所以需要实现PartialOrd的类型，使用 fn gt()，详情看源码
///
/// list[0] 和 largest = （number）：cannot move out of type `[T]`, a non-copy slice
/// 需要实现Copy trait的类型，否则无法拷贝栈上的数据
/// Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。
/// 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。
/// Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait（eg: String）
fn largest_gen<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

/// 结构体定义中的泛型
struct Point<T> {
    x: T,
    y: T,
}

struct CusPoint<T, R> {
    x: T,
    y: R,
}
// 方法定义中的泛型
impl <T> Point<T> {

    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    fn getX(&self) -> &T {
        &self.x
    }
}
/// 使用trait bound 有条件的实现方法
/// PartialOrd（允许比较） 和 Display（启用打印）
impl<T: Display + PartialOrd> Point<T> {

    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// 枚举定义中的泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
