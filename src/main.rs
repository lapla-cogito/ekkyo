use std::str::FromStr as _;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = std::env::args().skip(1).fold("".to_owned(), |mut acc, s| {
        acc += &(s.to_owned() + " ");
        acc
    });

    let mut configs = vec![ekkyo::config::Config::from_str(config.trim_end()).unwrap()];

    if config.is_empty() {
        configs = vec![ekkyo::config::Config::default()];
    }

    let mut peers: Vec<ekkyo::peer::Peer> =
        configs.into_iter().map(ekkyo::peer::Peer::new).collect();

    for peer in &mut peers {
        peer.start();
    }

    let mut handles = Vec::new();

    for mut peer in peers {
        let handle = tokio::spawn(async move {
            loop {
                peer.next().await;
                tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.1)).await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
