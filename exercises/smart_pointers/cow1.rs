// cow1.rs

// 这个练习探索 Cow，或者说 Clone-On-Write（写时克隆） 类型。
// Cow 是一个 clone-on-write 智能指针
// 它可以封装并提供一个借用数据的不可变访问，并当需要修改或所有权时克隆数据。
// 该类型旨在通过 Borrow trait 处理一般借用数据。

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果还没有持有，克隆到一个 vector。
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn main() {
    // 没有 clone 发生，因为 `input` 不需要被改变。
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Borrowed(_) => println!("I borrowed the slice!"),
        _ => panic!("expected borrowed value"),
    }

    // 发生了 clone， 因为 `input` 需要被改变。
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I modified the slice and now own it!"),
        _ => panic!("expected owned value"),
    }

    // 没有 clone 发生，因为已经持有了 `input`。
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    match abs_all(&mut input) {
        // TODO
        Cow::Owned(_) => println!("I own this slice!"),
        _ => panic!("expected borrowed value"),
    }
}
