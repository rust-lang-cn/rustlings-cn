// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// 执行 `rustlings hint lifetimes2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
