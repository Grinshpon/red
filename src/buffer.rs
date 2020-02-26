extern crate termion;
use termion::*;
use termion::screen::*;

#[macro_use]
use crate::util::*;

use std::io::{Write, Stdout};
use std::collections::BTreeMap;

pub struct Buffer {
  buffer: BTreeMap<usize, ()>, //placeholder () will be Rope
  cursor: Pos,
  selection: Vec<(usize,u16,u16)>, // vector of 3-tuples representing line, start, end. (for Ctrl-V behavior)
  //registers: ??,
  //swap: file,
  context: Box<AlternateScreen<Stdout>>
}

impl Buffer {
  fn set_cursor(&mut self, x: u16, y: u16) {
    setCursor!(*(self.context), x, y);
    self.cursor = (x,y);
  }

  fn move_cursor(&mut self, x: u16, y: u16) {
    let (mut nx, mut ny) = self.cursor;
    nx += x;
    ny += y;
    setCursor!(*(self.context), nx, ny);
    self.cursor = (nx,ny);
  }
}
