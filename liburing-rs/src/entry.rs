#[derive(Default, Debug, PartialEq)]
pub enum OpType {
    #[default]
    Nop,
    Accept,
    Read,
    Write,
}

#[derive(Default, Debug)]
pub struct Entry {
    pub res: i32,
    pub flags: u32,
    pub op_type: OpType,
    pub multishot: bool,
}
