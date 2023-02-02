use libc::{c_int, winsize};
use libc::{STDOUT_FILENO, TIOCGWINSZ};
use std::io::{self, Result};
use std::mem::MaybeUninit;

pub fn get_window_size() -> Result<(usize, usize)> {
    let win = unsafe {
        let mut win = MaybeUninit::<winsize>::uninit();
        os_result(libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, win.as_mut_ptr()))
            .map(|_| win.assume_init())?
    };
    Ok((win.ws_row as usize, win.ws_col as usize))
}

fn os_result(err: c_int) -> Result<()> {
    if err < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
