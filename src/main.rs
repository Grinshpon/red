extern crate termion;

#[macro_use]
pub mod util;

use termion::*;
use termion::screen::*;

use std::io::{Write, stdout, stdin};
use std::env;
use std::fs;
use std::path::Path;

pub mod window;
use crate::window::*;

pub mod input;

pub mod read;
use crate::read::*;

macro_rules! moveCursor {
  ( $scr:expr, $x:expr, $y:expr ) => {{
    write!($scr,"{}",cursor::Goto($x,$y)).unwrap();
  }}
}



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
    Ok(mut f) => {
      // init(..)
      //...read_file ...
      //...
      let mut main_win = read_file(&mut f);

      //program finishes
      fs::remove_file( &Path::new(&format!("{}.swp",f.name)) )?;
    }
    Err(err) => {
      println!("Error opening file: {}", err);
    }
  }
  Ok(())
}
/*
fn main() {
  let args: Vec<String> = env::args().collect();
  let file = {
    if args.len() > 1 {
      let fpath = Path::new(&args[1]);
      match fs::read_to_string(fpath) {
        Ok(f) => Some(f),
        Err(err) => {
          println!("Could not read file: {}", err);
          None
        },
      }
    } else {None}
  };
  let _deletedis = Bar::new();
  let mut input = String::new();
  let size = terminal_size()
    .expect("Can not get terminal size");
  {
    let mut screen = AlternateScreen::from(stdout());
    moveCursor!(screen, 1,1);
    match file {
      Some(content) => {write!(screen, "{}", content).unwrap();},
      _ => {},
    }
    write!(screen, "Screen size: {:?}\n\nWrite something and press enter: ", size).unwrap();
    screen.flush().unwrap();
    stdin().read_line(&mut input)
      .expect("Could not read input");
  }

  println!("{}",input);
}
*/
