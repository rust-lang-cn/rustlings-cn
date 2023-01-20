// lifetimes3.rs
//
// 生命周期在结构体持有引用时同样是必需的。
//
// 执行 `rustlings hint lifetimes3` 或在观察模式下使用 `hint` 子命令来获取提示。

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
