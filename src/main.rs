use std::time::Instant;

use talk::crypto::KeyChain;
use talk::net::SessionConnector;
use talk::link::rendezvous::Connector as RendezvousConnector;
use talk::link::rendezvous::Client;

    
#[tokio::main]
async fn main() {
    let keychain = KeyChain::random();
    let my_card = keychain.keycard();

    let server = "3.15.200.1:9000";

    let connector = RendezvousConnector::new(server, keychain, Default::default());

    let client = Client::new(server, Default::default());

    client.publish_card(my_card.clone(), Some(0)).await.unwrap();
    let shard = client.get_shard(0).await.unwrap();

    let other = shard.into_iter().filter(|keycard| keycard.identity() != my_card.identity()).next().unwrap();

    let connector = SessionConnector::new(connector);

    let mut session = connector.connect(other.identity()).await.unwrap();

    let bytes = 1_000_000;
    let message = (0..bytes/4).collect::<Vec<u32>>();

    let mut start = Instant::now();
    for i in 1.. {
        if i % 1000 == 0 {
            let end = Instant::now();
            println!("Throughput: {} BPS.", bytes as u128 / (end.duration_since(start).as_millis() / 1000));
            start = end;
        }
        session.send(&message).await.unwrap();
    }


}