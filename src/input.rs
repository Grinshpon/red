extern crate termion;
use termion::event::Key;

//use std::io::{Read, Write};
use std::collections::HashMap;

macro_rules! hashmap {
  ( $( $key: expr => $val:expr ),* ) => {{
    let mut temp = HashMap::new();
    $(
      temp.insert($key,$val);
    )*
    temp
  }}
}

use crate::util::*;
use crate::window::*;

pub struct Input { //will become command
  key: Key,
  position: Pos,
}
//pub type Action = Vec<Input>;
//pub type Done = Vec<Action>; //will become "Done" stack
//pub type Undone = Vec<Action>; //will become "Undone" stack
//pub type Comm = Vec<Action>; //will become command mode input stack

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

#[derive(Debug,Clone,Copy)]
pub enum Action {
  Insert,
  Visual,
  Delete,
  VisualBlock,
  Replace,
  ExitMode,
  Word,
  Up,
  Down,
  Left,
  Right,
  Undo,
  Redo,
  Write,
  Quit,
  ModNum(u16),
}

pub type ActionChain = Vec<Action>; //concept? So "d3w" becomes [Delete, ModNum(3), Word]

pub struct KeyMap(HashMap<Key,Action>);

impl KeyMap {
  pub fn default() -> KeyMap {
    KeyMap(hashmap! {
      Key::Char('\n') => Action::Quit, //PLACEHOLDER
//    Key::Char('h') => Action::Left,
//    Key::Char('l') => Action::Right,
//    Key::Char('k') => Action::Up,
//    Key::Char('j') => Action::Down,
      Key::Left      => Action::Left,
      Key::Right     => Action::Right,
      Key::Up        => Action::Up,
      Key::Down      => Action::Down,
      Key::Esc       => Action::ExitMode,
      Key::Char('i') => Action::Insert,
      Key::Char('v') => Action::Visual
    })
  }
}

pub fn perform_action(keymap: &KeyMap, window: &mut Window, key: &Key) -> bool {
  let mut quit = false;
  match keymap.0.get(key) {
    None => { },
    Some(action) => match action {
      Action::Quit => { quit = true; },
      Action::ExitMode => { window.mode = Mode::Command; window.clear(); window.display(); },
      Action::Insert => { window.mode = Mode::Insert; window.display(); },
      Action::Left => {
        let (x,y) = window.buffer.cursor;
        if x > 1 {
          window.buffer.set_cursor(x-1,y, window.offset+1);
        }
      }
      Action::Right => {
        let (x,y) = window.buffer.cursor;
        if x >= 1 {
          window.buffer.set_cursor(x+1,y, window.offset+1);
        }
      }
      _ => { }
    }
  }
  quit
}
