use ropey::Rope;

use termion::screen::*;
use termion::cursor::*;
use termion::raw::RawTerminal;


use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::{BufRead, BufReader};
use std::io::{Seek, SeekFrom};
use std::io::{Write, Stdout, stdin};

use crate::window::*;
use crate::util::*;
use crate::buffer::*;

/*
pub struct RFile {
  pub name: String,
  pub main: Option<File>, // opening an empty instance will create a diff file but not a main file
  pub diff: File
}
*/

pub fn open_file(mfp: Option<String>) -> io::Result<(String, Option<File>, File)> {
  match mfp {
    Some(fps) => {
      let fp = Path::new(&fps);
      if fp.is_file() {
        let temp_main = fs::OpenOptions::new()
          .read(true)
          .write(true)
          .open(fp)?;
        let diff_fps = format!("{}{}",fps,(".diff"));
        let diff_fp = Path::new(&diff_fps);
        if diff_fp.exists() {
          Err(Error::new(ErrorKind::Other, "Diff file already exists")) //TODO: add existing-diff handling
        }
        else {
          let temp_diff = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(diff_fp)?;
          //fs::copy(fp, diff_fp)?;
          Ok((fps, Some(temp_main), temp_diff))
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
          .open(Path::new( &format!("{}{}",fps,(".diff")) ))?;
        Ok((fps, None, temp))
      }
    },
    None => {
      let mut empty = String::from("newFile");
      let mut i = 0;
      while (Path::new(&format!("{}{}",empty,".diff")).exists()) {
        empty = format!("{}({}){}{}",empty,i,'.',".diff");
        i += 1;
      }
      let temp = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(Path::new(&format!("{}{}",empty,".diff")))?;
      Ok((empty, None, temp))
    },
  }
}


pub fn read_file(name: String, mfile: Option<File>, mut diff: File, context: Box<AlternateScreen<RawTerminal<Stdout>>>) -> io::Result<Buffer> {
  void!(diff.seek(SeekFrom::Start(0)));
  //let mut win = View();
  let mut buffer = match &mfile {
    Some(file) => Rope::from_reader(
      BufReader::new(file)
    )?,
    None => Rope::new(),
  };
  Ok(Buffer {
    name: name,
    file: mfile,
    diff: diff,
    content: buffer,
    cursor: (1,1),
    selection: None,
    context: context,
  })
}
