// primitive_types6.rs
// 使用索引来访问 `numbers` 的第二个元素。
// 你可以在 ??? 处放置表达式来获取第二个元素以通过测试用例。
// 执行 `rustlings hint primitive_types6` 或在观察模式下使用 `hint` 子命令来获取提示。

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // 替换下面 ??? 为元组索引表达式。
    let second = numbers.1;

    assert_eq!(2, second,
        "这不是元组的第2个元素！")
}
