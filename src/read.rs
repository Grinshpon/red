use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{Error, ErrorKind};

pub struct RFile {
  main: Option<File>, // opening an empty instance will create a swap file but not a main file
  swap: File
}

pub fn open_file(mfp: Option<&String>) -> io::Result<RFile> {
  match mfp {
    Some(fps) => {
      let fp = Path::new(fps);
      if fp.is_file() {
        let temp_main = File::open(fp)?;
        let temp_swap = File::create(Path::new( &format!("{}{}",fps,(".swp")) ))?;
        Ok(RFile{main: Some(temp_main), swap: temp_swap})
      }
      else if fp.exists() { // means the entity exists but is not a file
        Err(Error::new(ErrorKind::Other, "Cannot open directory"))
      }
      else { // means we are creating a new file
        let temp = File::create(Path::new( &format!("{}{}",fps,(".swp")) ))?;
        Ok(RFile{main: None, swap: temp})
      }
    },
    None => { // opening an empty swp file
      let mut empty = String::from("empty.swp");
      let mut i = 0;
      while (Path::new(&empty).exists()) {
        empty = {
          let tmp: Vec<&str> = empty.split(".").collect();
          format!("{}{}{}",tmp[0],i,tmp[1])
        };
        i += 1;
      }
      let temp = File::create(Path::new(&empty))?;
      Ok(RFile{main: None, swap: temp})
    },
  }
}


// pub fn read_file
