// tests3.rs
// 这个测试不在测试我们的函数 -- 让它以测试通过的方法进行测试。
// 然后编写第二个测试来测试当我们调用 `is_even(5)` 时是否会获得我们想要的结果。
// 执行 `rustlings hint tests3` 或在观察模式下使用 `hint` 子命令来获取提示。

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(6));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}
