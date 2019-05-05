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
) -> Result<*mut libc::c_void, String> {
    let alignment = offset % page_size() as u64;
    let aligned_offset = offset - alignment;
    let aligned_len = length + alignment as usize;
    if aligned_len == 0 {
        return Err("memory map must have a non-zero length".into());
    }
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            aligned_len as libc::size_t,
            prot,
            flags,
            fd,
            aligned_offset as libc::off_t,
        )
    };

    if ptr == libc::MAP_FAILED {
        Err(format!("unable to mmap: {}", io::Error::last_os_error()))
    } else {
        Ok(unsafe { ptr.offset(alignment as isize) })
    }
}

fn munmap(addr: *mut libc::c_void, length: usize) -> Result<(), String> {
    let alignment = addr as usize % page_size();
    let ret = unsafe {
        libc::munmap(
            addr.offset(-(alignment as isize)),
            (length + alignment) as libc::size_t,
        )
    };
    if ret != 0 {
        return Err(format!(
            "unable to unmap mmap: {}",
            io::Error::last_os_error()
        ));
    } else {
        Ok(())
    }
}

pub fn file_mincore(f: RawFd, size: u64) -> Result<Vec<bool>, String> {
    if size == 0 {
        return Ok(vec![]);
    }
    // PROT_NONE = 0x0
    // MAP_SHARED = 0x1
    let mmap_fd = mmap(f, 0, size as usize, 0x0, 0x1).map_err(|e| e.to_string())?;
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
    munmap(mmap_fd, size as usize)?;
    if ret != 0 {
        return Err(format!("MINCORE syscall failed, return {}", ret));
    }
    Ok(vec.into_iter().map(|v| v & 1 != 0).collect::<Vec<bool>>())
}
