// strings1.rs
// 不改变函数签名使我可以编译！
// 执行 `rustlings hint strings1` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
    // or
    // String::from("blue")
}
