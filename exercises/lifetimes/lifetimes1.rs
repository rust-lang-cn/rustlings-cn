// lifetimes1.rs
//
// Rust 编译器需要知道如何确认提供的引用是有效的，
// 因此它可以让程序员知道一个引用是否有在使用前超出作用域的风险。
// 记住，引用是借用，并且没有拥有它们自己的数据。
// 如果它们的所有者超出作用域呢？
//
// 执行 `rustlings hint lifetimes1` 或在观察模式下使用 `hint` 子命令来获取提示。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
