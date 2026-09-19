mod collector;

use collector::procfs::list_processes;

fn main() {
    println!("Running processes:");

    for process in list_processes() {
        println!("{:>8}  {}", process.pid, process.command);
    }
}
