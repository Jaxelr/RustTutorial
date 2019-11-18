use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    //Initial basic example of message sending.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("First got: {}", received);

    //We can see the concurrency working here (thread sleep is the hint).
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Second got: {}", received);
    }

    //Multiple Producers, one consumer.
    let (tx3, rx3) = mpsc::channel();

    let tx4 = mpsc::Sender::clone(&tx3);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("multiple"),
            String::from("thread"),
        ];

        for val in vals {
            tx4.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx3 {
        println!("Third got: {}", received);
    }
}
