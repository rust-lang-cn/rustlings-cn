// from_str.rs
// 这个类似于 from_into.rs， 但是这次我们实现 `FromStr`
// 并返回错误而不是返回一个默认值。
// 另外，一旦实现 FromStr，你可以在字符串使用 `parse` 方法来生成一个实现者类型的对象。
// 阅读更多：https://doc.rust-lang.org/std/str/trait.FromStr.html
// 执行 `rustlings hint from_str` 或在观察模式下使用 `hint` 子命令来获取提示。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我们将使用错误类型于 `FromStr` 实现。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 空的输入字符串
    Empty,
    // 不正确的字段数
    BadLen,
    // 空的 name 字段
    NoName,
    // 包装来自 parse::<usize>() 的错误
    ParseInt(ParseIntError),
}

// I AM NOT DONE

// 步骤：
// 1. 如果提供的字符串长度为 0，应该返回一个错误
// 2. 根据逗号位置分割给定字符串
// 3. 分割结果应只有两个元素，否则返回一个错误
// 4. 提取分割结果的第一个元素并作为 name
// 5. 提取分割结果的另一个元素并将其类似 `"4".parse::<usize>()` 这样解析为 `usize`，作为 age
// 6. 如果在提取 name 和 age 时发生错误，应该返回一个错误
// 如果没有发生问题，则返回一个 Person 对象的 Result
//
// 顺便说一句：`Box<dyn Error>` 实现了 `From<&'_ str>`，着意味着如果你想要返回一个字符串错误信息，
// 你可以直接返回 `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
