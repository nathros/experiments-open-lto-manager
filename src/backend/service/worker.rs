use std::sync::mpsc::{self, Receiver};
use std::sync::{LazyLock, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub static WK: LazyLock<Mutex<Vec<Worker>>> = LazyLock::new(|| Mutex::new(vec![]));

pub struct Worker {
    pub rx_message: Receiver<String>,
    pub thread: JoinHandle<isize>,
}

fn worker_start() {
    let (tx, rx) = mpsc::channel();

    let w = Worker {
        rx_message: rx,
        thread: thread::spawn(move || {
            (0..1000).for_each(|i: i32| {
                thread::sleep(Duration::from_secs(1));
                let val = i.to_string();
                tx.send(val).unwrap();
            });

            return 0;
        }),
    };

    //WK.lock().
}
