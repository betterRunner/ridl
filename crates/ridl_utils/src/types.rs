use std::collections::HashMap;

use crate::code_block::CodeBlock;

/// Commons
pub enum IdlProtocolType {
  Thrift,
}

impl IdlProtocolType {
  pub fn as_str(&self) -> &'static str {
    match self {
      IdlProtocolType::Thrift => ".thrift",
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum IdlType {
  Interface,
  Enum,
}

impl IdlType {
  pub fn as_str(&self) -> &'static str {
    match self {
      IdlType::Interface => "struct",
      IdlType::Enum => "enum",
    }
  }
}

/// Blocks
pub type IdlBlocksMap = HashMap<IdlType, Vec<CodeBlock>>;
pub type NamespaceIdlBlocksMap = HashMap<String, IdlBlocksMap>;
