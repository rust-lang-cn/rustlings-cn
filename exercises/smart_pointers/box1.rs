// box1.rs
//
// 在编译期，Rust 需要知道一个类型需要占用多少空间。这在递归类型中是有问题的，
// 递归类型可以有一个同类型的值作为类型的一部分。
// 为了解决这个问题，我们可以使用 `Box` - 一个智能指针，用于在堆上存储数据，
// 也允许我们包装一个递归类型。
//
// 我们要在这个练习实现的递归类型是 `cons list` - 一个常见于函数式编程语言的数据结构。
// cons list 的每个项包含两个元素：现在这个项的值和下一个项。最后一个项是一个叫做 `Nil` 的值。
//
// 步骤 1：在 enum 定义中使用一个 `Box` 使代码可以编译
// 步骤 2：创建一个空的和一个非空的 cons lists 取代 `todo!()`
//
// 注意：不能改变测试代码
//
// 执行 `rustlings hint box1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    todo!()
}

pub fn create_non_empty_list() -> List {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
