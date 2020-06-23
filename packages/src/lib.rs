mod front_of_house;
// 使用 use 将模块引入作用域
// 绝对路径，相对路径: use front_of_house::hosting;
pub use crate::front_of_house::hosting;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        /**
         * 使用super起始的相对路径
         * 从父目录开始调用函数
         *
         * fix函数在back_of_house模块中，使用super进入back_of_house模块
         * 也就是crate根
         */
        super::serve_order();
        /**
         * 当前mod下的相对路径
         */
        self::cook_order();
    }

    fn cook_order() {

    }

    // 结构体定义的前面使用了 pub ，这个结构体会变成公有的，
    // 但是这个结构体的字段仍然是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {

        // 关联函数
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonal_fruit: "peaches".to_string()
            }
        }
    }

    // 枚举则和结构体不同，共有的，枚举成员也是共有的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn serve_order() {

}

pub fn eat_at_restaurant() {
    // // absolute path
    // crate::front_of_house::hosting::add_to_wait_list();
    //
    // // relative path
    // front_of_house::hosting::add_to_wait_list();
    // 使用use引入作用域的模块
    // 类似文件系统中的创建软连接
    hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = "Wheat".to_string();
    println!("{} toast please", meal.toast);

    let order_1 = back_of_house::Appetizer::Soup;
}

/// 别名
/// as 关键字
use std::fmt::Result;
use std::io::Result as IoResult;

/// pub use 重导出名称
/// 通过 pub use，现在可以通过新路径
/// hosting::add_to_waitlist 来调用 add_to_waitlist 函数。
/// 如果没有指定 pub use，eat_at_restaurant
/// 函数可以在其作用域中调用 hosting::add_to_waitlist，
/// 但外部代码则不允许使用这个新路径
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/// 嵌套路径
/// use std::cmp::Ordering;
/// use std::io;
/// 可以简化
use std::{cmp::Ordering, io};
/// 儿子的儿子
/// use std::cmp;
/// use std::cmp::Ord;
use std::cmp::{self, Ord};

/// global运算符 *
/// 将所有公有定义引入作用域
use std::collections::*;
