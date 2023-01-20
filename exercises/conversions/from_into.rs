// From trait 被用于 值-到-值 的转换。
// 如果 From 被一个类型正确地实现，则 Info trait 应相反地工作。
// 阅读更多：https://doc.rust-lang.org/std/convert/trait.From.html
// 执行 `rustlings hint from_into` 或在观察模式下使用 `hint` 子命令来获取提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 我们实现 Default trait 用于当给定的字符串无法转换为一个 Person 对象时的回退。
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 你的任务是完成这个实现
// 以便于编译行 `let p = Person::from("Mark,20")`。
// 请注意，你需要将 age 部分解析为一个 `usize`，方法类似于 `"4".parse::<usize>()`。
// 它的结果需要被妥善处理。
//
// 步骤：
// 1. 如果提供的字符串长度为 0，则返回默认的 Person
// 2. 在字符串的逗号处分离
// 3. 提取分离结果的第一部分作为 name
// 4. 如果 name 是空的，返回默认的 Person
// 5. 提取分离结果的另一部分并将其解析为一个 `usize` 作为 age
// 如果当解析 age 时发生了错误，那么返回默认版本的 Person
// 不然， 利用结果返回一个实例化的 Person 对象。

// I AM NOT DONE

impl From<&str> for Person {
    fn from(s: &str) -> Person {
    }
}

fn main() {
    // 使用 `from` 函数
    let p1 = Person::from("Mark,20");
    // 由于 From 被 Person 实现了，我们应该可以使用 Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
