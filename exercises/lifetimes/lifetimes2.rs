// lifetimes2.rs
//
// 所以，如果编译器只验证传递给标注了参数和返回类型的引用，我们需要改变什么？
//
// 执行 `rustlings hint lifetimes2` 或在观察模式下使用 `hint` 子命令来获取提示。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}
