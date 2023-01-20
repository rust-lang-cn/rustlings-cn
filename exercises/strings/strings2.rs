// strings2.rs
// 不改变函数签名使我可以编译！
// 执行 `rustlings hint strings2` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
