
#[macro_use]
pub mod util;

extern crate termion;
use termion::cursor;
use termion::screen::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

use std::io::{Write, stdout, stdin};
use std::env;
use std::fs;
use std::path::Path;

pub mod window;
use crate::window::*;

pub mod input;
use crate::input::*;

pub mod read;
use crate::read::*;

pub mod buffer;
use crate::buffer::*;

pub mod charstring;
//use crate::charstring::*;

pub mod rope;
//use crate::rope::*;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let file = {
    if args.len() > 1 {
      open_file(Some(args[1].clone()))
    }
    else {
      open_file(None)
    }
  };
  match file {
    Ok((name, mut f, mut df)) => {
      // init(..)
      //...read_file ...
      //...
      let mut screen = Box::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()));
      //write!(screen,"{}",cursor::Goto(1,1)).unwrap();
      let mut buffer = read_file(name.clone(), f, df, screen)?;

      let mut window = Window::from(buffer);
      window.display();

      let stdin = stdin();

      let mut keybindings = KeyBindings::default(); // eventually should read an rc file to change keymap appropriately

      for key in stdin.keys() {
        if perform_action(&keybindings, &mut window, &key.unwrap()) {
          break;
        }
        else {
          window.buffer.context.flush().unwrap();
        }
      }


      //program finishes
      fs::remove_file( &Path::new(&format!("{}.diff", name)) )?;
    }
    Err(err) => {
      println!("Error opening file: {}", err);
    }
  }
  //println!("finished");
  Ok(())
}
