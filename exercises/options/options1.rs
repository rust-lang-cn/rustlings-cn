// options1.rs
// 执行 `rustlings hint options1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

// 这个函数返回冰箱中还有多少冰淇淋。
// 10PM 点前，这里有5根冰淇淋。
// 10PM 时，有人吃光它们了，所以就一根也没有了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 这里我们使用二十四小时制，所以 10PM 的值是22并且 12AM 的值是0
    // Option 输出需要很好地处理 time_of_day > 23 的情况。
    // TODO: 完成函数体 - 记住要返回一个 Option！
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: 修复这个测试。你要怎么获取 Option 内含的值？
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, 5);
    }
}
