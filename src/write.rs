use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_write(from_compute: Receiver<String>) -> JoinHandle<()> {
    spawn(move || {
        for d in from_compute {
            println!("{}", d);
        }
    })
}
