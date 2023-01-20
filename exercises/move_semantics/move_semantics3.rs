// move_semantics3.rs
// 不添加新行使我可以编译 -- 只更改已存在的行！
// （也不需要在行中使用多个分号！）
// 执行 `rustlings hint move_semantics3` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
