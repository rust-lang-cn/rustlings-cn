// modules1.rs
// 执行 `rustlings hint modules1` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
