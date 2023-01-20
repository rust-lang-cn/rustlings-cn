// traits1.rs
// 是时候实现一些 trait 了！
//
// 你的任务是为类型 String` 实现 trait `AppendBar`。
//
// trait AppendBar 只有一个函数，它将 "Bar"
// 添加到任意实现了这个 trait 的对象中。
// 执行 `rustlings hint traits1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: 为类型 `String` 实现 `AppendBar`。
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
