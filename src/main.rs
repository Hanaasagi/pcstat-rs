extern crate serde;
extern crate serde_json;
use std::env;
mod mincore;
mod stat;

fn main() {
    let mut rtn: Vec<stat::PcStatus> = vec![];
    for filename in env::args().skip(1) {
        rtn.push(stat::get_pc_status(filename.to_string()).unwrap());
    }
    println!("{}", serde_json::to_string_pretty(&rtn).unwrap());
}
