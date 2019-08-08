pub struct Bar {
  command_mode: bool,
  left_aligned: String,
  right_aligned: String,
}

impl Bar {
  pub fn new() -> Bar {
    Bar {
      command_mode: false,
      left_aligned: String::new(),
      right_aligned: String::new(),
    }
  }
  fn left_text (val: String) {}
  fn right_text (val: String) {}
}
