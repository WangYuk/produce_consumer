use std::sync::{mpsc, Arc, Mutex};
use std::thread;


fn producer(id: usize, tx: mpsc::Sender<(usize, usize)>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for i in (id - 1) * 100 + 1..id * 100 {
            tx.send((id, i)).unwrap();
        }
    })
}

fn consumer(
    id: usize,
    shared_rx: Arc<Mutex<mpsc::Receiver<(usize, usize)>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let rx = shared_rx.lock().unwrap();
        if let Ok((i, n)) = rx.recv() {
            println!("consumer {} get {} from producer {}", id, n, i);
        } else {
            return ();
        }
    })
}

fn main() {
    let mut producer_v = vec![];
    let mut consumer_v = vec![];
    let (tx, rx) = mpsc::channel();
    let shared_rx = Arc::new(Mutex::new(rx));

    for i in 1..4 {
        let hdl = consumer(i, shared_rx.clone());
        consumer_v.push(hdl);
    }

    for i in 1..3 {
        let hdl = producer(i, tx.clone());
        producer_v.push(hdl);
    }

    drop(tx);

    for hdl in producer_v {
        hdl.join().unwrap();
    }

    for hdl in consumer_v {
        hdl.join().unwrap();
    }
}
