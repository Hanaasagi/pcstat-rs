use super::mincore::file_mincore;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::time::SystemTime;

#[derive(Debug, Serialize)]
pub struct PcStatus {
    name: String,
    size: u64,
    m_time: SystemTime,
    pages: u64,
    cached: u64,
    uncached: u64,
    percent: f64,
}

impl PcStatus {
    fn new(
        name: String,
        size: u64,
        m_time: SystemTime,
        pages: u64,
        cached: u64,
        uncached: u64,
        percent: f64,
    ) -> Self {
        Self {
            name,
            size,
            m_time,
            pages,
            cached,
            uncached,
            percent,
        }
    }
}

pub fn get_pc_status(filename: String) -> Result<PcStatus, String> {
    let metadata = fs::metadata(&filename).map_err(|e| e.to_string())?;
    if metadata.is_dir() {
        return Err("file is directory".into());
    }

    let size = metadata.len();
    let m_time = metadata.modified().map_err(|e| e.to_string())?;

    let f = File::open(&filename).map_err(|e| e.to_string())?;
    let ppstat = file_mincore(f.as_raw_fd(), size)?;
    let pages = ppstat.len();
    let mut cached = 0;
    let mut uncached = 0;
    for &p in &ppstat {
        if p {
            cached += 1;
        } else {
            uncached += 1;
        }
    }
    let percent = (cached as f64 / pages as f64) * 100.00;

    Ok(PcStatus::new(
        filename,
        size,
        m_time,
        pages as u64,
        cached,
        uncached,
        percent,
    ))
}
