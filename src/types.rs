#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ASNum(u16);

impl From<ASNum> for u16 {
    fn from(asn: ASNum) -> u16 {
        asn.0
    }
}

impl From<u16> for ASNum {
    fn from(asn: u16) -> ASNum {
        ASNum(asn)
    }
}
