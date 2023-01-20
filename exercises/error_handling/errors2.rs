// errors2.rs
// 假设我们正在编写一个游戏，你可以通过代币购买物品。每个物品需要5个代币，
// 并且你在购买物品时需要消耗1个代币作为手续费。
// 游戏的玩家将会输入他们想要买的物品数量，然后 `total_cost` 函数将会计算代币的总数。
// 但是，由于玩家输入了数量，我们将其作为字符串获取 -- 并且
// 他们可能键入任何东西，而不仅是数字！

// 现在，这个函数完全没有处理错误（也没有适当处理成功的案例）。我们想要做的是：
// 如果我们在一个不是数字的字符串上调用 `parse` 函数，这个函数会返回一个 `ParseIntError`，
// 在这种情况下，我们希望直接从我们的函数返回那个错误并且不再尝试乘或加。

// 至少有两种方法可以实现，它们都是正确的 -- 但是有一个会更短！
// 执行 `rustlings hint errors2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
