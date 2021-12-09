use std::time::Instant;

use talk::crypto::KeyChain;
use talk::net::SessionConnector;
use talk::link::rendezvous::Listener as RendezvousListener;
use talk::link::rendezvous::Client;
use talk::net::SessionListener;

    
#[tokio::main]
async fn main() {
    let keychain = KeyChain::random();
    let my_card = keychain.keycard();

    let server = "3.15.200.1:9000";

    let listener = RendezvousListener::new(server, keychain, Default::default()).await;

    let client = Client::new(server, Default::default());

    client.publish_card(my_card.clone(), Some(0)).await.unwrap();

    let mut listener = SessionListener::new(listener);

    let (_, mut session) = listener.accept().await;

    let bytes = 1_000_000;

    let mut start = Instant::now();
    for i in 1.. {
        if i % 1000 == 0 {
            let end = Instant::now();
            println!("Throughput: {} BPS.", bytes as u128 / (end.duration_since(start).as_millis() / 1000));
            start = end;
        }
        let _message = session.receive::<Vec<u32>>().await.unwrap();
    }
}