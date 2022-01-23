mod paramcli;
mod read;
mod uniq;
mod write;

use paramcli::*;
use read::*;
use std::sync::mpsc::channel;
use uniq::*;
use write::*;

pub fn traitement(p: &Paramcli) {
    //MPSC chanels
    let (to_compute, from_read) = channel();
    let (to_write, from_compute) = channel();

    let hread = if !p.fic.is_empty() {
        start_thread_read_file(to_compute, &p.fic)
    } else {
        start_thread_read_stdin(to_compute)
    };
    let hcompute = start_thread_uniq(from_read, to_write, p.count, p.not_case_sensitive);

    let hwrite = start_thread_write(from_compute);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if hcompute.join().is_err() {
        println!("Thread compute finished with error");
    }
    if hwrite.join().is_err() {
        println!("Thread compute finished with error");
    }
}

fn main() {
    let param = Paramcli::new();
    traitement(&param);
}
