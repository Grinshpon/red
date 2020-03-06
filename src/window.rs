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


pub mod bottom_bar;
pub use bottom_bar::Bar;

pub mod line_num;
pub use line_num::LineN;

//pub mod tabs;
//pub use tabs::TabBar;

use crate::util::*;
use crate::buffer::*;

use std::io::Write;

pub enum Mode {
  Normal,
  Command,
  Insert,
  Select,
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
    buffer.set_cursor(offset, 1);
    Window {
      buffer: buffer,
      mode: Mode::Command,
      lines: lines,
      offset: offset,
      scroll: 1,
      width: width,
      height: height,
    }
  }

  pub fn display(&mut self) {
    self.buffer.set_cursor(1,1);
    let mut ln = self.scroll;
    for line in self.buffer.content.lines() {
      if ln < self.lines {
        write!(self.buffer.context, "{}{} ", color::Fg(color::Yellow), line_num(ln, self.offset)).unwrap();
        write!(self.buffer.context, "{}{}\r", color::Fg(color::Reset), line).unwrap();
        ln += 1;
      }
    }
    self.buffer.set_cursor(2+self.offset+self.buffer.cursor.0,self.buffer.cursor.1);
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
