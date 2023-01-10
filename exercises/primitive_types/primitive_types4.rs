// primitive_types4.rs
// 在 ??? 处获取一个数组的切片来使测试可以通过。
// 执行 `rustlings hint primitive_types4` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = ???

    assert_eq!([2, 3, 4], nice_slice)
}
