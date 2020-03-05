use std::fmt;
use std::ops::{Index,IndexMut};

#[derive(Debug)]
pub struct CharString(pub Vec<char>);

impl CharString {
  pub fn len(&self) -> usize {
    self.0.len()
  }
  pub fn show(&self) -> String {
    self.0.iter().collect()
  }

  pub fn from(s: &str) -> CharString {
    CharString(s.chars().collect())
  }

  pub fn empty() -> CharString {
    CharString{0: vec![]}
  }

  pub fn insert(&mut self, i: usize, c: char) {
    self.0.insert(i, c);
  }
}

impl fmt::Display for CharString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.0.iter().collect();
        write!(f, "{}", s)
    }
}

impl Index<usize> for CharString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for CharString {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
