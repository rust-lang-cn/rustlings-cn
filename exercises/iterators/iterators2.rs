// iterators2.rs
// 在这个练习，你可以学习到一些迭代器能提供的独有特性。
// 跟着步骤完成练习。
// 执行 `rustlings hint iterators2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

// 步骤 1.
// 完成 `capitalize_first` 函数。
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => ???,
    }
}

// 步骤 2。
// 应用 `capitalize_first` 到字符串切片的切片。
// 返回一个字符串 vector。
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    vec![]
}

// 步骤 3。
// 再次应用 `capitalize_first` 到字符串切片的切片。
// 返回单独一个字符串。
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
