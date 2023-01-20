// iterators4.rs
// 执行 `rustlings hint iterators4` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // 完成这个函数以返回 num 的阶乘
    // 不要使用
    // - return
    // 尝试不要使用
    // - 命令式循环 (for, while)
    // - 额外的变量
    // 额外的挑战，不要使用：
    // - 递归
    // 执行 `rustlings hint iterators4` 获取提示。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
