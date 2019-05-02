extern crate libc;
extern crate syscall;
use std::os::unix::io::RawFd;
use std::{io, ptr};

fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

fn mmap(
    fd: RawFd,
    offset: u64,
    length: usize,
    prot: libc::c_int,
    flags: libc::c_int,
) -> io::Result<*mut libc::c_void> {
    let alignment = offset % page_size() as u64;
    let aligned_offset = offset - alignment;
    let aligned_len = length + alignment as usize;
    if aligned_len == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "memory map must have a non-zero length",
        ));
    }
    unsafe {
        let ptr = libc::mmap(
            ptr::null_mut(),
            aligned_len as libc::size_t,
            prot,
            flags,
            fd,
            aligned_offset as libc::off_t,
        );

        if ptr == libc::MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            Ok(ptr.offset(alignment as isize))
        }
    }
}

pub fn file_mincore(f: RawFd, size: u64) -> Result<Vec<bool>, String> {
    if size == 0 {
        return Ok(vec![]);
    }
    // PROT_NONE = 0x0
    // MAP_SHARED = 0x1
    let mmap_fd = mmap(f, 0, size as usize, 0x0, 0x1).unwrap();
    let vec_size = (size as usize + page_size() - 1) / page_size();
    let mut vec: Vec<u8> = vec![0; vec_size];
    let ret = unsafe {
        syscall::syscall3(
            syscall::nr::MINCORE,
            mmap_fd as usize,
            size as usize,
            vec.as_mut_ptr() as usize,
        )
    };
    if ret != 0 {
        return Err(format!("MINCORE syscall return {}", ret));
    }
    Ok(vec.into_iter().map(|v| v & 1 != 0).collect::<Vec<bool>>())
}
