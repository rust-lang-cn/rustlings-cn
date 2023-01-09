// clippy2.rs
// 执行 `rustlings hint clippy2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option {
        res += x;
    }
    println!("{}", res);
}
