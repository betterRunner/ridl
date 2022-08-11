use ridl_utils::fs::{get_all_files_path_of_folder, read_file_str_content};
use ridl_utils::types::{IdlBlocksMap, IdlProtocolType, NamespaceIdlBlocksMap};

mod searcher_thrift;
use searcher_thrift::searcher as searcher_thrift;

type BlockSearcher = fn(source_str: &str) -> (String, IdlBlocksMap);

pub fn iter_and_collect_all_idl_blocks(
  idl_protocol_type: &IdlProtocolType,
  folder_path: &str,
) -> NamespaceIdlBlocksMap {
  // 1. get path of all idl files
  let idl_files_path = get_all_files_path_of_folder(folder_path, idl_protocol_type.as_str());
  let idl_files_path = idl_files_path.lock().unwrap();
  println!("all idl files path: {:?}", idl_files_path);

  // 2. read all these files and find all idl blocks
  // TODO: multi threads to speed up
  let idl_files_content: Vec<String> = idl_files_path
    .iter()
    .map(|s| read_file_str_content(s.as_str()))
    .collect();
  // select the corresponding block runner by idl type
  let searcher: BlockSearcher = match idl_protocol_type {
    IdlProtocolType::Thrift => searcher_thrift,
    // TODO: other idl types.
  };

  // // 3. combine all blocks into namespace_map
  let mut namespace_map = NamespaceIdlBlocksMap::new();
  for source_str in idl_files_content {
    let (namespace, idl_blocks_map) = searcher(source_str.as_str());
    let ori_map = IdlBlocksMap::new();
    let ori_map = namespace_map.get(namespace.as_str()).unwrap_or(&ori_map);

    let mut merged_map = IdlBlocksMap::new();
    let keys = idl_blocks_map.keys();
    for k in keys {
      // merge two vectors
      let v1 = ori_map.get(k).unwrap_or(&[].to_vec()).clone();
      let v2 = idl_blocks_map.get(k).unwrap_or(&[].to_vec()).clone();
      let v_merged = merged_map.entry(k.clone()).or_insert(v1);
      v_merged.extend(v2);
    }
    namespace_map.insert(namespace, merged_map);
  }
  namespace_map
}
