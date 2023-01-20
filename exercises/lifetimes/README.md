# 生命周期

生命周期告诉编译器在给定情况下如何检查引用存活久到被认为有效。
比如生命周期说 "确保参数 'a' 和参数 'b' 存活一样久使得返回值有效"。

它们仅在借用（即引用）时是必需的，因为复制或移动的参数在其作用域内，无法在外部引用。
生命周期意味可以检查例如函数的调用代码，以确保其参数有效。生命周期是由它们的调用者决定的。

## 更多信息

- [使用生命周期检查引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [使用生命周期检查引用（中文版，译者注）](https://rustwiki.org/zh-CN/book/ch10-03-lifetime-syntax.html)
- [生命周期 (于《通过例子学 Rust》)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [生命周期 (于《通过例子学 Rust》)（中文版，译者注）](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime.html)
