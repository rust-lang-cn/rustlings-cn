// clippy1.rs
// Clippy 工具是用于分析代码的 lint 集合，因此你可以发现常见的错误并改进 Rust 代码。
//
// 当有 clippy 警告时，这些练习会失败
// 检查 clippy 的输出的建议以解决练习。
// 执行 `rustlings hint clippy1` 或在观察模式下使用 `hint` 子命令来获取提示。

use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
