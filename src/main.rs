extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate syscall;
use clap::{App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
mod mincore;
mod mnt;
mod stat;
use mnt::switch_mount_ns;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> io::Result<()> {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("pid")
                .short("p")
                .help("process id")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("files")
                .short("f")
                .help("file list")
                .takes_value(true)
                .min_values(1),
        )
        .get_matches();

    let mut files: HashSet<String> = HashSet::new();

    if let Some(pid) = matches.value_of("pid") {
        let pid = pid.parse::<u32>().expect("invalid pid");
        switch_mount_ns(pid);

        let file_path = format!("/proc/{}/maps", pid);
        let f = File::open(file_path)?;
        let f = BufReader::new(f);
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
    } else {
        for file in matches
            .values_of("files")
            .unwrap()
            .map(|s| s.into())
            .collect::<Vec<String>>()
        {
            files.insert(file);
        }
    }

    let mut rtn: Vec<stat::PcStatus> = vec![];
    for filename in files {
        rtn.push(stat::get_pc_status(filename.to_string()).unwrap());
    }
    println!("{}", serde_json::to_string_pretty(&rtn).unwrap());
    Ok(())
}
