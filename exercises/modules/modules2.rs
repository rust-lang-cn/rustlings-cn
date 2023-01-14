// modules2.rs
// 通过使用 'use' 和 'as' 关键字，你可以将模块路径带到作用域并为它们提供新名字。
// 修复这些 'use' 语句使代码可以编译。
// 执行 `rustlings hint modules2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

mod delicious_snacks {
    // TODO: 修复这些 use 语句
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
