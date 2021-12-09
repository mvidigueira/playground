use std::time::Instant;

use talk::crypto::KeyChain;
use talk::link::rendezvous::Listener as RendezvousListener;
use talk::link::rendezvous::Client;
use talk::net::SessionListener;

    
#[tokio::main]
async fn main() {
    let keychain = KeyChain::random();
    let my_card = keychain.keycard();

    let server = "13.59.89.246:9000";

    let listener = RendezvousListener::new(server, keychain, Default::default()).await;

    let client = Client::new(server, Default::default());

    client.publish_card(my_card.clone(), Some(0)).await.unwrap();

    let mut listener = SessionListener::new(listener);

    let bytes = 1_000_000;

    let mut start = Instant::now();
    for i in 1.. {
        let (_, mut session) = listener.accept().await;
        if i % 1000 == 0 {
            let end = Instant::now();
            println!("Throughput: {} BPS.", (1_000 * bytes * 1_000_000u64)/ end.duration_since(start).as_micros() as u64);
            start = end;
        }
        let message = session.receive::<Vec<u32>>().await.unwrap();
        session.send(&message).await.unwrap();

        session.end();
    }
}