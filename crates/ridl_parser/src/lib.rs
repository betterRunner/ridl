use ridl_utils::types::{IdlBlocksMap, IdlProtocolType, NamespaceIdlBlocksMap};
mod thrift;
use thrift::parser::{parser as parser_thrift, IdlMetaMap, NamespaceIdlMetaMap};

type Parser = fn(blocks_map: &IdlBlocksMap) -> IdlMetaMap;

pub fn parse_idl_metas_from_blocks(
  idl_protocol_type: &IdlProtocolType,
  namespace_idl_blocks_map: &NamespaceIdlBlocksMap,
) -> NamespaceIdlMetaMap {
  let mut idl_metas_map = NamespaceIdlMetaMap::new();

  let parser: Parser = match idl_protocol_type {
    IdlProtocolType::Thrift => parser_thrift,
    // TODO: other idl types.
  };

  for (k, v) in namespace_idl_blocks_map {
    let idl_metas = parser(v);
    idl_metas_map.insert(k.clone(), idl_metas);
  }

  idl_metas_map
}
