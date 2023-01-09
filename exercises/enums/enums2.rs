// enums2.rs
// 执行 `rustlings hint enums2` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
