#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct HoldTime(u16);

impl HoldTime {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<HoldTime> for u16 {
    fn from(ht: HoldTime) -> u16 {
        ht.0
    }
}

impl From<u16> for HoldTime {
    fn from(ht: u16) -> HoldTime {
        HoldTime(ht)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Version(u8);

impl Version {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Version {
    fn default() -> Self {
        Version(4)
    }
}

impl From<Version> for u8 {
    fn from(v: Version) -> u8 {
        v.0
    }
}

impl TryFrom<u8> for Version {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if v > 4 {
            return Err(anyhow::anyhow!("invalid version"));
        }

        Ok(Version(v))
    }
}
