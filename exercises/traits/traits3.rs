// traits3.rs
//
// 你的任务是给两个结构体实现 Licensed 特性，
// 并且让它们返回同样的信息，但是不要写两遍同样的函数。
//
// 思考一下你可以往 Licensed trait 中加什么。
// 执行 `rustlings hint traits3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 不要修改此行
impl Licensed for OtherSoftware {} // 不要修改此行

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
