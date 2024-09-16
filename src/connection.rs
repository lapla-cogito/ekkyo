use anyhow::Context as _;
use tokio::io::AsyncWriteExt as _;

#[derive(Debug)]
pub struct Connection {
    connection: tokio::net::TcpStream,
    buf: bytes::BytesMut,
}

impl Connection {
    pub async fn connect(
        config: &crate::config::Config,
    ) -> Result<Self, crate::error::ConnectionErr> {
        let connection = match config.mode {
            crate::config::Mode::Active => Self::connect_remote(config).await,
            crate::config::Mode::Passive => Self::accept_remote(config).await,
        }?;

        let buf = bytes::BytesMut::with_capacity(150);

        Ok(Self { connection, buf })
    }

    async fn connect_remote(
        config: &crate::config::Config,
    ) -> anyhow::Result<tokio::net::TcpStream> {
        tracing::info!("connecting to remote peer {0}:{1}", config.remote_ip, 179);
        tokio::net::TcpStream::connect((config.remote_ip, 179))
            .await
            .context(format!(
                "failed to connect to remote peer {0}:{1}",
                config.remote_ip, 179
            ))
    }

    async fn accept_remote(
        config: &crate::config::Config,
    ) -> anyhow::Result<tokio::net::TcpStream> {
        let listener = tokio::net::TcpListener::bind((config.local_ip, 179))
            .await
            .context(format!(
                "failed to bind to local peer {0}:{1}",
                config.local_ip, 179
            ))?;

        let (connection, _) = listener
            .accept()
            .await
            .context("failed to accept connection")?;
        Ok(connection)
    }

    pub async fn send(&mut self, msg: crate::packet::message::Message) {
        let bytes: bytes::BytesMut = msg.into();
        self.connection.write_all(&bytes[..]).await.unwrap();
    }

    pub async fn get_message(&mut self) -> Option<crate::packet::message::Message> {
        self.read_data_from_tcp_connection().await;
        let buffer = self.split_buffer_at_message_separator()?;
        crate::packet::message::Message::try_from(buffer).ok()
    }

    pub async fn read_data_from_tcp_connection(&mut self) {
        loop {
            let mut buf = vec![];
            match self.connection.try_read_buf(&mut buf) {
                Ok(0) => {
                    tracing::info!("connection closed");
                    break;
                }
                Ok(n) => {
                    self.buf.extend_from_slice(&buf[..n]);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => {
                    tracing::error!("failed to read from socket; err = {:?}", e);
                    break;
                }
            }
        }
    }

    fn split_buffer_at_message_separator(&mut self) -> Option<bytes::BytesMut> {
        let idx = self.get_idx_message_separator().ok()?;
        if self.buf.len() < idx {
            return None;
        }

        Some(self.buf.split_to(idx))
    }

    fn get_idx_message_separator(&self) -> anyhow::Result<usize> {
        let min_message_len = 19;
        if self.buf.len() < min_message_len {
            return Err(anyhow::anyhow!("buffer too short"));
        }

        Ok(u16::from_be_bytes([self.buf[16], self.buf[17]]) as usize)
    }
}
