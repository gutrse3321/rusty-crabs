use std::fmt::Pointer;

fn main() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("&v[2]: {}", third);

    match v.get(2) {
        Some(third) => println!("match third ele: {}", third),
        None => println!("no match"),
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // 使用枚举存多种类型
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text("yellow".to_string()),
        SpreadSheetCell::Float(10.10),
    ];
    let int = &row[0];
    match int {
        SpreadSheetCell::Int(i) => {
            println!("int: {}", i);
        },
        SpreadSheetCell::Float(_) => {},
        SpreadSheetCell::Text(_) => {},
    }

    func_hash_map();
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

use std::collections::HashMap;

fn func_hash_map() {
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);

    // 使用队伍列表和分数列表创建
    let teams = vec!["blue".to_string(), "yellow".to_string()];
    let ini_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(ini_scores.iter()).collect();

    /// 覆盖值
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("blue".to_string(), 100);
    // 只有在键无值时插入
    scores.entry("yellow".to_string()).or_insert(1919);

    println!("print scores hash map:");
    for (key, val) in scores {
        println!("key:{}, val:{}", key, val);
    }
}
