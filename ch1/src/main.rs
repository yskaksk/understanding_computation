fn main() {
    // 基本データ
    let b = (true && false) || true;
    println!("(true && false) || true = {}", b);

    let n = (3 + 3) * (14 / 2);
    println!("(3 + 3) * (14 / 2) = {}", n);

    let s = "hello".to_string() + "world";
    println!("\"hello\".to_string() + \"world\" = {}", s);

    let ss = &String::from("hello world")[6..7];
    println!("&String::from(\"hello world\")[6..7] = {}", ss);

    // シンボルはない（多分）

    // データ構造
    let mut numbers = vec!["zero", "one", "two"];
    println!("vec![\"zero\", \"one\", \"two\"] = {:?}", numbers);
    println!("numbers[1] = {:?}", numbers[1]);
    numbers.push("three");
    numbers.push("four");
    println!("numbers.push(\"three\")");
    println!("numbers.push(\"four\")");
    println!("numbers = {:?}", numbers);

    use std::collections::HashMap;
    let mut fruit = HashMap::new();
    fruit.insert("a", "apple");
    fruit.insert("b", "banana");
    fruit.insert("c", "coconut");
    println!("fruit : {:?}", fruit);
    println!("fruit[\"a\"] = {:?}", fruit["a"]);

    // Proc（無名関数）
    let multiply = |x: i32, y: i32| { x * y};
    println!("let multiply = |x: i32, y: i32| {{ x * y}};");
    println!("multiply(3, 4) = {}", multiply(3, 4));

    // 制御フロー（略）

    // オブジェクトとメソッド（略）

    // クラスとモジュール
    //
    // クラスはstructで代替する
    // 継承はないので、Enumで代替する
}
