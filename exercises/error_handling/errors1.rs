// errors1.rs
// 这个函数在你传入一个空字符串时会拒绝生成要被打印在名牌上的字符串。
// 它可以变得更好，如果它能解释发生了什么问题的话，而不是直接返回一个 `None`。
// 幸好，Rust 有一个类似 `Option` 的结构用来解释错误情况。让我们使用它！
// 执行 `rustlings hint errors1` 或在观察模式下使用 `hint` 子命令来获取提示。

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 不允许空名字。
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
