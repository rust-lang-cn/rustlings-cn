// move_semantics6.rs
// 执行 `rustlings hint move_semantics6` 或在观察模式下使用 `hint` 子命令来获取提示。
// You can't change anything except adding or removing references.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();

    println!("{}", data);
}
