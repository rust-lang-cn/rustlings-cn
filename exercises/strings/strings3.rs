// strings3.rs
// 执行 `rustlings hint strings3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: 删除字符串两端的空格！
    ???
}

fn compose_me(input: &str) -> String {
    // TODO: 添加 " world!" 到字符串！有好几种方法可以做到！
    ???
}

fn replace_me(input: &str) -> String {
    // TODO: 将字符串中的 "cars" 替换为 "balloons"！
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
