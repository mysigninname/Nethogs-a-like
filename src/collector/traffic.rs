use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrafficTotals {
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

pub fn read_traffic_totals() -> io::Result<TrafficTotals> {
    let contents = fs::read_to_string("/proc/net/dev")?;

    let mut received_bytes = 0;
    let mut transmitted_bytes = 0;

    for line in contents.lines().skip(2) {
        let Some((_, values)) = line.split_once(':') else {
            continue;
        };

        let fields: Vec<&str> = values.split_whitespace().collect();

        if fields.len() < 9 {
            continue;
        }

        received_bytes += fields[0].parse::<u64>().unwrap_or(0);
        transmitted_bytes += fields[8].parse::<u64>().unwrap_or(0);
    }

    Ok(TrafficTotals {
        received_bytes,
        transmitted_bytes,
    })
}
