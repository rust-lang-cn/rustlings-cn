// functions3.rs
// 执行 `rustlings hint functions3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

fn main() {
    call_me();
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
