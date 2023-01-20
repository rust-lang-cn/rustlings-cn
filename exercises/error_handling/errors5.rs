// errors5.rs

// 这个程序使用基于 errors4 改动的代码。

// 这个练习使用了一些我们后面才会了解的概念，比如 `Box` 和 `From` trait。
// 现在理解它们与否是不重要的，不过如果你喜欢的话，可以提前阅读。
// 现在，想象 `Box<dyn ...>` 类型为一个 "任意我想要的东西，其满足？？？" 类型，
// 考虑到 Rust 通常的运行时安全标准，这应该会让你觉得有些宽容！

// 简而言之，这个关于 box 的特别用法是用于在你想拥有一个值并只关心它是一个实现了特定 trait 的类型时使用的。
// 为此，这个 Box 被声明为 Box<dyn Trait> 类型，其中 Trait 是编译器在context中使用的任意值中查找到的 trait。
// 在这个练习中，context是可以被作为 Result 返回的潜在错误。

// 我们要使用什么来描述两种错误？换而言之，有一个 trait 是两种错误都实现了的吗？

// 执行 `rustlings hint errors5` 或在观察模式下使用 `hint` 子命令来获取提示。

use std::error;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// TODO: 更新 `main()` 的返回值类型以使编译通过。
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// 不要改变这行以下的任何东西。

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// 这是必须的，以便 `CreationError` 可以实现 `error::Error`。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
