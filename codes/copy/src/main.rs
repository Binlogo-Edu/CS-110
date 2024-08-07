extern crate libc;

use libc::{
    c_uint, c_void, close, open, read, write, O_CREAT, O_EXCL, O_RDONLY, O_WRONLY, S_IRUSR, S_IWUSR,
};
use std::env;
use std::ffi::CString;
use std::io;

const BUFFER_SIZE: usize = 1024;

fn copy_file(src: &str, dst: &str) -> io::Result<()> {
    let src_c = CString::new(src).unwrap();
    let dst_c = CString::new(dst).unwrap();

    unsafe {
        let src_fd = open(src_c.as_ptr(), O_RDONLY);
        if src_fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let dst_fd = open(
            dst_c.as_ptr(),
            O_WRONLY | O_CREAT | O_EXCL,
            (S_IRUSR | S_IWUSR) as c_uint,
        );
        if dst_fd < 0 {
            close(src_fd);
            return Err(io::Error::last_os_error());
        }

        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            let bytes_read = read(src_fd, buffer.as_mut_ptr() as *mut c_void, BUFFER_SIZE);
            if bytes_read < 0 {
                close(src_fd);
                close(dst_fd);
                return Err(io::Error::last_os_error());
            }
            if bytes_read == 0 {
                break;
            }

            let mut bytes_written = 0;
            while bytes_written < bytes_read {
                let result = write(
                    dst_fd,
                    buffer[bytes_written as usize..bytes_read as usize].as_ptr() as *const c_void,
                    (bytes_read - bytes_written) as usize,
                );
                if result < 0 {
                    close(src_fd);
                    close(dst_fd);
                    return Err(io::Error::last_os_error());
                }
                bytes_written += result;
            }
        }

        close(src_fd);
        close(dst_fd);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <source> <destination>", args[0]);
        return;
    }

    let src = &args[1];
    let dst = &args[2];

    match copy_file(src, dst) {
        Ok(_) => println!("File copied successfully."),
        Err(e) => eprintln!("Error copying file: {}", e),
    }
}

#[cfg(test)]
mod tests;
