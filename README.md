# Nethogs-a-like

Project summary
We will build a portable Linux network-monitoring utility inspired by NetHogs.

Main goals
Display network usage per process.
Show upload and download rates.
Track sockets rather than relying only on packet-time PID lookup.
Reduce unknown traffic through socket ownership and process-lifetime tracking.
Support an eBPF-enhanced mode when the kernel allows it.
Provide a /proc, netlink, and libpcap fallback mode.
Work independently of the init system.
Run interactively without requiring a background service.
Optionally provide both SysVinit and systemd service files.
Produce standalone release binaries for common Linux architectures.
Proposed technology
Rust: main application, accounting, process tracking, terminal UI, CLI, packaging.
C eBPF: kernel probes for socket and network events.
libbpf/libbpf-rs: communication between Rust and eBPF components.
/proc: process, descriptor, socket, and fallback information.
Netlink: additional socket and interface information.
libpcap: fallback traffic capture.
Ratatui or another terminal UI library: interactive display.
Planned operating modes
text


classic       /proc + libpcap; widest compatibility
enhanced      /proc + netlink + socket tracking
ebpf          kernel-level socket/process tracking; best attribution
automatic     select the best available mode
Important design decision
The core program will not know or care whether the host uses SysVinit, systemd, OpenRC, or no service manager. Init support will be limited to separate packaging files:

text


packaging/
├── systemd/netmonitor.service
└── sysvinit/netmonitor
GitHub Actions can build, test, package, and upload binaries as artifacts. GitHub’s Rust workflow documentation supports Cargo-based builds and artifact uploads, and matrix builds are appropriate for producing multiple target architectures. 
GitHub

Suggested repository layout
text


netmonitor/
├── .github/
│   └── workflows/
│       └── ci.yml
├── ebpf/
│   ├── src/
│   └── build.rs
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── process.rs
│   ├── sockets.rs
│   ├── accounting.rs
│   ├── collector/
│   │   ├── mod.rs
│   │   ├── procfs.rs
│   │   ├── netlink.rs
│   │   ├── pcap.rs
│   │   └── ebpf.rs
│   └── ui.rs
├── packaging/
│   ├── systemd/
│   │   └── netmonitor.service
│   └── sysvinit/
│       └── netmonitor
├── Cargo.toml
├── Cargo.lock
├── README.md
└── LICENSE
