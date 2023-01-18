// errors3.rs
// 当我们尝试使用在上个练习中完成的函数 `total_cost` 时，遇到了问题。
// 它并没有工作！为什么不？我们应该怎么修复它？
// 执行 `rustlings hint errors3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
