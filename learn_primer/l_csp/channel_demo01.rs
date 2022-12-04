use std::thread;
use std::sync::mpsc::{channel, sync_channel};


// 单生产者使用 channel
fn demo01() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 10);
}

// 多生产者使用 MPSC channel
fn demo02() {
    let (tx, rx) = channel();
    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
        });
    }

    for i in 0..5 {
        let recv_data = rx.recv().unwrap();
        assert!(5 > recv_data );
        println!("{}", recv_data);
    }
}

// 同步通道
fn demo03() {
    let (tx, rx) = sync_channel(1);
    tx.send(1).unwrap();

    // 子线程在sync_channel可以继续添加元素之前，一直被阻塞。
    // 不要使主线程被阻塞。
    thread::spawn(move || {
        tx.send(2).unwrap();
    });

    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);

}
