use std::fmt;
use std::ops::{Index,IndexMut};

#[derive(Debug)]
pub struct CharString {
    pub val: Vec<char>,
}
#[allow(non_snake_case)]
pub fn CharString (s: Vec<char>) -> CharString {
    CharString{val: s}
}

impl CharString {
  pub fn len(&self) -> usize {
    self.val.len()
  }
}

impl fmt::Display for CharString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.val.iter().collect();
        write!(f, "{}", s)
    }
}

impl Index<usize> for CharString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.val[index]
    }
}

impl IndexMut<usize> for CharString {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.val[index]
    }
}