#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Event {
    Start,
    TcpConnect,
    BgpOpen(crate::packet::open::OpenMessage),
}
