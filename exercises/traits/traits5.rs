// traits5.rs
//
// 你的任务是替换 '??' 部分使得代码可以编译。
// 不要修改此外的任何行。
// 执行 `rustlings hint traits5` 或在观察模式下使用 `hint` 子命令来获取提示。

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 你只能修改下一行
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
