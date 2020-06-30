#[cfg(test)]
mod tests {

    #[test]
    fn iterator_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        // next是Iterator trait实现者被要求定义的唯一方法。
        // next 一次返回迭代器中的一个项，封装在Some中
        // next每调用一次，即使用了一次v1_iter迭代器
        // 迭代完就返回None，如下
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        /*
            另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。
            iter 方法生成一个不可变引用的迭代器。
            如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
            类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter
         */
    }

    #[test]
    fn iter_num() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // sum会获取迭代器的所有权，在此之后无法使用v1_iter
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6)
    }

    /// 调用迭代器适配器map来创建一个新迭代器
    #[test]
    fn iter_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        // map调用生成的迭代器的结果收集到一个vector中
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4])
        /*
            调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
            因为 map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。
            这是一个展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的例子。
         */
    }

    /// 使用闭包获取环境
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {size: 10, style: "sneaker".to_string()},
            Shoe {size: 13, style: "sandal".to_string()},
            Shoe {size: 10, style: "boot".to_string()},
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        for shoe in in_my_size {
            println!("{}", shoe.style)
        }
        // assert_eq!(in_my_size, vec![
        //     Shoe {size: 13, style: "sandal".to_string()},
        //     Shoe {size: 10, style: "boot".to_string()},
        // ])
    }

    /// 实现Iterator trait
    /// 自定义迭代器
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {count: 0}
        }
    }

    impl Iterator for Counter {

        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn custom_iter() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn custom_iter_other_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum)
    }
}