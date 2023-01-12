// strings4.rs

// 好的，这里有一串值 -- 一些是 `String`，一些是 `&str`。
// 你的任务将使用值来调用两个函数中的某一个，基于你认为这个值是什么。
// 也就是，添加 `string_slice` 或者 `string` 到每行的括号前。
// 如果你对了，它将可以编译！
// 这次没有提示！

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
