extern crate serde;
extern crate serde_json;
extern crate syscall;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
mod mincore;
mod mnt;
mod stat;
use mnt::switch_mount_ns;

fn main() -> io::Result<()> {
    let pid = 7912;
    switch_mount_ns(pid);

    let file_path = format!("/proc/{}/maps", pid);
    let f = File::open(file_path)?;
    let f = BufReader::new(f);
    let mut files: HashSet<String> = HashSet::new();
    for line in f.lines() {
        if let Ok(line) = line {
            let parts = line
                .split_ascii_whitespace()
                .into_iter()
                .collect::<Vec<&str>>();
            if parts.len() == 6 && parts[5].starts_with("/") {
                files.insert(parts[5].to_owned());
            }
        }
    }
    let mut rtn: Vec<stat::PcStatus> = vec![];
    for filename in files {
        rtn.push(stat::get_pc_status(filename.to_string()).unwrap());
    }
    println!("{}", serde_json::to_string_pretty(&rtn).unwrap());

    //let mut rtn: Vec<stat::PcStatus> = vec![];
    //for filename in env::args().skip(1) {
    //rtn.push(stat::get_pc_status(filename.to_string()).unwrap());
    //}
    //println!("{}", serde_json::to_string_pretty(&rtn).unwrap());
    Ok(())
}
