// quiz2.rs
// 这是一个关于以下部分的测试：
// - 字符串 Strings
// - Vecs
// - 移动语义 Move semantics
// - 模块 Modules
// - 枚举 Enums

// 让我们构建一个小机器，以函数的形式。
// 我们将使用一列字符串和指令作为输入。
// 这些指令决定在如何操作字符串。它们可以是：
// - 使字符串为大写
// - 修整字符串
// - 添加特定次数的 "bar" 到字符串
// 具体形式如下：
// - 输入是一个 Vector，每个元素为一个长度为 2 的元组，，
//   元组的第一个元素是一个字符串，第二个元素是指令
// - 输出是一个字符串类型的 Vector。
// 这次没有提示！

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 补全函数签名！
    pub fn transformer(input: ???) -> ??? {
        // TODO: 完成 output 的声明！
        let mut output: ??? = vec![];
        for (string, command) in input.iter() {
            // TODO: 完成函数体。你可以做到的！
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: 我们需要导入什么使作用域中有 `transformer`？
    use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
