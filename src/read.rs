use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{Error, ErrorKind};

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
        let temp_main = File::open(fp)?;
        let swap_fps = format!("{}{}",fps,(".swp"));
        let swap_fp = Path::new(&swap_fps);
        if swap_fp.exists() {
          Err(Error::new(ErrorKind::Other, "Swap file already exists")) //TODO: add existing-swap handling
        }
        else {
          let temp_swap = File::create(swap_fp)?;
          fs::copy(fp, swap_fp)?;
          Ok(RFile{name: fps, main: Some(temp_main), swap: temp_swap})
        }
      }
      else if fp.exists() { // means the entity exists but is not a file
        Err(Error::new(ErrorKind::Other, "Cannot open directory"))
      }
      else { // means we are creating a new file, main file will be created on first save
        let temp = File::create(Path::new( &format!("{}{}",fps,(".swp")) ))?;
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
      let temp = File::create(Path::new(&format!("{}{}",empty,".swp")))?;
      Ok(RFile{name: empty, main: None, swap: temp})
    },
  }
}


// pub fn read_file
