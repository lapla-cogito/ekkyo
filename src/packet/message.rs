#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Message {
    Open(crate::packet::open::OpenMessage),
}

impl TryFrom<bytes::BytesMut> for Message {
    type Error = crate::error::ConvertBytesErr;

    fn try_from(bytes: bytes::BytesMut) -> Result<Self, Self::Error> {
        if bytes.len() < crate::constants::HEADER_LEN {
            return Err(Self::Error::from(anyhow::anyhow!(
                "message length is less than header length"
            )));
        }

        let header = crate::packet::hdr::Header::try_from(bytes::BytesMut::from(
            &bytes[..crate::constants::HEADER_LEN],
        ))?;

        match header.message_type {
            crate::packet::hdr::MessageType::Open => {
                let open = crate::packet::open::OpenMessage::try_from(bytes)?;

                Ok(Self::Open(open))
            }
            _ => Err(Self::Error::from(anyhow::anyhow!("unknown message type"))),
        }
    }
}

impl From<Message> for bytes::BytesMut {
    fn from(msg: Message) -> bytes::BytesMut {
        match msg {
            Message::Open(open) => open.into(),
        }
    }
}

impl Message {
    pub fn new_open(asnum: crate::types::ASNum, ip: std::net::Ipv4Addr) -> Self {
        Self::Open(crate::packet::open::OpenMessage::new(asnum, ip))
    }
}
