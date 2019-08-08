extern crate termion;

use termion::async_stdin;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub struct Input; //will become command
pub type Done = Vec<Input>; //will become "Done" stack
pub type Undone = Vec<Input>; //will become "Undone" stack
pub type Comm = Vec<Input>; //will become command mode input stack

pub fn input_handler(scr: &mut dyn Write, ) {
    let mut stdin = async_stdin().bytes();
    let mut last_input = String::new();

    // create input channel and run in separate thread
}
