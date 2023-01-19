// traits2.rs
//
// 你的任务是为一个 string 类型的 vector 实现 `AppendBar` 特性。
//
// 为了实现这个特性，思考一下 '添加 "Bar"' 到一个 string 类型 vector 意味着什么。
//
// 这次没有样板代码，
// 你可以做到的！
// 执行 `rustlings hint traits2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: 为 string 类型的 vector 实现 `AppendBar` 特性。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
