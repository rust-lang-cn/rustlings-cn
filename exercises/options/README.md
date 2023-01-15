# Options

Option 类型表明一个可选值：每个 Option 要么是一个包含值的 Some，要么是什么都没有的 None。
Option 类型在 Rust 编码中非常常见，因为它们常用于：
- 初始化值
- 输出值未在函数规范范围内时作为返回值
- 作为报告其它简单错误的返回值，当错误时返回 None
- Option 结构的字段
- 可以被借出或 "取出(taken)" 的结构体字段
- 可选的函数参数
- 可空的指针
- 把事情从困境中解救出来

## 更多信息

- [Option 枚举形式](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 枚举形式（中文版，译者注）](https://rustwiki.org/zh-CN/book/ch10-01-syntax.html#%E6%9E%9A%E4%B8%BE%E5%AE%9A%E4%B9%89%E4%B8%AD%E7%9A%84%E6%B3%9B%E5%9E%8B)
- [Option 模块文档](https://doc.rust-lang.org/std/option/)
- [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
