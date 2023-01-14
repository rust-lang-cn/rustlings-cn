// hashmaps1.rs
// 需要定义一个哈希表格式的水果篮子。
// 它的键表明水果的名字，值表明篮子里有多少个这种水果。
// 你需要放入至少三种不同类型的水果（例如 apple, banana, mango）到篮子中
// 并且水果总数应至少为5。
//
// 执行 `rustlings hint hashmaps1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = // TODO: 在这里声明你的哈希表。

    // 已经给你两根香蕉了 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 在这里将更多水果放入你的篮子中。

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
