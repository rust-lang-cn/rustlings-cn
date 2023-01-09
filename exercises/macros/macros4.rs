// macros4.rs
// 执行 `rustlings hint macros4` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
