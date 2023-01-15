// options2.rs
// 执行 `rustlings hint options2` 或在观察模式下使用 `hint` 子命令来获取提示。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 将这个改为一个值为 "Some" 类型的 if let 语句
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: 写为 while let 语句 - 记住 vector.pop 也添加了一层 Option<T>
        // 你可以堆叠 `Option<T>` 到 while let 和 if let 中
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
