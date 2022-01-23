use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_uniq(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    count: bool,
    not_case_sensitive: bool,
) -> JoinHandle<()> {
    spawn(move || {
        let mut buffer = Vec::new();
        for l in from_read {
            buffer.push(l);
        }
        //HashMap<String,u32>
        let mut uniq = HashMap::new();
        for l in buffer {
            let ll = if not_case_sensitive {
                l.to_lowercase()
            }else{
                l
            };
            let exist = uniq.get(&ll);
            match &exist {
                None => {
                    uniq.insert(ll, 1);
                }
                Some(x) => {
                    uniq.insert(ll, *x + 1);
                }
            }
        }
        for l in uniq {
            let s = if count {
                let mut s1 = String::new();
                s1.push_str(&l.0);
                s1.push(' ');
                s1.push_str(&format!("{}", l.1));
                s1
            } else {
                l.0
            };
            if to_write.send(s).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}
