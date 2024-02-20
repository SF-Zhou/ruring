use crate::flags::CQEFlags;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum OpType {
    #[default]
    Nop,
    Accept,
    Read,
    Write,
    Receive,
    Send,
    SendZeroCopy,
}

#[derive(Default, Debug, Clone)]
pub struct Entry {
    pub res: i32,
    pub len: u32,
    pub flags: CQEFlags,
    pub send_bid: u16,
    pub bid: u16,
    pub op_type: OpType,
    pub multishot: bool,
    pub fd: Option<std::sync::Arc<std::net::TcpStream>>,
}
