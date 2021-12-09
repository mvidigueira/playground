use std::time::Instant;

use talk::crypto::KeyChain;
use talk::net::Connector;
use talk::net::SessionConnector;
use talk::link::rendezvous::Connector as RendezvousConnector;
use talk::link::rendezvous::Client;

    
#[tokio::main]
async fn main() {
    let keychain = KeyChain::random();
    let my_card = keychain.keycard();

    let server = "13.59.89.246:9000";

    let connector = RendezvousConnector::new(server, keychain, Default::default());

    let client = Client::new(server, Default::default());

    let shard = client.get_shard(0).await.unwrap();

    let other = shard.into_iter().filter(|keycard| keycard.identity() != my_card.identity()).next().unwrap();

    let connector = SessionConnector::new(connector);

    let bytes: u64 = 1_000_000;
    let message = (0..bytes as u32/4).collect::<Vec<u32>>();

    let mut start = Instant::now();
    for i in 1.. {
        let mut session = connector.connect(other.identity()).await.unwrap();
        if i % 1000 == 0 {
            let end = Instant::now();
            println!("Throughput: {} BPS.", (1_000 * bytes * 1_000_000u64)/ end.duration_since(start).as_micros() as u64);
            start = end;
        }
        session.send(&message).await.unwrap();
        session.end();
    }
}