use std::{thread, time};

/* fn main() {
    for n in 1.. 1001 {
        let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(|| {
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
            });
            handles.push(handle);
        }

        while let Some(handle) = handles.pop() {
            handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
 */

//  fn main() {
//     for n in 1.. 1001 {
//         let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
//         let start = time::Instant::now();
//         for _m in 0..n {
//             let handle = thread::spawn(|| {
//                 let start = time::Instant::now();
//                 let pause = time::Duration::from_millis(20);
//                 while start.elapsed() < pause {
//                     // 会向操作系统发出一个信号，指示当前线程让出调度，让其他线程继续执行
//                     // thread::yield_now();
//                     // 跳过操作系统，直接向CPU发送自旋循环提示信号
//                     std::sync::atomic::spin_loop_hint();
//                 }
//             });
//             handles.push(handle);
//         }

//        /*  while let Some(handle) = handles.pop() {
//             handle.join();
//         } */
//        for handle in handles {
//         let _ = handle.join();
//        }

//         let finish = time::Instant::now();
//         println!("{}\t{:02?}", n, finish.duration_since(start));
//     }
// }

fn main() {
    let pause = time::Duration::from_millis(20);
    let handle1 = thread::spawn(move || {
        thread::sleep(pause);
    });
    let handle2 = thread::spawn(move|| {
        thread::sleep(pause);
    });
    let _ = handle1.join();
    let _ = handle2.join();
}
