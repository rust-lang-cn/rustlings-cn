// primitive_types3.rs
// 在 ??? 处创建一个至少包含100个元素的数组。
// 执行 `rustlings hint primitive_types3` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let a = [123; 456];

    if a.len() >= 100 {
        println!("喔，那是一个大数组");
    } else {
        println!("额，还不够我塞牙缝的。");
    }
}
