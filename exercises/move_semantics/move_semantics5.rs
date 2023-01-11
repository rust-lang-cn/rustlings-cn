// move_semantics5.rs
// 在只重新排序 `main()` 内代码行的情况下使我可以编译，
// 但是不要增加、修改或删除任何行。
// 执行 `rustlings hint move_semantics5` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
