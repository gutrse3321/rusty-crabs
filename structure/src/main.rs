struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 如果想要打印出调试信息，需要加上注解
// 增加注解来派生 Debug trait，并使用调试格式打印
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// 结构体允许拥有多个impl块
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_eat(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50
    };
    println!("rect: {:#?}", rect);
    println!("area: {}", area(&rect));
    /**
     * Rust 并没有一个与 C++ -> 等效的运算符；
     * 相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
     * 方法调用是 Rust 中少数几个拥有这种行为的地方
     * 当使用 object.something() 调用方法时，
     * Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
     * 也就是说，这些代码是等价的：
            p1.distance(&p2);
            (&p1).distance(&p2);
     */
    println!("Rect method area: {}", rect.area());

    let rect_1 = Rect { width: 30, height: 50 };
    let rect_2 = Rect { width: 10, height: 40 };
    let rect_3 = Rect { width: 60, height: 45 };
    println!("Can rect1 eat rect2? {}", rect_1.can_eat(&rect_2));
    println!("Can rect1 eat rect3? {}", rect_1.can_eat(&rect_3));

    // 关联函数
    let sq = Rect::square(30);
    println!("square: {:#?}", sq);

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
