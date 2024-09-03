#[derive(Debug)]
pub struct Peer {
    state: crate::state::State,
    queue: crate::queue::Queue,
    config: crate::config::Config,
}

impl Peer {
    pub fn new(config: crate::config::Config) -> Self {
        Peer {
            state: crate::state::State::Idle,
            queue: crate::queue::Queue::new(),
            config,
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
                crate::state::State::Idle => match event {
                    crate::event::Event::Start => {
                        self.state = crate::state::State::Connect;
                    }
                },
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

    #[tokio::test]
    async fn connect_transition() {
        let config: crate::config::Config =
            "64512 127.0.0.1 64513 127.0.0.2 active".parse().unwrap();
        let mut peer = Peer::new(config);
        peer.start();
        peer.next().await;
        assert_eq!(peer.state, crate::state::State::Connect);
    }
}
