// if2.rs

// 第一步： 使我可以编译！
// 第二步： 使 bar_for_fuzz 和 default_to_baz 可以测试通过！
// 执行 `rustlings hint if2` 或在观察模式下使用 `hint` 子命令来获取提示。

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// 不需要改变任何测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
