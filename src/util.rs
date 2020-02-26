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

#[macro_export]
macro_rules! setCursor {
  ( $scr:expr, $x:expr, $y:expr ) => {{
    write!($scr,"{}",cursor::Goto($x,$y)).unwrap();
  }}
}


