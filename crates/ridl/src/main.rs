use ridl_block::iter_and_collect_all_idl_blocks;
use ridl_parser::parse_idl_metas_from_blocks;
use ridl_utils::types::{IdlProtocolType};

fn main() {
  let protocol_type = IdlProtocolType::Thrift;
  // Block
  let namespace_blocks_map =
    iter_and_collect_all_idl_blocks(&protocol_type, "./playground/0-parser/demo");
  // println!("idl blocks: {:?}", namespace_blocks_map);

  // Parser
  parse_idl_metas_from_blocks(&protocol_type, &namespace_blocks_map);

  // Codegen
}
