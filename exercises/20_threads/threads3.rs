use std::{sync::{mpsc, Arc}, thread, time::Duration};

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    let arc_q = Arc::new(q);
    let arc_q1 = Arc::clone(&arc_q);
    let arc_q2 = Arc::clone(&arc_q);

    let tx2 = tx.clone();

    thread::spawn(move || {
        for val in &arc_q1.first_half {
            println!("Sending {val:?}");
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in &arc_q2.second_half {
            println!("Sending {val:?}");
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();
        let queue_length = queue.length;

        send_tx(queue, tx);

        let mut total_received: u32 = 0;
        for received in rx {
            println!("Got: {received}");
            total_received += 1;
        }

        println!("Number of received values: {total_received}");
        assert_eq!(total_received, queue_length);
    }
}
