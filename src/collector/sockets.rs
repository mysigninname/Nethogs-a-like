use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SocketOwner {
    pub pid: u32,
    pub process_name: String,
}

pub fn group_socket_owners(
    processes: &[crate::collector::procfs::ProcessInfo],
    process_sockets: &[crate::process::ProcessSockets],
) -> HashMap<u64, Vec<SocketOwner>> {
    let mut owners: HashMap<u64, Vec<SocketOwner>> = HashMap::new();

    for process_socket in process_sockets {
        let process_name = processes
            .iter()
            .find(|process| process.pid == process_socket.pid)
            .map(|process| process.command.clone())
            .unwrap_or_else(|| "unknown".to_string());

        for inode in &process_socket.socket_inodes {
            owners.entry(*inode).or_default().push(SocketOwner {
                pid: process_socket.pid,
                process_name: process_name.clone(),
            });
        }
    }

    owners
}

pub fn sockets_for_process(pid: u32) -> crate::process::ProcessSockets {
    let fd_path = PathBuf::from(format!("/proc/{pid}/fd"));
    let mut socket_inodes = Vec::new();

    let Ok(entries) = fs::read_dir(fd_path) else {
        return crate::process::ProcessSockets {
            pid,
            socket_inodes,
        };
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

    crate::process::ProcessSockets {
        pid,
        socket_inodes,
    }
}
