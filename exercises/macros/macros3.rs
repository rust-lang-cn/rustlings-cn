// macros3.rs
// Make me compile, without taking the macro out of the module!
// 执行 `rustlings hint macros3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
