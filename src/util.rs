pub type Pos = (u16, u16);
pub struct Char;

#[macro_export]
macro_rules! void {
  ( $f:expr ) => {{
    let _ = $f;
  }}
}

#[macro_export]
macro_rules! get {
  ( $map:expr, $key:expr ) => {(
    $map.get_mut($key).unwrap()
  )}
}

pub trait Digits {
  fn len_digits(self) -> u16;
}

impl Digits for usize {
  fn len_digits(self) -> u16 {
    if self < 10 {
      1
    }
    else if self < 100 {
      2
    }
    else if self < 1000 {
      3
    }
    else {
      let mut n = self / 10;
      if n > 0 {
        1 + n.len_digits()
      }
      else { 0 }
    }
  }
}
