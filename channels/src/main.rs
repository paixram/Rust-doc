
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); // espera a que el mensaje sea recivido
    println!("Got {}!", received);

    // enviar multiples valores por un hilo
    //
    let (tz, rz) = mpsc::channel();
    
    let tzc = tz.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tzc.send(val).unwrap();
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
            tz.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rz {
        println!("Got: {}", received);
    } 
}
