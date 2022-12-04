use std::sync::mpsc::channel;
use std::thread;

// 示例：未销毁写端的数据生产者，引起读端的阻塞。
fn demo01() {
    let (tx, rx) = channel();
    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
        });
    }

    // 共享通道需要显式销毁
    // drop(tx);
    for j in rx.iter() {
        println!("{}", j);
    }
}

fn demo02() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(1i32).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
    });
    for x in rx.iter() {
        println!("{}", x);
    }
}
