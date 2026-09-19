use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProcessSockets {
    pub pid: u32,
    pub socket_inodes: Vec<u64>,
}

pub fn sockets_for_process(pid: u32) -> ProcessSockets {
    let fd_path = PathBuf::from(format!("/proc/{pid}/fd"));
    let mut socket_inodes = Vec::new();

    let Ok(entries) = fs::read_dir(fd_path) else {
        return ProcessSockets { pid, socket_inodes };
    };

    for entry in entries.flatten() {
        let Ok(target) = fs::read_link(entry.path()) else {
            continue;
        };

        let target = target.to_string_lossy();

        let Some(inode_text) = target
            .strip_prefix("socket:[")
            .and_then(|value| value.strip_suffix(']'))
        else {
            continue;
        };

        let Ok(inode) = inode_text.parse::<u64>() else {
            continue;
        };

        if !socket_inodes.contains(&inode) {
            socket_inodes.push(inode);
        }
    }

    ProcessSockets { pid, socket_inodes }
}
