mod collector;
mod process;

use collector::procfs::list_processes;
use collector::sockets::sockets_for_process;
use collector::traffic::read_traffic_totals;
use std::thread::sleep;
use std::time::Duration;


fn main() {
        loop {
        print!("\x1B[2J\x1B[1;1H");

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

    for (inode, owners) in &owners {
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
                if let Some(socket_owners) = owners.get(&socket.inode) {
                    println!("{socket:?} owners: {socket_owners:?}");
                }
            }
        }
        Err(error) => {
            eprintln!("Could not read socket tables: {error}");
        }
    }

    println!();
    println!("NETWORK SPEED");

    match read_traffic_totals() {
        Ok(first) => {
            sleep(Duration::from_secs(1));

            match read_traffic_totals() {
                Ok(second) => {
                    let received_per_second = second
                        .received_bytes
                        .saturating_sub(first.received_bytes);

                    let transmitted_per_second = second
                        .transmitted_bytes
                        .saturating_sub(first.transmitted_bytes);

                    println!("Received: {} bytes/s", received_per_second);
                    println!("Transmitted: {} bytes/s", transmitted_per_second);
                }
                Err(error) => {
                    eprintln!("Could not read second network total: {error}");
                }
            }
        }
        Err(error) => {
            eprintln!("Could not read first network total: {error}");
        }
    }
            sleep(Duration::from_secs(1));
    }
}
