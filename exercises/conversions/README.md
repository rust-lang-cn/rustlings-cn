# 类型转换

Rust 提供了多种方法将一个类型的值转为另一个类型。

类型转换的最简单方式是类型转换表达式。它用二元运算符 `as` 表示。
例如，`println!("{}", 1 + 1.0);` 不能编译，因为 `1` 是一个整数而 `1.0` 是一个浮点数。
然而，`println!("{}", 1 as f32 + 1.0)` 可以编译。
练习 [`using_as`](using_as.rs) 试图涵盖这一点。

Rust 还提供了实现后会便于类型转换的 trait。这些 traits 可以从这看到：
[`convert`](https://doc.rust-lang.org/std/convert/index.html)。
这些 traits 是：
- `From` 和 `Into` 包含于 [`from_into`](from_into.rs)
- `TryFrom` 和 `TryInto` 包含于 [`try_from_into`](try_from_into.rs)
- `AsRef` 和 `AsMut` 包含于 [`as_ref_mut`](as_ref_mut.rs)

此外，`std::str` 模块提供了一个叫做 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) 的 trait，它帮助转换字符串为目标类型，通过字符串的 `parse` 方法。
如果适当实现于给定类型 `Person`，那么 `let p: Person = "Mark,20".parse().unwrap()` 应该可以编译并不会导致 panic。

这些应该是 ***在标准库中*** 将类型转换为你想要的类型的主要方法。

## 更多信息

书本中没有直接涵盖这些内容，但是标准库中有非常好的文档。
- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)