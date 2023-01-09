// functions4.rs
// 执行 `rustlings hint functions4` 或在观察模式下使用 `hint` 子命令来获取提示。

// 这个商店正在打折，如果价格是偶数，你可以获得 10 Rust元的优惠，
// 但是如果价格是奇数，则优惠 3 Rust元。
// （不要担心函数体本身，我们现在只对函数签名感兴趣。
// 如果有的话，这是一个提前了解未来练习的好方法。）

fn main() {
    let original_price = 51;
    println!("你的售价为 {}", sale_price(original_price));
}

fn sale_price(price: u32) -> u32 {
    if is_even(price as i32) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
