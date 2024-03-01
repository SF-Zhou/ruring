#[derive(Default, Debug, Clone, Copy)]
pub struct Config {
    pub entries: u32,
    pub recv_buffer_size: usize,
    pub recv_buffer_count: usize,
}

impl Config {
    pub fn new(entries: u32) -> Config {
        Config {
            entries,
            recv_buffer_count: 256,
            recv_buffer_size: 4096,
        }
    }
}
