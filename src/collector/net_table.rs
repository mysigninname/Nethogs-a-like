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

pub fn read_all_socket_tables() -> io::Result<Vec<SocketInfo>> {
    let tables = [
        ("/proc/net/tcp", Protocol::Tcp),
        ("/proc/net/tcp6", Protocol::Tcp6),
        ("/proc/net/udp", Protocol::Udp),
        ("/proc/net/udp6", Protocol::Udp6),
    ];

    let mut sockets = Vec::new();

    for (path, protocol) in tables {
        sockets.extend(read_socket_table(path, protocol)?);
    }

    Ok(sockets)
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

    let (local_address, local_port) = parse_endpoint(fields[1], protocol)?;
    let (remote_address, remote_port) = parse_endpoint(fields[2], protocol)?;

    let inode = fields[10].parse::<u64>().ok()?;

    Some(SocketInfo {
        inode,
        protocol,
        local_address,
        local_port,
        remote_address: Some(remote_address),
        remote_port: Some(remote_port),
        state: socket_state(fields[3], protocol).to_string(),
    })
}

fn parse_endpoint(value: &str, protocol: Protocol) -> Option<(String, u16)> {
    let (address, port) = value.split_once(':')?;
    let port = u16::from_str_radix(port, 16).ok()?;

    let address = match protocol {
        Protocol::Tcp | Protocol::Udp => parse_ipv4_address(address)?,
        Protocol::Tcp6 | Protocol::Udp6 => address.to_string(),
    };

    Some((address, port))
}

fn parse_ipv4_address(value: &str) -> Option<String> {
    if value.len() != 8 {
        return None;
    }

    let bytes = [
        u8::from_str_radix(&value[6..8], 16).ok()?,
        u8::from_str_radix(&value[4..6], 16).ok()?,
        u8::from_str_radix(&value[2..4], 16).ok()?,
        u8::from_str_radix(&value[0..2], 16).ok()?,
    ];

    Some(format!(
        "{}.{}.{}.{}",
        bytes[0], bytes[1], bytes[2], bytes[3]
    ))
}

fn socket_state(value: &str, protocol: Protocol) -> &str {
    match protocol {
        Protocol::Tcp | Protocol::Tcp6 => match value {
            "01" => "ESTABLISHED",
            "02" => "SYN_SENT",
            "03" => "SYN_RECV",
            "04" => "FIN_WAIT1",
            "05" => "FIN_WAIT2",
            "06" => "TIME_WAIT",
            "07" => "CLOSE",
            "08" => "CLOSE_WAIT",
            "09" => "LAST_ACK",
            "0A" => "LISTEN",
            "0B" => "CLOSING",
            _ => "UNKNOWN",
        },
        Protocol::Udp | Protocol::Udp6 => value,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_ipv4_address;

    #[test]
    fn parses_proc_ipv4_address() {
        assert_eq!(
            parse_ipv4_address("0100007F"),
            Some("127.0.0.1".to_string())
        );
    }

    #[test]
    fn parses_socket_inode_and_tcp_state() {
        let line = "0: 0100007F:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000 0 0 0 12345 1";

        let socket = super::parse_socket_line(line, super::Protocol::Tcp)
            .expect("socket line should parse");

        assert_eq!(socket.inode, 12345);
        assert_eq!(socket.local_port, 8080);
        assert_eq!(socket.state, "LISTEN");
    }

}
