// errors6.rs

// 使用能够捕获所有错误的类型，比如说 `Box<dyn error::Error>`，在库代码中是不推荐的，
// 其调用者可能想要基于错误的内容做决定，而不是将错误打印出来或向前传播。
// 这里，我们定义了一个自定义错误类型，使调用者在我们的函数返回错误时做判断成为可能。

// 执行 `rustlings hint errors6` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

use std::num::ParseIntError;

// 这是一个我们将会在 `parse_pos_nonzero()` 用到的自定义错误类型。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: 在这里添加另一个错误转换函数。
    // fn from_parseint...
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: 改变这里以返回一个适当的错误，而不是在
    // `parse()` 返回错误时发生 panic。
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // 我们不能构造一个 ParseIntError，所以只能进行模式匹配。
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
