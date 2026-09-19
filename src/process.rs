#[derive(Debug)]
pub struct ProcessSockets {
    pub pid: u32,
    pub socket_inodes: Vec<u64>,
}
