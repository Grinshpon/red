use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::{BufRead, BufReader};
use std::io::{Seek, SeekFrom};

use crate::window::*;

use crate::util::*;

pub struct RFile {
  pub name: String,
  pub main: Option<File>, // opening an empty instance will create a swap file but not a main file
  pub swap: File
}

pub fn open_file(mfp: Option<String>) -> io::Result<RFile> {
  match mfp {
    Some(fps) => {
      let fp = Path::new(&fps);
      if fp.is_file() {
        let temp_main = fs::OpenOptions::new()
          .read(true)
          .write(true)
          .open(fp)?;
        let swap_fps = format!("{}{}",fps,(".swp"));
        let swap_fp = Path::new(&swap_fps);
        if swap_fp.exists() {
          Err(Error::new(ErrorKind::Other, "Swap file already exists")) //TODO: add existing-swap handling
        }
        else {
          let temp_swap = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(swap_fp)?;
          fs::copy(fp, swap_fp)?;
          Ok(RFile{name: fps, main: Some(temp_main), swap: temp_swap})
        }
      }
      else if fp.exists() { // means the entity exists but is not a file
        Err(Error::new(ErrorKind::Other, "Cannot open directory"))
      }
      else { // means we are creating a new file, main file will be created on first save
        let temp = fs::OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .open(Path::new( &format!("{}{}",fps,(".swp")) ))?;
        Ok(RFile{name: fps, main: None, swap: temp})
      }
    },
    None => {
      let mut empty = String::from("newFile");
      let mut i = 0;
      while (Path::new(&format!("{}{}",empty,".swp")).exists()) {
        empty = format!("{}({}){}{}",empty,i,'.',".swp");
        i += 1;
      }
      let temp = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(Path::new(&format!("{}{}",empty,".swp")))?;
      Ok(RFile{name: empty, main: None, swap: temp})
    },
  }
}


pub fn read_file(rfile: &mut RFile) -> View { //try iterator?
  rfile.swap.seek(SeekFrom::Start(0));
  let mut win = View();
  let mut bufR = BufReader::new(&rfile.swap); //may replace with with_capacity for large files
  let mut eof = false;
  let mut line_num = 0;
/*
  while !eof {
    win.contents.push({
      let mut tmp = vec![];
      bufR.read_until(b'\n', &mut tmp); //will handle result later
      tmp
    });
    if win.contents[line_num].is_empty() {
      eof = true;
      //void!(win.contents.pop());
    }
    else {
      line_num += 1;
    }
  }
*/

  for mline in bufR.lines() {
    match mline {
      Ok(line) => {
        win.contents.push(line);
        //temporary
        println!("{}",win.contents[line_num]);
        line_num += 1;
      },
      Err(_) => {}, //todo: add error handling
    }
  }

  win
}
