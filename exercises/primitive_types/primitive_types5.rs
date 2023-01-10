// primitive_types5.rs
// 结构 `cat` 元组，使 println 可以正常工作。
// Destructure the `cat` tuple so that the println will work.
// 执行 `rustlings hint primitive_types5` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} 已经 {} 岁了。", name, age);
}
