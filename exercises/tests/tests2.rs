// tests2.rs
// 这个测试有一个问题 -- 使测试可以编译！使测试通过！使测试失败！
// 执行 `rustlings hint tests2` 或在观察模式下使用 `hint` 子命令来获取提示。

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1==1, true);
    }
}
