fn main() {
    // 调用from时，请求其所需的内存
    let mut s = String::from("hello"); // s有效
    s.push_str(", tomonori");
    println!("{}", s);
} // 作用域已结束
  // s 不再有效
// 内存在拥有它的变量离开作用域后就被自动释放
// rust调用特殊函数 drop，在 } 自动调用
