# Traits

trait(特性) 是方法的集合。

数据类型可以实现 traits。为此，为数据类型定义了构成 trait 的方法。
比如，数据类型 `String` 实现了 `From<&str>` trait。这允许用户写出 `String::from("hello")`。

如此，traits 类似于 Java 的接口（interfaces）和 C++ 的抽象类（abstract classes）。

一些额外的 Rust 常见的 traits：
- `Clone` (`clone` 方法)
- `Display` (允许通过 `{}` 格式化显示)
- `Debug` (允许通过 `{:?}` 格式化显示)

因为 traits 指示了数据类型之间的共享行为，它们在写泛型时非常有用。


## 更多信息

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Traits（中文版，译者注）](https://rustwiki.org/zh-CN/book/ch10-02-traits.html)
