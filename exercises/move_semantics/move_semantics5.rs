// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
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
