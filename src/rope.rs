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
}

#[derive(Debug)]
struct RopeRoot {
  root: RopeNode,
}

impl RopeRoot {
  fn from(cstr: CharString) -> RopeRoot {
    RopeRoot {root: RopeNode::from(cstr)}
  }

  fn append(&mut self, nright: RopeRoot) {
    match self.root.right {
      Some(_) => {
        let mut oldroot = RopeNode {val: 0, left: None, right: None};
        mem::swap(&mut self.root, &mut oldroot);
        self.root.val = oldroot.val;
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

#[derive(Debug)]
pub struct Rope {
  rope: RopeRoot,
  len: usize
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
