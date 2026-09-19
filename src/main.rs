mod collector;

use collector::procfs::list_processes;
use collector::sockets::sockets_for_process;

fn main() {
    println!("{:>8}  {:<24} SOCKET INODES", "PID", "PROCESS");

    for process in list_processes() {
        let sockets = sockets_for_process(process.pid);

        if sockets.socket_inodes.is_empty() {
            continue;
        }

        println!(
            "{:>8}  {:<24} {:?}",
            sockets.pid, process.command, sockets.socket_inodes
        );
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
