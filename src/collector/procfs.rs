use std::fs;

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub command: String,
}

pub fn list_processes() -> Vec<ProcessInfo> {
    let Ok(entries) = fs::read_dir("/proc") else {
        return Vec::new();
    };

    let mut processes = Vec::new();

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        let Ok(pid) = name.parse::<u32>() else {
            continue;
        };

        let command_path = format!("/proc/{pid}/comm");

        let Ok(command) = fs::read_to_string(command_path) else {
            continue;
        };

        processes.push(ProcessInfo {
            pid,
            command: command.trim().to_string(),
        });
    }

    processes.sort_by_key(|process| process.pid);
    processes
}

#[cfg(test)]
mod tests {
    use super::list_processes;

    #[test]
    fn finds_current_process() {
        let processes = list_processes();
        let current_pid = std::process::id();

        assert!(
            processes.iter().any(|process| process.pid == current_pid),
            "the current process was not found"
        );
    }
}
