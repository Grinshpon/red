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

//pub mod util;

pub mod bottom_bar;
pub use bottom_bar::Bar;

pub mod line_num;
pub use line_num::LineN;

pub mod tabs;
pub use tabs::TabBar;

use crate::util::Pos;

pub type Contents = Vec<String>;

/*
pub type Line = String; // or Vec<Word> where Word = String
pub type Paragraph = Vec<Line>;
pub type Contents = Vec<Paragraph>;
*/
// this should be turned into a rope type structure.
/*
  idea:
    content
    |- paragraphs
       |- lines
          |- words?
*/

/*
impl <String>Display for Vec<String> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f
*/

enum Mode {
  Normal,
  Command,
  Insert,
  Select,
}

pub struct View {
  pub x: u16,
  pub y: u16,
  pub cursor: Pos,
  pub contents: Contents,
}
pub fn View() -> View {
  View {
    x: 0,
    y: 0,
    cursor: (0,0),
    contents: Vec::new(),
  }
}
