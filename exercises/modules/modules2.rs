// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// 执行 `rustlings hint modules2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

mod delicious_snacks {
    // TODO: Fix these use statements
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
