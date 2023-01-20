// vecs1.rs
// 你的任务是创建一个 `Vec`，它要与数组 `a` 的元素相同。
// 让我可以编译并通过测试！
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// 执行 `rustlings hint vecs1` 或在观察模式下使用 `hint` 子命令来获取提示。

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 一个平平无奇的数组
    let v = vec![10, 20, 30, 40]; // TODO: 使用 vectors 宏在这里定义你的 vector

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
