// vecs2.rs
// 给定了一个由偶数组成的 Vec。你的任务是完成循环，
// 以将 Vec 中的每个数字都乘以2.
//
// 让我可以通过测试！
//
// 执行 `rustlings hint vecs2` 或在观察模式下使用 `hint` 子命令来获取提示。

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: 在这里将 Vec `v` 内的每个元素乘以2
        *i *= 2;
    }

    // 在这里，`v` 应当等于 [4, 8, 12, 16, 20]。
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| {
        // TODO: 像上面做的一样 - 但是不是改变 Vec，
        // 你可以直接返回新数字！
        num * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
