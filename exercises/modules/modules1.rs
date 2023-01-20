// modules1.rs
// 执行 `rustlings hint modules1` 或在观察模式下使用 `hint` 子命令来获取提示。

mod sausage_factory {
    // 不要让模块外的任何人看到这个！
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
