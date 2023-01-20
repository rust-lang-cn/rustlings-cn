// modules3.rs
// 你可以使用 'use' 关键字将任何地方的模块，特别是来自 Rust 标准库的模块路径
// 引入你的作用域。
// 从 std::time 模块引入 SystemTime 和 UNIX_EPOCH 。
// 如果你能用一行完成，就可以获得额外的风格积分！
// 执行 `rustlings hint modules3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

// TODO: 完成这个 use 语句
use ???

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
