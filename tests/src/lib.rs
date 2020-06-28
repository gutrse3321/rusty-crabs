#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn it_works() {
        // 使用assert_eq! 测试判断运算与第二个参数结果值是否相等
        // 不相等将panic!
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 6 {
            Ok(())
        } else {
            Err(String::from("不等于啊，老弟"))
        }
    }

    #[test]
    fn another() {
        panic!("Make this test fail")
    }


    /// 使用assert!宏来检查结果
    /// assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。
    /// 需要向 assert! 宏提供一个求值为布尔值的参数。
    /// 如果值是 true，assert! 什么也不做，同时测试会通过。
    /// 如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。
    /// assert! 宏帮助我们检查代码是否以期望的方式运行。
    #[test]
    fn larger_can_holder_smaller() {

        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle{ width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger))
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
