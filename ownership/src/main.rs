// doc https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html
fn main() {
    // 调用from时，请求其所需的内存
    let mut s = String::from("hello"); // s有效
    s.push_str(", tomonori");
    println!("{}", s);

    func_1();
} // 作用域已结束
  // s 不再有效
// 内存在拥有它的变量离开作用域后就被自动释放
// rust调用特殊函数 drop，在 } 自动调用

fn func_1() {
    // 变量赋值变量
    // 类似浅拷贝，但是之前的值将无效
    // rust称为 移动
    let s1 = String::from("hello");
    let s2 = s1; // 拷贝s1的指针、长度、深度。rust将认为s1不再有效
    println!("{}, tomonori", s2);
    // 当s1和s2离开作用域，他们都会尝试释放相同的内存
    // 这叫做二次释放(double free)的错误，内存安全性bug之一
    // s1在被s2拷贝后，rust将s1认为不再有效，则在离开作用域时，不需要清理s1任何东西

    // 确实需要赋值且需要新的内存空间(深拷贝)
    // 使用clone方法
    let s3 = String::from("deeper");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    // 已知大小的类型，int32 4个字节，整个存在栈上
    // 所以这里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它
    // 一个类型拥有Copy trait，之前的变量在将其赋值给其他变量后任然可以使用
    // rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
    let num1 = 5;
    let num2 = num1;
    /**
        所有整数类型，比如 u32。
        布尔类型，bool，它的值是 true 和 false。
        所有浮点数类型，比如 f64。
        字符类型，char。
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
     */
    println!("{}, {}", num1, num2);
}

fn func_2() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

/**
 * 引用与借用
 */
fn func_3() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    //TODO 不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用
    // 使用一个新的作用域，就可以，但是不是 同时 拥有
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    change(&mut s);
}

// & 符号就是引用，它允许你使用值但不获取其所有权
// &String s 指向 String s1： s的指针存的是 上面s1的指针
//TODO 我们将获取引用作为函数参数称为 借用（borrowing）
// 不可修改借用的变量
fn calculate_length(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// 可变引用
// 必须将 s 改为 mut。
// 然后必须创建一个可变引用 &mut s 和接受一个可变引用 some_string: &mut String
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
