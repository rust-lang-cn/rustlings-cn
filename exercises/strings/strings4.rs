// strings4.rs

// 好的，这里有一串值 -- 一些是 `String`，一些是 `&str`。
// 你的任务将使用值来调用两个函数中的某一个，基于你认为这个值是什么。
// 也就是，添加 `string_slice` 或者 `string` 到每行的括号前。
// 如果你对了，它将可以编译！
// 这次没有提示！

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    ???("blue");
    ???("red".to_string());
    ???(String::from("hi"));
    ???("rust is fun!".to_owned());
    ???("nice weather".into());
    ???(format!("Interpolation {}", "Station"));
    ???(&String::from("abc")[0..1]);
    ???("  hello there ".trim());
    ???("Happy Monday!".to_string().replace("Mon", "Tues"));
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
