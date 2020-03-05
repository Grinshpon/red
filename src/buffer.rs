extern crate termion;
use termion::*;
use termion::screen::*;
use termion::raw::RawTerminal;

extern crate ropey;
use ropey::Rope;

#[macro_use]
use crate::util::*;

use std::io::{Write, Stdout};
use std::fs::File;
use std::collections::BTreeMap;

pub struct Buffer {
  pub name: String,
  pub file: Option<File>,
  pub diff: File,
  pub buffer: Rope,
  pub cursor: Pos,
  pub selection: Option<Vec<(usize,u16,u16)>>, // vector of 3-tuples representing line, start, end. (for Ctrl-V behavior)
  //pub registers: ??,
  pub context: Box<AlternateScreen<RawTerminal<Stdout>>>,
}

impl Buffer {
  pub fn set_cursor(&mut self, x: u16, y: u16) {
    set_cursor(&mut *(self.context), x, y);
    self.cursor = (x,y);
  }

  pub fn move_cursor(&mut self, x: u16, y: u16) {
    let (mut nx, mut ny) = self.cursor;
    nx += x;
    ny += y;
    set_cursor(&mut *(self.context), nx, ny);
    self.cursor = (nx,ny);
  }

  pub fn close() { // should be io::Result
    // ...
  }
}

fn set_cursor(scr: &mut AlternateScreen<RawTerminal<Stdout>>, x: u16, y: u16) {
  write!(scr,"{}",cursor::Goto(x,y)).unwrap();
}
