#[derive(Debug)]
pub struct Peer {
    state: crate::state::State,
    queue: crate::queue::Queue,
    config: crate::config::Config,
    connection: Option<crate::connection::Connection>,
}

impl Peer {
    pub fn new(config: crate::config::Config) -> Self {
        Peer {
            state: crate::state::State::Idle,
            queue: crate::queue::Queue::new(),
            config,
            connection: None,
        }
    }

    #[tracing::instrument]
    pub fn start(&mut self) {
        tracing::info!("started peer");
        self.queue.enqueue(crate::event::Event::Start);
    }

    #[tracing::instrument]
    pub async fn next(&mut self) {
        if let Some(event) = self.queue.dequeue() {
            tracing::info!("processing event: {:?}", event);

            match self.state {
                crate::state::State::Idle => {
                    if event == crate::event::Event::Start {
                        self.connection = crate::connection::Connection::connect(&self.config)
                            .await
                            .ok();
                        if self.connection.is_some() {
                            self.queue.enqueue(crate::event::Event::TcpConnect);
                        } else {
                            panic!("failed to connect: {:?}", self.config);
                        }
                        self.state = crate::state::State::Connect;
                    }
                }
                crate::state::State::Connect => {
                    if event == crate::event::Event::TcpConnect {
                        self.connection
                            .as_mut()
                            .expect("connection is none")
                            .send(crate::packet::message::Message::new_open(
                                self.config.local_as,
                                self.config.local_ip,
                            ))
                            .await;
                        self.state = crate::state::State::OpenSent;
                    }
                }
                _ => {
                    tracing::error!("unhandled state: {:?}", self.state);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr as _;

    #[tokio::test]
    async fn connect_transition() {
        let mut peer = Peer::new(crate::config::Config::default());
        peer.start();

        tokio::spawn(async move {
            let remote_config =
                crate::config::Config::from_str("64513 127.0.0.2 65412 127.0.0.1 passive").unwrap();

            let mut remote_peer = Peer::new(remote_config);
            remote_peer.start();
            remote_peer.next().await;
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        peer.next().await;
        assert_eq!(peer.state, crate::state::State::Connect);
    }

    #[tokio::test]
    async fn open_sent_transition() {
        let config = crate::config::Config::default();
        let mut peer = Peer::new(config);
        peer.start();

        tokio::spawn(async move {
            let remote_config =
                crate::config::Config::from_str("64513 127.0.0.2 65412 127.0.0.1 passive").unwrap();

            let mut remote_peer = Peer::new(remote_config);
            remote_peer.start();
            for _ in 0..2 {
                remote_peer.next().await;
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        for _ in 0..2 {
            peer.next().await;
        }

        assert_eq!(peer.state, crate::state::State::OpenSent);
    }
}
