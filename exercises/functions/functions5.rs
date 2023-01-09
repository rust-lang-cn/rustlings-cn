// functions5.rs
// 执行 `rustlings hint functions5` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let answer = square(3);
    println!("3的平方是 {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
