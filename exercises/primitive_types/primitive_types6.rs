// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// 执行 `rustlings hint primitive_types6` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = ???;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
