mod collector;
mod process;

use collector::procfs::list_processes;
use collector::sockets::sockets_for_process;

fn main() {
    println!("{:>8}  {:<24} SOCKET INODES", "PID", "PROCESS");

    let processes = list_processes();
    let mut process_sockets = Vec::new();

    for process in &processes {
        let sockets = sockets_for_process(process.pid);

        if sockets.socket_inodes.is_empty() {
            continue;
        }

        println!(
            "{:>8}  {:<24} {:?}",
            sockets.pid, process.command, sockets.socket_inodes
        );

        process_sockets.push(sockets);
    }

    let owners = collector::sockets::group_socket_owners(&processes, &process_sockets);

    println!();
    println!("SOCKET OWNERS");

    for (inode, owners) in owners {
        println!("Socket {inode}:");

        for owner in owners {
            println!("  PID {}: {}", owner.pid, owner.process_name);
        }
    }

    println!();
    println!("SOCKET TABLE");

    match collector::net_table::read_all_socket_tables() {
        Ok(sockets) => {
            println!("Found {} sockets", sockets.len());

            for socket in sockets {
                println!("{socket:?}");
            }
        }
        Err(error) => {
            eprintln!("Could not read socket tables: {error}");
        }
    }
}
