// crossbeam提供Unix Select的实现
use channel::select;
use crossbeam::channel;
use std::thread;

// 采用CSP计算方式，通过多个协作线程计算fib数
fn fib_parallel(fib: channel::Sender<u32>, quit: channel::Receiver<()>) {
    let (mut x, mut y) = (0, 1);
    loop {
        select! {
            send(fib, x) -> _ => {
                let t = x;
                x=y;
                y=t+y;
            }
            recv(quit) -> _ => {
                println!("end of calculation");
                return;
            }
        }
    }
}

pub fn demo03() {
    let (fib_tx, fib_rx) = channel::bounded(0);  // 有界通道，采取0容量
    let (quit_tx, quit_rx) = channel::bounded(0); 
    thread::spawn(move || {
        for _ in 0..10 {
            println!("{}", fib_rx.recv().unwrap());
        }
        quit_tx.send(()).unwrap();
    });
    fib_parallel(fib_tx, quit_rx);
}