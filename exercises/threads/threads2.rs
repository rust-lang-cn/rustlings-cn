// threads2.rs
// 执行 `rustlings hint threads2` 或在观察模式下使用 `hint` 子命令来获取提示。
// 基于上一个练习，我们想要所有线程完成它们的工作，但是这回
// 派生线程需要需要负责更新一个共享的值： JobStatus.jobs_completed

// I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: 在更新共享值之前，你还需要做点什么
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: 打印 JobStatus.jobs_completed 的值。你可以从输出中注意到有趣的东西吗？
        // 你必须要 'join' 所有的线程句柄吗？
        println!("jobs completed {}", ???);
    }
}
