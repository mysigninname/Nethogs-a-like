use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Tcp6,
    Udp,
    Udp6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketInfo {
    pub inode: u64,
    pub protocol: Protocol,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: Option<String>,
    pub remote_port: Option<u16>,
    pub state: String,
}

pub fn read_socket_table(path: &str, protocol: Protocol) -> io::Result<Vec<SocketInfo>> {
    let contents = fs::read_to_string(path)?;
    Ok(parse_socket_table(&contents, protocol))
}

pub fn parse_socket_table(contents: &str, protocol: Protocol) -> Vec<SocketInfo> {
    contents
        .lines()
        .skip(1)
        .filter_map(|line| parse_socket_line(line, protocol))
        .collect()
}

fn parse_socket_line(line: &str, protocol: Protocol) -> Option<SocketInfo> {
    let fields: Vec<&str> = line.split_whitespace().collect();

    // Fields:
    // 0 sl
    // 1 local_address
    // 2 remote_address
    // 3 state
    // ...
    // 8 uid
    // 9 timeout
    // 10 inode
    if fields.len() < 11 {
        return None;
    }

    let (local_address, local_port) = parse_endpoint(fields[1])?;
    let (remote_address, remote_port) = parse_endpoint(fields[2])?;
    let inode = fields[10].parse::<u64>().ok()?;

    Some(SocketInfo {
        inode,
        protocol,
        local_address,
        local_port,
        remote_address: Some(remote_address),
        remote_port: Some(remote_port),
        state: fields[3].to_string(),
    })
}

fn parse_endpoint(value: &str) -> Option<(String, u16)> {
    let (address, port) = value.split_once(':')?;
    let port = u16::from_str_radix(port, 16).ok()?;

    Some((address.to_string(), port))
}
