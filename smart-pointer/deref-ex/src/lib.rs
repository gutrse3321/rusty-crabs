#[cfg(test)]
mod tests {
    use std::ops::Deref;

    /// 通过解应用运算符追踪指针的值
    #[test]
    fn it_works() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        /*
            如果希望对 y 的值做出断言，
            必须使用 *y 来追踪引用所指向的值（也就是 解引用）。
            一旦解引用了 y，
            就可以访问 y 所指向的整型值并可以与 5 做比较
         */
        assert_eq!(5, *y);
    }

    /// 像引用一样使用Box<T>
    #[test]
    fn box_works() {
        let x = 5;
        // 解引用运算符也一样能工作
        // 唯一不同点
        // 将y设置为指向x值的box实例，而不是指向x值的引用
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    /// 自定义只能指针
    struct MyBox<T>(T);

    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    /// 为了启用 * 运算符的解引用功能，需要实现 Deref trait
    impl <T> Deref for MyBox<T> {
        // type Target = T; 语法定义了用于此 trait 的关联类型
        type Target = T;

        // deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值，
        // 并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力
        fn deref(&self) -> &T {
            // 返回引用，则所有权还在
            &self.0
        }
    }

    #[test]
    fn my_box_works() {
        let x = 5;
        // 解引用运算符也一样能工作
        // 唯一不同点
        // 将y设置为指向x值的box实例，而不是指向x值的引用
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *(y.deref()));
        // Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作，
        // 如此我们便不用担心是否还需手动调用 deref 方法了。
        assert_eq!(5, *y);
    }

    /// 函数和方法的隐式解引用强制多态(deref coercions)
    /// 解引用强制多态的加入使得 Rust 程序员
    /// 编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。
    /// 这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。
    ///
    /// hello("Rust")
    /// 可以使用字符串 slice 作为参数调用 hello 函数。
    ///
    /// let b = MyBox::new("Rust".to_string());
    /// hello(&b)
    /// 解引用强制多态使得用 MyBox<String> 类型值的引用调用 hello 成为可能
    fn hello(name: &str) {
        println!("hello, {}", name)
    }
}
