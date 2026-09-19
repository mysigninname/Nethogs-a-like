use std::fs;

fn main() {
    println!("Running processes:");

    let Ok(entries) = fs::read_dir("/proc") else {
        eprintln!("Could not read /proc");
        return;
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.chars().all(|character| character.is_ascii_digit()) {
            continue;
        }

        let pid = &name;
        let command_path = format!("/proc/{pid}/comm");

        if let Ok(command) = fs::read_to_string(command_path) {
            println!("{pid:>8}  {}", command.trim());
        }
    }
}
