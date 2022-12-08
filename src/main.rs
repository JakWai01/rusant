mod sender;
mod receiver;

use sender::Sender;
use receiver::Receiver;

fn main() {
    println!("Hello World!");

    let sender = sender::SenderPipeline{};
    sender.send();

    let receiver = receiver::ReceiverPipeline{};
    receiver.receive();
}