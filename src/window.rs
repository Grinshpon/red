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

pub mod bottom_bar;
pub use bottom_bar::Bar;

pub mod line_num;
pub use line_num::LineN;

pub mod tabs;
pub use tabs::TabBar;

enum Mode {
  Normal,
  Command,
  Insert,
  Select,
}

pub struct Main {
  x: u16,
  y: u16,
  cursor: (u16, u16),
  contents: String,
}