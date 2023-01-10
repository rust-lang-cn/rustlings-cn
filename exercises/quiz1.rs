// quiz1.rs
// 这是一个关于以下章节的随堂测试：
// - 变量
// - 函数
// - If

// 玛丽正在买苹果。苹果的价格通过以下方式计算：
// - 一个苹果消耗 2 锈币。
// - 如果玛丽买了超过 40 个苹果，每个苹果就只用 1 锈币！
// 写一个根据订单的苹果个数计算价格的函数。
// 这次没有提示！


// I AM NOT DONE

// 将你的函数放在这里！
// fn calculate_price_of_apples {

// 不要修改这个函数
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
