use std::fmt;
use std::mem;
use crate::charstring::*;

#[derive(Debug)]
enum Node {
  Leaf(CharString),
  Branch(Box<RopeNode>)
}

type NextNode = Option<Node>;

#[derive(Debug)]
struct RopeNode {
  val:   usize,
  left:  NextNode,
  right: NextNode
}

impl RopeNode {
  fn from(cstr: CharString) -> RopeNode {
    RopeNode {
      val: cstr.len(),
      left: Some(Node::Leaf(cstr)),
      right: None
    }
  }

  fn show(&self) -> String {
    let mut s: String = match &self.left {
      Some(Node::Leaf(st)) => st.show(),
      Some(Node::Branch(t)) => t.show(),
      None => String::new()
    };
    s.push_str(&(match &self.right {
      Some(Node::Leaf(st)) => st.show(),
      Some(Node::Branch(t)) => t.show(),
      None => String::new()
    }));
    s
  }

  fn insert_whole(&mut self, place: usize, st: CharString) {
    let mut pl = place;
    let slen = st.len();
    let target = {
      if place <= self.val {
        self.val += slen;
        &mut self.left
      }
      else {
        pl -= self.val;
        &mut self.right
      }
    };
    match target {
      &mut Some(Node::Branch(ref mut n)) => {n.insert_whole(pl,st);},
      &mut Some(Node::Leaf(ref mut s)) => {
        if pl < s.len() {
          for i in 0..st.len() {
            s.insert(pl+i, st[i]);
          }
        }
        else {
          let mut tmp = CharString::empty();
          mem::swap(&mut tmp, s);
          let nrope = RopeNode {val: tmp.len(), left: Some(Node::Leaf(tmp)), right: Some(Node::Leaf(st))};
          *target = Some(Node::Branch(Box::new(nrope)));
          //*target = Some(Node::Branch(Box::new(RopeNode {val: s.len(), left: Some(Node::Leaf(s)), right: Some(Node::Leaf(st))})));
        }
      },
      None => {*target = Some(Node::Leaf(st));}
    }
  }
}

#[derive(Debug)]
struct RopeRoot {
  root: RopeNode,
}

impl RopeRoot {
  fn from(cstr: CharString) -> RopeRoot {
    RopeRoot {root: RopeNode::from(cstr)}
  }

  fn concat(&mut self, nright: RopeRoot) {
    match self.root.right {
      Some(_) => {
        let mut oldroot = RopeNode {val: 0, left: None, right: None};
        mem::swap(&mut self.root, &mut oldroot);
        self.root.val = {
          let mut length = oldroot.val;
          let mut current = &oldroot;
          'lp: loop {
            match &current.right {
              Some(Node::Leaf(s)) => {length += s.len(); break 'lp;},
              Some(Node::Branch(n)) => {length += n.val; current = &n;},
              None => {break 'lp;}
            }
          }
          length
        };
        self.root.left = Some(Node::Branch(Box::new(oldroot)));
        self.root.right = Some(Node::Branch(Box::new(nright.root)));
      },
      None => {
        match nright.root.right {
          Some(_) => {
            self.root.right = Some(Node::Branch(Box::new(nright.root)));
          },
          None => {
            self.root.right = nright.root.left;
          }
        }
      }
    }
  }
  pub fn insert_whole(&mut self, place: usize, s: CharString) {
    self.root.insert_whole(place, s);
  }
}

impl fmt::Display for RopeRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root.show())
    }
}

#[derive(Debug)]
pub struct Rope {
  len: usize,
  rope: RopeRoot
}

impl Rope {
  pub fn single(cstr: CharString) -> Rope {
    let rp = RopeRoot {root: RopeNode::from(cstr)};
    let len = rp.root.val;
    Rope{len: len, rope: rp}
  }

  pub fn from(s: CharString) -> Rope {
    let mut ixs: Vec<usize> = vec![];
    for i in 1..(s.len()) { //if first elem(0) is whitespace then it will be part of first word
      if !s[i].is_alphanumeric() {
        ixs.push(i);
      }
    }
    if ixs.len() == 0 {
      Rope::single(s)
    }
    else {
      let (left, _) = s.val.split_at(ixs[0]);
      let mut res = Rope::single(CharString(left.to_vec()));
      for i in 1..ixs.len() {
        let (_,r) = s.val.split_at(ixs[i-1]);
        let (ns, _r) = r.split_at(ixs[i]-ixs[i-1]);
        res.concat(Rope::single(CharString(ns.to_vec()))); //replace with Rope::insert_whole
        if i == ixs.len()-1 {
          res.concat(Rope::single(CharString(_r.to_vec())));
        }
      }
      res
    }
  }

  pub fn append(&mut self, s: CharString) {
    self.concat(Rope::from(s));
  }

  pub fn insert_whole(&mut self, place: usize, s: CharString) {
    self.len += s.len();
    self.rope.insert_whole(place, s);
  }

  pub fn insert(&mut self, place: usize, s: CharString) {
    let slen = s.len();
    let mut ixs: Vec<usize> = vec![];
    for i in 1..(s.len()) { //if first elem(0) is whitespace then it will be part of first word
      if !s[i].is_alphanumeric() {
        ixs.push(i);
      }
    }
    if ixs.len() == 0 {
      //
    }
    else {
      //
    }
  }

  pub fn insert_ch(&mut self, place: usize, c: char) {

  }

  pub fn concat(&mut self, node: Rope) {
    self.len += node.len;
    self.rope.concat(node.rope);
  }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rope.fmt(f)
    }
}
