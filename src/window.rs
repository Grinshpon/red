//  The window is separated like so:
//
//  +-----------------------------------------------------------------------------+
//  |tabs (hidden when empty)                                                     |
//  |-----------------------------------------------------------------------------|
//  |l|                                                                           |
//  |i|                                                                           |
//  |n|                                                                           |
//  |e|         main view                                                         |
//  | |                                                                           |
//  |n|                                                                           |
//  |u|                                                                           |
//  |m|                                                                           |
//  |b|                                                                           |
//  |e|                                                                           |
//  |r|                                                                           |
//  |s|                                                                           |
//  | |                                                                           |
//  | |                                                                           |
//  | |                                                                           |
//  | |                                                                           |
//  | |                                                                           |
//  | |                                                                           |
//  | |                                                                           |
//  |-----------------------------------------------------------------------------|
//  |command/info/mode bar                                     (cursor position)  |
//  +-----------------------------------------------------------------------------+

use termion::{color, style};
use termion::*;

use crate::util::*;
use crate::buffer::*;

use std::io::Write;
use std::fmt::*;

#[derive(Debug)]
pub enum Mode {
  Command,
  Insert,
  Visual,
  VisualBlock,
}
impl Display for Mode {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let s = match self {
      Mode::Command => "",
      Mode::Insert => "-- INSERT --",
      Mode::Visual => "-- VISUAL --",
      Mode::VisualBlock => "-- VISUAL BLOCK --",
    };
    write!(f, "{}", s)
  }
}

pub struct Window {
  pub buffer: Buffer,
  pub mode: Mode,
  pub lines: usize,
  pub offset: u16,
  pub scroll: usize,
  pub width:  u16,
  pub height: u16,
}

impl Window {
  pub fn from(mut buffer: Buffer) -> Window {
    let (width, height) = terminal_size()
      .expect("Could not get terminal size");
    let lines = buffer.content.len_lines();
    let offset = lines.len_digits();
    buffer.set_cursor(1, 1, offset);
    Window {
      buffer: buffer,
      mode: Mode::Command,
      lines: lines,
      offset: offset,
      scroll: 1,
      width: width,
      height: height-1, // 2 rows on bottom reserved for command bar
    }
  }

  pub fn display(&mut self) {
    self.buffer.set_cursor(1,1,0);
    let mut ln = self.scroll;
    for line in self.buffer.content.lines_at(self.scroll-1) { //what's confusing is termion stuff is 1-indexed but ropey stuff is 0-indexed. plus termion uses u16 for sizes and ropey uses usize.
      if ln < self.lines && ln < self.scroll + (self.height as usize) {
        write!(self.buffer.context, "{}{} ", color::Fg(color::Yellow), line_num(ln, self.offset)).unwrap();
        write!(self.buffer.context, "{}{}\r", color::Fg(color::Reset), line).unwrap();
        ln += 1;
      }
    }
    self.buffer.set_cursor(1,self.height+1,0);
    write!(self.buffer.context, "{}", self.mode).unwrap();
    // display position of cursor and scroll percantage/top/bot on right side of bottom bar
    self.buffer.set_cursor(1, 1, self.offset+1);
    self.buffer.context.flush().unwrap();
  }
}

fn line_num(ln: usize, offset: u16) -> String {
  let ld = ln.len_digits();
  let mut s = String::new();
  for _ in 0..(offset-ld) {
    s.push(' ');
  }
  format!("{}{}",s,ln)
}
