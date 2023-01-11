// structs1.rs
// 解决所有的 TODOs 以让测试通过！
// 执行 `rustlings hint structs1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

struct ColorClassicStruct {
    // TODO: 这边有东西
}

struct ColorTupleStruct(/* TODO: 这边有东西 */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: 实例化一个经典的 c 结构！
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构！
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个类单元结构体！
        // let unit_like_struct =
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
