// 这个强大的 wrapper 提供了存储正整数的能力。
// 使用泛型重写它以支持包装任何类型。

// 执行 `rustlings hint generics2` 或在观察模式下使用 `hint` 子命令来获取提示。

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
