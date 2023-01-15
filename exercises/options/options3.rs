// options3.rs
// 执行 `rustlings hint options3` 或在观察模式下使用 `hint` 子命令来获取提示。

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // 修复问题但不能删除这行
}
