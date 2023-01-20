// threads1.rs
// 执行 `rustlings hint threads1` 或在观察模式下使用 `hint` 子命令来获取提示。
// 这个程序应等待所有派生线程完成后再退出。

// I AM NOT DONE

use std::thread;
use std::time::Duration;


fn main() {

    let mut handles = vec![];
    for i in 0..10 {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        });
    }

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: 从 thread::spawn 返回了一个结构，你可以使用它吗？
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
}
