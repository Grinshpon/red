extern crate termion;

use termion::async_stdin;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use crate::util::*;

pub struct Input { //will become command
  ch: Char,
  position: Pos,
}
pub type Action = Vec<Input>;
pub type Done = Vec<Action>; //will become "Done" stack
pub type Undone = Vec<Action>; //will become "Undone" stack
pub type Comm = Vec<Action>; //will become command mode input stack

/*
Basicallly how this works:
  When in input mode, any characters you press are saved in an action: an
  input chain. After a brief period of inactivity or if `esc` is pressed,
  the action chain is considered closed/complete. Then the action is pushed
  onto the "done" stack.

  When undoing an action, it's effects are reversed, and the action is popped
  from the "done" stack and pushed onto the "undone" stack.

  When redoing an action, it's effects are applied, and the action is popped
  from the "undone" stack and pushed back onto the "done" stack.
*/

pub fn input_handler(scr: &mut dyn Write, ) {
    let mut stdin = async_stdin().bytes();
    let mut last_input = String::new();

    // create input channel and run in separate thread
}
