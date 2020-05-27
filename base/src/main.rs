fn main() {
    /**
     * 常量constants
     */
    const MAX_POINTS: u32 = 100_000;
    println!("val: {}", MAX_POINTS);

    /**
     * 隐藏 shadowing
     */
    let var = 1;
    let var = var + 1;
    let var = var * 2;
    println!("value: {}", var);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("len: {}", spaces);

    // // 可变声明需要类型对应
    // let mut spaces = "     ";
    // spaces = spaces.len(); //panic: expected `&str`, found `usize`
    // println!("len: {}", spaces);

    /**
     * 数据类型 data type
     */
    // 需要加上类型声明 不然panic let ss = "42".parse().expect("fuck");
    //    |                        ^^ consider giving `ss` a type
    let ss: u32 = "42".parse().expect("fuck");
    println!("ss: {}", ss);

    // 整形 i8 16 32 64 128 isize      u8 16 32 64 128 usize
    // isize 和 usize 类型依赖运行程序的计算机架构 主要作为某些集合的索引
    // 64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的

    // 浮点数 f32 f64  f64为默认

    // 布尔值 bool

    // 字符
    let c: char = 'z';
    println!("c: {}", c);

    //复合类型
    // 元组 tuple   数组 array
    // 元组 tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //解构赋值
    let (var_1, var_2, var_3) = tup;
    println!("var1: {}, var2: {}, var3: {}", var_1, var_2, var_3);
    //索引赋值
    let var_4 = tup.0;
    let var_5 = tup.1;
    let var_6 = tup.2;
    println!("var4: {}, var5: {}, var6: {}", var_4, var_5, var_6);
    // 数组 array （vector类型 允许增长、缩短，array不可。go的切片）

}
