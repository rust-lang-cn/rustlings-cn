// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// 执行 `rustlings hint lifetimes3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
