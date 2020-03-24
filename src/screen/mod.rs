#![allow(dead_code)]

use std::io::{self, Read};

extern crate libc;

mod termios;
use crate::screen::termios::Termios;

#[macro_use]
mod escape_sequence;


pub struct Screen {
    is_debug: bool,
    termios: Termios,
    terminal_width: usize,
    board_buffer: String,
    top_bar_buffer: String,
//    score_buffer: String,
}

impl Screen {
    pub fn new() -> Screen {
        let mut termios = Termios::new();

        termios.mode_off(libc::ICANON | libc::ECHO);
        print!(alternate_screen!(enable));

        Screen {
            is_debug: false,
            termios,
            terminal_width: 0,
            board_buffer: String::new(),
            top_bar_buffer: String::new(),
        }
    }

    pub fn new_with_terminal_width(width: usize) -> Screen {
        let mut termios = Termios::new();

        termios.mode_off(libc::ICANON | libc::ECHO);
        print!(alternate_screen!(enable));

        Screen {
            is_debug: false,
            termios,
            terminal_width: width,
            board_buffer: String::new(),
            top_bar_buffer: String::new(),
        }
    }

    pub fn new_debug_mode() -> Screen {
        let mut termios = Termios::new();

        termios.mode_off(libc::ICANON | libc::ECHO);
        // print!(alternate_screen!(enable));

        Screen {
            is_debug: true,
            termios,
            terminal_width: 0,
            board_buffer: String::new(),
            top_bar_buffer: String::new(),
        }
    }

    pub fn read_key(&self) -> char {
        let mut buf = [1;1];
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_exact(&mut buf);

        buf[0] as char
    }

    pub fn print(&self) {
        print!("{}{}", clear!(), home_cursor!());
        println!("{}", self.top_bar_buffer);
        println!();
        print!("{}", self.board_buffer);
    }

    pub fn set_board(&mut self, board_buffer: String) {
        self.board_buffer.clear();

        if self.terminal_width == 0 {
            self.board_buffer = board_buffer
        } else {
            for line in board_buffer.lines() {
                //let count = count_color_escape_sequences(line);
                //let count_zenkaku = count_zenkaku_number(line);
                //let width = self.terminal_width + count - count_zenkaku;
                let count= count_color_escape_sequences_and_zenkaku_number(line);
                let width = self.terminal_width + count.0 - count.1;
                self.board_buffer += &format!("{:^width$}\n", line, width = width);
            }
        }
    }

    pub fn set_top_bar(&mut self, top_bar_buffer: String) {
        self.top_bar_buffer.clear();

        if self.terminal_width == 0 {
            self.top_bar_buffer = top_bar_buffer;
        } else {
            let count = count_color_escape_sequences(&top_bar_buffer);
            let width = self.terminal_width + count;
            self.top_bar_buffer = format!("{:^width$}", top_bar_buffer, width = width);
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.termios.set_initial_mode();
        if self.is_debug == false {
            print!(alternate_screen!(disable));
        }
    }
}

fn count_color_escape_sequences(text: &str) -> usize {
    let mut count = 0;
    let mut is_count = false;

    for c in text.chars() {
        match c {
            '\x1b' => {
                count += 1;
                is_count = true;
            }
            'm' => {
                count += 1;
                is_count = false;
            }
            _ => {
                if is_count {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_zenkaku_number(text: &str) -> usize {
    let mut count = 0;
    for c in text.chars() {
        match c {
            '１' | '２' | '３' | '４' | '５' |
            '６' | '７' | '８' | '９' | '０' => {
                count += 1;
            },
            _ => ()

        }
    }
    count
}

fn count_color_escape_sequences_and_zenkaku_number(text: &str) -> (usize, usize) {
    let mut color_escape_sequence_count = 0;
    let mut zenkaku_count = 0;
    let mut is_count = false;

    for c in text.chars() {
        match c {
            '１' | '２' | '３' | '４' | '５' |
            '６' | '７' | '８' | '９' | '０' => {
                zenkaku_count += 1;
            },
            _ => ()
        }

        match c {
            '\x1b' => {
                color_escape_sequence_count += 1;
                is_count = true;
            }
            'm' => {
                color_escape_sequence_count += 1;
                is_count = false;
            },
            _ => {
                if is_count {
                    color_escape_sequence_count += 1;
                }
            }
        }
    }
    
    (color_escape_sequence_count, zenkaku_count)
}

#[test]
fn test_count_color_escape_sequences() {
    let text = "aiueo";
    assert_eq!(count_color_escape_sequences(text), 0);
    let text = "\x1b[2maaa";
    assert_eq!(count_color_escape_sequences(text), 4);
    let text = "\x1b[100maaa";
    assert_eq!(count_color_escape_sequences(text), 6);
    let text = "\x1b[100maaa\x1b[100m\x1b[100m";
    assert_eq!(count_color_escape_sequences(text), 18);
}
#[test]
fn test_count_zenkaku_number() {
    assert_eq!(count_zenkaku_number("123"), 0);
    assert_eq!(count_zenkaku_number("１２３"), 3);
    assert_eq!(count_zenkaku_number("１２３123"), 3);
    assert_eq!(count_zenkaku_number("１２３123１２３"), 6);
}
#[test]
fn test_count_color_escape_sequences_and_zenkaku_number() {
    let text = "";
    assert_eq!(count_color_escape_sequences_and_zenkaku_number(text), (0, 0));
    let text = "\x1b[100maaa\x1b[100m\x1b[100m";
    assert_eq!(count_color_escape_sequences_and_zenkaku_number(text),
               (count_color_escape_sequences(text),
                0));
    let text = "１２３123１２３";
    assert_eq!(count_color_escape_sequences_and_zenkaku_number(text),
               (0,
                count_zenkaku_number(text)));
    let text = "\x1b[100maaa１２３123１２３\x1b[100maaa\x1b[100m";
    assert_eq!(count_color_escape_sequences_and_zenkaku_number(text),
               (count_color_escape_sequences(text),
                count_zenkaku_number(text)));
}