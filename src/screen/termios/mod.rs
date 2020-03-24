extern crate libc;
use std::default::Default;
use std::os::raw::{c_int, c_uchar, c_uint};

type cc_t = c_uchar;
type speed_t = c_uint;
type tcflag_t = c_uint;

fn termios_new() -> libc::termios {
    libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; libc::NCCS],
        c_ispeed: 0,
        c_ospeed: 0,
    }
}
 
#[test]
#[ignore]
fn test_termios_new() {
    let mut mode: libc::termios = termios_new();
    unsafe {
        let mut ptr = &mut mode;
        let ret = libc::tcgetattr(0, ptr);
        if ret != 0 {
            panic!();
        }
    }
}

pub struct Termios {
    mode: libc::termios,
    initial_mode: libc::termios,
}

impl Termios {
    pub fn new() -> Termios {
        let mut mode = termios_new();
        
        unsafe {
            let result = libc::tcgetattr(0, &mut mode);
            if result != 0 {
                panic!();
            }
        }

        Termios {
            mode: mode,
            initial_mode: mode.clone(),
        }
    }

    pub fn set_initial_mode(&self) -> &Termios {
        unsafe {
            let result = libc::tcsetattr(0, libc::TCSANOW, &self.initial_mode);
            if result != 0 {
                panic!();
            }
        }
        self
    }

    pub fn set_mode(&self) -> &Termios {
        unsafe {
            let result = libc::tcsetattr(0, libc::TCSANOW, &self.mode);
            if result != 0 {
                panic!();
            }
        }
        self
    }

    pub fn mode_on(&mut self, flag: tcflag_t) -> &Termios {
        self.mode.c_lflag |= flag;
        self.set_mode()
    }

    pub fn mode_off(&mut self, flag: tcflag_t) -> &Termios {
        self.mode.c_lflag &= !flag;
        self.set_mode()
    }
}

#[test]
#[ignore]
fn test_termios() {
    let mut termios = Termios::new();
    termios.mode_on(libc::ECHO);
    termios.set_initial_mode();
}