#[cfg(test)]
mod tests {
    use crate::tests::List::{Cons, Nil};

    /// Box<T>，用于在堆上分配值
    /// Box<T> 类型是一个智能指针，
    /// 因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待。
    /// 当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 的实现，
    /// box 所指向的堆数据也会被清除
    #[test]
    fn it_works() {
        // 使用box在堆上存了一个int32的值
        let b = Box::new(5);
        println!("b = {}", b)
    }

    /// 创建递归类型
    /// 使用Box<T>给递归类型一个已知的大小
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    #[test]
    fn box_iter() {
        /*
            Cons 成员将会需要一个 i32 的大小加上储存 box 指针数据的空间。
            Nil 成员不储存值，所以它比 Cons 成员需要更少的空间。
            现在我们知道了任何 List 值最多需要一个 i32 加上 box 指针数据的大小。
            通过使用 box ，打破了这无限递归的连锁，
            这样编译器就能够计算出储存 List 值需要的大小了。
         */
        // 如果直接用枚举List存的话，将会遇到
        // recursive type `List` has infinite size
        // 无限大的大小错误
        let list = Cons(1,
                        Box::new(Cons(2,
                            Box::new(Cons(3,
                                Box::new(Cons(4,
                                    Box::new(Nil))))))));
    }
}
