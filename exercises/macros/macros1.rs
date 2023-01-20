// macros1.rs
// 执行 `rustlings hint macros1` 或在观察模式下使用 `hint` 子命令来获取提示。

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
