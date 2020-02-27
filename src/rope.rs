use std::fmt;
use std::mem;
use crate::charstring::*;

#[derive(Debug)]
enum Either<E,A> {
  Left(E),
  Right(A)
}

type NextNode = Option<Either<CharString, Box<RopeNode>>>;

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
      left: Some(Either::Left(cstr)),
      right: None
    }
  }

  fn show(&self) -> String {
    let mut s: String = match &self.left {
      Some(Either::Left(st)) => st.show(),
      Some(Either::Right(t)) => t.show(),
      None => String::new()
    };
    s.push_str(&(match &self.right {
      Some(Either::Left(st)) => st.show(),
      Some(Either::Right(t)) => t.show(),
      None => String::new()
    }));
    s
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
              Some(Either::Left(s)) => {length += s.len(); break 'lp;},
              Some(Either::Right(n)) => {length += n.val; current = &n;},
              None => {break 'lp;}
            }
          }
          length
        };
        self.root.left = Some(Either::Right(Box::new(oldroot)));
        self.root.right = Some(Either::Right(Box::new(nright.root)));
      },
      None => {
        match nright.root.right {
          Some(_) => {
            self.root.right = Some(Either::Right(Box::new(nright.root)));
          },
          None => {
            self.root.right = nright.root.left;
          }
        }
      }
    }
  }
}

impl fmt::Display for RopeRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root.show())
    }
}

#[derive(Debug)]
pub struct Rope {
  rope: RopeRoot,
  len: usize
}

impl Rope {
  pub fn single(cstr: CharString) -> Rope {
    let rp = RopeRoot {root: RopeNode::from(cstr)};
    let len = rp.root.val;
    Rope{rope: rp, len: len}
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
        res.concat(Rope::single(CharString(ns.to_vec()))); //replace with Rope::insert
        if i == ixs.len()-1 {
          res.concat(Rope::single(CharString(_r.to_vec())));
        }
      }
      res
    }
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

/*
RopeNode::Leaf(s) => {
  let size = s.len;
  let nleft = RopeNode::Leaf(s);
  self.root = RopeNode::Node { val: size, left: Some(Box::new(nleft)), right: Some(Box::new(nright.root)) };
}
*/

/*
Rope {
  root: RopeNode::Node {
    val: cstr.len,
    left: Some(Box::new(RopeNode::Leaf(cstr))),
    right: None
  }
}
*/

/*
let mut root = RopeNode::Node {val: 0, left: None, right: None};
let right = (match &self.root {
  RopeNode::Node {val,left,right} => Some(right),
  RopeNode::Leaf(_) => None,
}).unwrap();
match right {
  Some(rnode) => {}
  None => {
    self.root.right = nright;
  }
}
*/
