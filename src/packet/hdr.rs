use bytes::BufMut as _;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Header {
    length: u16,
    pub message_type: MessageType,
}

impl Header {
    pub fn new(length: u16, message_type: MessageType) -> Self {
        Self {
            length,
            message_type,
        }
    }
}

impl TryFrom<bytes::BytesMut> for Header {
    type Error = crate::error::ConvertBytesErr;

    fn try_from(value: bytes::BytesMut) -> Result<Self, Self::Error> {
        let _marker = &value[0..16];
        let length = u16::from_be_bytes([value[16], value[17]]);
        let message_type = value[18].try_into()?;

        Ok(Self {
            length,
            message_type,
        })
    }
}

impl From<Header> for bytes::BytesMut {
    fn from(hdr: Header) -> bytes::BytesMut {
        let mut bytes = bytes::BytesMut::new();
        let marker = [0xff; 16];
        let length = hdr.length.to_be_bytes();
        let message_type: u8 = hdr.message_type.into();

        bytes.put(&marker[..]);
        bytes.put(&length[..]);
        bytes.put_u8(message_type);
        bytes
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MessageType {
    Open,
    Update,
    Notification,
    KeepAlive,
}

impl TryFrom<u8> for MessageType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Open),
            2 => Ok(Self::Update),
            3 => Ok(Self::Notification),
            4 => Ok(Self::KeepAlive),
            _ => Err(anyhow::anyhow!("invalid message type")),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(mt: MessageType) -> u8 {
        match mt {
            MessageType::Open => 1,
            MessageType::Update => 2,
            MessageType::Notification => 3,
            MessageType::KeepAlive => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_try_from() {
        let mut bytes = bytes::BytesMut::with_capacity(19);
        bytes.extend_from_slice(&[0xff; 16]);
        bytes.extend_from_slice(&19u16.to_be_bytes());
        bytes.extend_from_slice(&[1]);

        let header = Header::try_from(bytes).unwrap();
        assert_eq!(header.length, 19);
        assert_eq!(header.message_type, MessageType::Open);
    }

    #[test]
    fn test_message_type_try_from() {
        assert_eq!(MessageType::try_from(1).unwrap(), MessageType::Open);
        assert_eq!(MessageType::try_from(2).unwrap(), MessageType::Update);
        assert_eq!(MessageType::try_from(3).unwrap(), MessageType::Notification);
        assert_eq!(MessageType::try_from(4).unwrap(), MessageType::KeepAlive);
    }

    #[test]
    fn test_message_type_into() {
        assert_eq!(u8::from(MessageType::Open), 1);
        assert_eq!(u8::from(MessageType::Update), 2);
        assert_eq!(u8::from(MessageType::Notification), 3);
        assert_eq!(u8::from(MessageType::KeepAlive), 4);
    }
}
