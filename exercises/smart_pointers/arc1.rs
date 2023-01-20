// arc1.rs
// 在这个练习中，我们给定一个 u32 类型的 Vec，名为 "numbers"，内含从 0 到 99 的值 -- [ 0, 1, 2, ..., 98, 99 ]
// 我们想同时在 8 个不同的线程中使用这个数字集合。
// 每个线程将通过 offset 获取 8 个值的总和。
// 第一个线程 (offset 0)，将求和 0, 8, 16, ...
// 第二个线程 (offset 1)，将求和 1, 9, 17, ...
// 第三个线程 (offset 2)，将求和 2, 10, 18, ...
// ...
// 第八个线程 (offset 2)，将求和 7, 15, 23, ...

// 由于我们在使用线程，我们的值需要是线程安全的。因此，
// 我们使用 Arc。我们需要改变两个 TODO 处的内容。


// 通过填充第一个 TODO 注释处的 `shared_numbers` 的值使代码编译，
// 并为第二个 TODO 注释处的 `child_numbers` 创建一个初始化绑定。
// 尝试不要创建 `numbers` Vec 的任何副本！
// 执行 `rustlings hint arc1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = // TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
