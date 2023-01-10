// if1.rs
// 执行 `rustlings hint if1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
    // 完成这个函数使它返回一个较大值
    // 不要使用：
    // - 其它函数调用
    // - 额外的变量
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
