/*use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

struct Broadcast<T> {
    senders: Vec<Sender<T>>,
}

impl<T: Clone> Broadcast<T> {

fn new() -> Self {
        Broadcast {
            senders: Vec::new(),
        }
    }

fn register(&mut self) -> Receiver<T> {
        let (sender, receiver) = channel();
        self.senders.push(sender);
        receiver
    }

fn broadcast(&self, msg: T) {
        for sender in &self.senders {
            let _ = sender.send(msg.clone());
        }
    }
}

fn main() {
    let broadcast = Arc::new(Mutex::new(Broadcast::new()));

    let mut threads = Vec::new();

    for i in 0..10 {
        let broadcast_clone = Arc::clone(&broadcast);
        let handle = std::thread::spawn(move || {
            let receiver = broadcast_clone.lock().unwrap().register();
            loop {
                let msg = receiver.recv().unwrap();
                println!("Thread {} received message: {:?}", i, msg);
            }
        });
        threads.push(handle);
    }

    // simulate broadcasting messages
    for i in 0..10 {
        broadcast.lock().unwrap().broadcast(format!("message {}", i));
    }

    // wait for all threads to finish
    for handle in threads {
        handle.join().unwrap();
    }
}*/
