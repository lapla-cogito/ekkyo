use anyhow::Context as _;
use bytes::BufMut as _;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OpenMessage {
    hdr: crate::packet::hdr::Header,
    version: crate::types::Version,
    asnum: crate::types::ASNum,
    hold_time: crate::types::HoldTime,
    bgp_id: std::net::Ipv4Addr,
    opt_params: bytes::BytesMut,
    opt_params_len: u8,
}

impl OpenMessage {
    pub fn new(asnum: crate::types::ASNum, bgp_id: std::net::Ipv4Addr) -> Self {
        Self {
            hdr: crate::packet::hdr::Header::new(29, crate::packet::hdr::MessageType::Open),
            version: crate::types::Version::new(),
            asnum,
            hold_time: crate::types::HoldTime::new(),
            bgp_id,
            opt_params: bytes::BytesMut::new(),
            opt_params_len: 0,
        }
    }
}

impl TryFrom<bytes::BytesMut> for OpenMessage {
    type Error = crate::error::ConvertBytesErr;

    fn try_from(value: bytes::BytesMut) -> Result<Self, Self::Error> {
        let header = crate::packet::hdr::Header::try_from(bytes::BytesMut::from(&value[0..19]))?;
        let version = crate::types::Version::try_from(value[crate::constants::HEADER_LEN])?;
        let asnum = crate::types::ASNum::from(u16::from_be_bytes([
            value[crate::constants::HEADER_LEN + 1],
            value[crate::constants::HEADER_LEN + 2],
        ]));
        let hold_time = crate::types::HoldTime::from(u16::from_be_bytes([
            value[crate::constants::HEADER_LEN + 3],
            value[crate::constants::HEADER_LEN + 4],
        ]));
        let tmp: [u8; 4] = value
            [crate::constants::HEADER_LEN + 5..crate::constants::HEADER_LEN + 9]
            .try_into()
            .context(format!(
                "invalid BGP ID: {:?}",
                &value[crate::constants::HEADER_LEN + 5..crate::constants::HEADER_LEN + 9]
            ))?;
        let bgp_id = std::net::Ipv4Addr::from(tmp);
        let opt_params_len = value[crate::constants::HEADER_LEN + 9];
        let opt_params = bytes::BytesMut::from(
            &value[crate::constants::HEADER_LEN + 10
                ..crate::constants::HEADER_LEN + 10 + opt_params_len as usize],
        );

        Ok(Self {
            hdr: header,
            version,
            asnum,
            hold_time,
            bgp_id,
            opt_params,
            opt_params_len,
        })
    }
}

impl From<OpenMessage> for bytes::BytesMut {
    fn from(msg: OpenMessage) -> bytes::BytesMut {
        let mut bytes = bytes::BytesMut::new();
        let hdr_bytes: bytes::BytesMut = msg.hdr.into();
        bytes.put(hdr_bytes);
        bytes.put_u8(msg.version.into());
        bytes.put_u16(msg.asnum.into());
        bytes.put_u16(msg.hold_time.into());
        bytes.put_u32(msg.bgp_id.into());
        bytes.put_u8(msg.opt_params_len);
        bytes.put(msg.opt_params);

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_bytes_message() {
        let expected = OpenMessage::new(64512.into(), "127.0.0.1".parse().unwrap());
        let open_message_bytes: bytes::BytesMut = expected.clone().into();
        let open_message: OpenMessage = open_message_bytes.try_into().unwrap();

        assert_eq!(open_message, expected);
    }
}
