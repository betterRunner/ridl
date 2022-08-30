#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
  Line,
  Block,
  Tail,
}
