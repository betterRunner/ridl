use regex::Regex;

use ridl_utils::code_block::{search_blocks_from_source, CodeBlock};
use ridl_utils::types::{IdlBlocksMap, IdlType};

fn parse_namespace_by_re(source_str: &str) -> String {
  let RE = Regex::new(r"(?:^|\n)namespace go ([\w\.]+)\n").unwrap();
  match RE.captures(source_str) {
    Some(c) => c.get(1).unwrap().as_str().to_string(),
    None => String::from(""),
  }
}

pub fn searcher(source_str: &str) -> (String, IdlBlocksMap) {
  let mut cbsMap = IdlBlocksMap::new();

  let namespace = parse_namespace_by_re(source_str);
  println!("namespace {}", namespace);

  // 1. searching interface blocks
  println!("searching interface blocks..");
  let re_interface = Regex::new(IdlType::Interface.as_str()).unwrap();
  let source = source_str;
  let interface_blocks: Vec<CodeBlock> =
    search_blocks_from_source(source, re_interface, None, None, None);
  cbsMap.insert(IdlType::Interface, interface_blocks);

  // 2. searching enum blocks
  println!("searching enum blocks..");
  let re_enum = Regex::new(IdlType::Enum.as_str()).unwrap();
  let enum_blocks = search_blocks_from_source(source_str, re_enum, None, None, None);
  cbsMap.insert(IdlType::Enum, enum_blocks);

  (namespace, cbsMap)
}
