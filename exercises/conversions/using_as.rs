// Rust 中的类型转换是通过 `as` 运算符完成的。
// 请注意 `as` 运算符不止用于类型转换，
// 它还有助于重命名导入。
//
// 目标是确保除法不会编译失败，并返回合适的类型。
// 执行 `rustlings hint using_as` 或在观察模式下使用 `hint` 子命令来获取提示。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
