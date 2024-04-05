#[derive(PartialEq)]
#[allow(dead_code)]
enum NodeEnum {
  Number,
}

#[derive(Clone, Copy)]
pub enum Node {
  Number(i32),
}

pub fn as_number(node: Node) -> i32 {
  match node {
    Node::Number(n) => n,
  }
}