enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let home = IpAddr { kind: IpAddrKind::V4, address: "127.0.0.1".to_string() };
    // let loopback = IpAddr { kind: IpAddrKind::V6, address: "::1".to_string() };
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6("::1".to_string());

    // Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。
    // 这个枚举是 Option<T>, 定义于标准库中
    // 可以不需要Option::Some去使用，直接Some
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let some_num = Some(5);
    let some_str = Some("a string");
    // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
    let absent_num: Option<i32> = None;

    // match
    println!("cents: {}", value_in_cents(Coin::Quarter));

    // if let
    let some_u8_val = Some(0u8);
    if let Some(0u8) = some_u8_val {
        println!("three");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("fucking penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 0u8, // default
    }
}
