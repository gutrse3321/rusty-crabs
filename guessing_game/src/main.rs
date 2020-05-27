use std::io;
//定义随机数生成器应事先的方法 trait
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //rand::thread_rng函数提供实际使用的随机数生成器，当前执行线程的本地环境中，并冲操作系统获取随机数种子
    //获取指定的1和101来获取一个1到100之间的数
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    //loop {} 无限循环
    loop {
        println!("Please input ur guess.");

        //let 变量不可变  let mut 可变
        //String utf-8可增长文本块 字符串
        //::new :: 语法表明 new 是 String 类型的一个 关联函数(associated function)
        //关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。java中的静态方法
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            //处理io::Result执行后可能存在的错误。go中的if err != nil { panic("Failed to read line") }
            .expect("Failed to read line");

        //rust允许用一个新值来隐藏变量之前的值 (shadow)
        //这个功能常用在需要转换值类型之类的场景，他允许我们复用变量的名字，而不是被迫创建两个不同的变量
        //将expect替换成match语句，从panic到错误处理
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("U guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win");
                break;
            },
        }
    }
}
