// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// 执行 `rustlings hint tests3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!();
    }

    #[test]
    fn is_false_when_odd() {
        assert!();
    }
}
