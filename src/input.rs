extern crate termion;

use termion::async_stdin;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn input_handler(scr: &mut dyn Write, ) {
    let mut stdin = async_stdin().bytes();
    let mut last_input = String::new();

    // create input channel and run in separate thread
}
