use add_lib::add_one;

fn main() {
    // 调用的另外一个本地crate的函数
    println!("{}", add_one(10));
}
