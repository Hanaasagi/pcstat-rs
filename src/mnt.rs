use std::fs;
use std::process;
use syscall;

const CLONE_NEWNS: usize = 0x00020000;

pub fn switch_mount_ns(pid: u32) -> Result<(), String> {
    let self_ns = get_mount_ns(process::id());
    let pid_ns = get_mount_ns(pid);

    if self_ns != pid_ns {
        set_ns(pid_ns)?;
    }
    Ok(())
}

fn get_mount_ns(pid: u32) -> i64 {
    let file_path = format!("/proc/{}/ns/mnt", pid);
    // 'mnt:[4026531840]'
    let content = fs::read_link(file_path);
    if content.is_err() {
        return 0;
    }

    content
        .unwrap()
        .to_str()
        .unwrap()
        .trim_start_matches("mnt:[")
        .trim_end_matches("]")
        .parse::<i64>()
        .unwrap_or(0)
}

fn set_ns(fd: i64) -> Result<(), String> {
    let ret = unsafe { syscall::syscall2(syscall::nr::SETNS, fd as usize, CLONE_NEWNS) };

    if ret != 0 {
        return Err(format!("syscall SETNS failed, return {}", ret));
    }
    Ok(())
}
