struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User{
        username: String::from("tomonori"),
        email: String::from("hakurei@reimu.ru"),
        sign_in_count: 1,
        active: true
    };
    println!("{}", user1.username);
    user1.username = "yuki".to_string();
    println!("{}", user1.username);

    let user2 = User{
        username: "tomonori".to_string(),
        email: "gutrse3321@live.com".to_string(),
        ..user1
    };
    println!("user2.active: {}", user2.active);

    let user3 = build_user("464189307@qq.com".to_string(), "zero".to_string());
    println!("user3.username: {}", user3.username);
}

fn build_user(email: String, username: String) -> User {
    User{
        username,
        email,
        sign_in_count: 0,
        active: false
    }
}

/**
 * 没有命名字段的元组结构体
 */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn func_1() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/**
 * 没有任何字段的类单元结构体
 * 类单元结构体（unit-like structs）因为它们类似于 ()，即 unit 类型。
 * 类单元结构体常常在你想要在某个类型上实现 trait
 * 但不需要在类型中存储数据的时候发挥作用
 */
struct No {}
