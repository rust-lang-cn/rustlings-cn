// move_semantics4.rs
// 重构这份代码，不再将 `vec0` 传入 `fill_vec` 中，
// 而是在函数中创建 Vector 然后将其传回 main 函数。
// 执行 `rustlings hint move_semantics4` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` 不再使用 `vec: Vec<i32>` 作为参数
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
