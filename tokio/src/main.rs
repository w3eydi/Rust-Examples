use tokio::time::sleep;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use std::time::Duration;

#[derive(Debug)]
enum Message {
    Hello,
    World
}

async fn message_generator(mut channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(Duration::from_millis(100)).await,
            Err(_) => {
                eprintln!("Error, process didn't work");
                break;
            }
        }
    }
}

async fn message_catcher(mut channel: Receiver<Message>) {
    while let Some(msg) = channel.recv().await {
        println!("Writing a message: {:?}", msg);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel::<Message>(10);

    tokio::spawn(message_generator(tx));
    tokio::spawn(message_catcher(rx));

    sleep(Duration::from_millis(3000)).await;
}
