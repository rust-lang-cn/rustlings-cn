// functions2.rs
// 执行 `rustlings hint functions2` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("铛铛！呼叫号码 {}", i + 1);
    }
}
