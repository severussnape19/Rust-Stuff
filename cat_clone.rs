#![allow(dead_code)]
#![allow(unused)]

use std::{env, error::Error, io::stdout, process};

use nix::{fcntl::{OFlag, open}, libc::EXIT_FAILURE, sys::stat::Mode, unistd::{ Whence, lseek, read as nix_read, write as nix_write}};

const BUFSIZE: usize = 128;

fn err_exit(msg: impl Error) -> ! {
    eprintln!("Fatal Error: {msg}");
    process::exit(EXIT_FAILURE);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 || argv[2].to_lowercase() != "cat" || argv[2] == "--help" {
        eprintln!("Usage: {} <file>", argv[0]);
        process::exit(EXIT_FAILURE);
    }

    let fd = open(
        argv[1].as_str(),
        OFlag::O_RDONLY,
        Mode::S_IRUSR | Mode::S_IRGRP
    ).unwrap_or_else(|err| err_exit(err));

    lseek(&fd, 0, Whence::SeekSet).unwrap_or_else(|err| err_exit(err));
    let mut buffer = [0u8; BUFSIZE];

    loop {
        match nix_read(&fd, &mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                let mut bytes_written = 0;

                while bytes_read > bytes_written {
                    match nix_write(stdout(), &buffer[bytes_written..bytes_read]) {
                        Ok(n) => bytes_written += n,
                        Err(err) => err_exit(err),
                    }
                }
            },
            Err(err) => err_exit(err)
        }
    }
}
