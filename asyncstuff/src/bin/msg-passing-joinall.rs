use std::{future::Future, pin::{pin, Pin}};

use trpl;

// writing async fn is equivalent to writing a function which returns a future of the return type.
async fn send_values(tx: trpl::Sender<String>, vals: Vec<String>, delay: std::time::Duration) {
    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(delay).await;
    }
}

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let messages1 = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];
        let delay1 = std::time::Duration::from_millis(500);
        let send_fut1 = pin!(send_values(tx.clone(), messages1, delay1));

        let messages2 = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        let delay2 = std::time::Duration::from_millis(1500);
        let send_fut2 = pin!(send_values(tx, messages2, delay2));

        let receive_fut = pin!(async {
            while let Some(received) = rx.recv().await {
                println!("Got {received}");
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![
            send_fut1,
            send_fut2,
            receive_fut,
        ];
        trpl::join_all(futures).await;
    });
}
