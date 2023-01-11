// move_semantics6.rs
// 执行 `rustlings hint move_semantics6` 或在观察模式下使用 `hint` 子命令来获取提示。
// 除了添加或删除引用之外你不能改变任何东西。

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// 应该不获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应该获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
