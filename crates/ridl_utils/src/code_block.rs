use regex::Regex;

#[derive(Debug, Clone)]
pub struct CodeBlock {
  pub name: String,
  pub lines: Vec<String>,
}

pub fn search_blocks_from_source(
  source_str: &str,
  re_head: Regex,
  re_edge_left: Option<Regex>,
  re_edge_right: Option<Regex>,
  is_exclude_comment: Option<bool>,
) -> Vec<CodeBlock> {
  // default parameters
  let re_edge_left = re_edge_left.unwrap_or(Regex::new(r"\{").unwrap());
  let re_edge_right = re_edge_right.unwrap_or(Regex::new(r"\}").unwrap());
  let is_exclude_comment = is_exclude_comment.unwrap_or(true);

  // reg
  let re_comment_block_beg = Regex::new(r"^\s*/\*").unwrap();
  let re_comment_block_end = Regex::new(r"\*/\s*$").unwrap();
  let re_comment_line = Regex::new(r"^\s*//\s*(.*?)").unwrap();

  let mut in_block = false;
  let mut in_common_block = false;
  let mut opened_edges_cnt = 0;

  let mut blocks: Vec<CodeBlock> = Vec::new();
  let lines = source_str.lines();
  for line in lines {
    // has not found the block head
    if !in_block {
      // is match the block head
      in_block = re_head.is_match(line);
      if in_block {
        blocks.push(CodeBlock {
          name: get_name_from_block_head(line, &re_head, &re_edge_left),
          lines: Vec::new(),
        })
      }
    }

    // has found the block head
    if in_block {
      // need to skip the comment lines
      if is_exclude_comment {
        // 1. skips the lines of comment block
        let is_comment_beg = re_comment_block_beg.is_match(line);
        let is_comment_end = re_comment_block_end.is_match(line);
        if is_comment_beg || (in_common_block && is_comment_end) {
          in_common_block = true;
          if is_comment_end {
            in_common_block = false;
          }
          continue;
        }
        if in_common_block {
          continue;
        }

        // 2.skips the lines of comment line
        if re_comment_line.is_match(line) {
          continue;
        }
      }

      // push the current line into the block while maintaining a count that records the number of
      // left edges kept opened to make sure the block quit properly.
      blocks.last_mut().unwrap().lines.push(line.to_string());
      if re_edge_left.is_match(line) {
        opened_edges_cnt += 1;
      }
      if re_edge_right.is_match(line) {
        opened_edges_cnt -= 1;
        if opened_edges_cnt == 0 {
          in_block = false;
          continue;
        }
      }
    }
  }

  // if at the end still in the block, means it is not quit properly, should drop it.
  if in_block {
    blocks.pop();
  }
  blocks
}

fn get_name_from_block_head(line: &str, re_head: &Regex, re_edge_left: &Regex) -> String {
  let head_idx = re_head.find(line).unwrap().end();
  let edge_left_idx = match re_edge_left.find(line) {
    Some(m) => m.start(),
    None => line.len(),
  };
  if head_idx < edge_left_idx {
    line[head_idx..edge_left_idx].trim().into()
  } else {
    "".into()
  }
}

#[test]
fn test_no_blocks() {
  let source = r#"function struct() {
    // ...
  }"#;
  let struct_blocks =
    search_blocks_from_source(source, Regex::new(r"^struct").unwrap(), None, None, None);
  let enum_blocks =
    search_blocks_from_source(source, Regex::new(r"^enum").unwrap(), None, None, None);
  assert_eq!(struct_blocks.len(), 0);
  assert_eq!(enum_blocks.len(), 0);
}

#[test]
fn test_simple_blocks() {
  let source = r#"enum E1 {
    Soft = 0
    Hard = 1
  }

  struct S1 {
    1: i64 foo
    2: i64 bar
    3: i64 baz
  }
  
  enum E2 {
    Soft = 0
    Hard = 1
    Middle = 2
  }"#;
  let struct_blocks =
    search_blocks_from_source(source, Regex::new(r"^struct").unwrap(), None, None, None);
  let enum_blocks =
    search_blocks_from_source(source, Regex::new(r"^enum").unwrap(), None, None, None);

  assert_eq!(enum_blocks.len(), 2);
  assert_eq!(enum_blocks[0].name, "E1");
  assert_eq!(enum_blocks[0].lines.len(), 4);
  assert_eq!(enum_blocks[1].name, "E2");
  assert_eq!(enum_blocks[1].lines.len(), 5);

  assert_eq!(struct_blocks.len(), 1);
  assert_eq!(struct_blocks[0].name, "S1");
  assert_eq!(struct_blocks[0].lines.len(), 5);
}

#[test]
fn test_blocks_with_line_comment() {
  let source = r#"enum E1 {// comment line1
    Soft = 0 // comment line2
    Hard = 1/* comment line3 */
  }// comment line4

  struct S1 {   // comment line1
    1: i64 foo
    2: i64 bar /* comment line2 */
    3: i64 baz// comment line3
  }// comment line4
  "#;
  let struct_blocks =
    search_blocks_from_source(source, Regex::new(r"^struct").unwrap(), None, None, None);
  let enum_blocks =
    search_blocks_from_source(source, Regex::new(r"^enum").unwrap(), None, None, None);
  assert_eq!(struct_blocks.len(), 1);
  assert_eq!(struct_blocks[0].name, "S1");
  assert_eq!(struct_blocks[0].lines.len(), 5);
  assert_eq!(enum_blocks.len(), 1);
  assert_eq!(enum_blocks[0].name, "E1");
  assert_eq!(enum_blocks[0].lines.len(), 4);
  println!("struct_blocks {:?}", struct_blocks);
  println!("enum_blocks {:?}", enum_blocks);
}

#[test]
fn test_blocks_with_block_comment() {
  let source = r#"enum E1 {
  /* block comment 1 */
  Soft = 0
  Hard = 1
}/* block comment2 */

struct S1 {
  /**
   * block comment 1
   */
  1: i64 foo
  2: i64 bar
  3: i64 baz
}
"#;
  let struct_blocks =
    search_blocks_from_source(source, Regex::new(r"^struct").unwrap(), None, None, None);
  let enum_blocks =
    search_blocks_from_source(source, Regex::new(r"^enum").unwrap(), None, None, None);
  assert_eq!(struct_blocks.len(), 1);
  assert_eq!(struct_blocks[0].name, "S1");
  assert_eq!(struct_blocks[0].lines.len(), 5); // not 7
  assert_eq!(enum_blocks.len(), 1);
  assert_eq!(enum_blocks[0].name, "E1");
  assert_eq!(enum_blocks[0].lines.len(), 4); // not 5
  println!("struct_blocks {:?}", struct_blocks);
  println!("enum_blocks {:?}", enum_blocks);
}

#[test]
fn test_nested_block() {
  let source = r#"interface I1 {
    foo: String,
    bar: {
      baz: String,
    },
  };"#;
  let blocks =
    search_blocks_from_source(source, Regex::new(r"^interface").unwrap(), None, None, None);
  assert_eq!(blocks.len(), 1);
  assert_eq!(blocks[0].lines.len(), 6);
  println!("nested block: {:?}", blocks);
}

#[test]
fn test_invalid_nested_block() {
  let source = r#"interface I1 {
    foo: String,
    bar: {
      baz: String,
    },
  ;"#;
  let blocks =
    search_blocks_from_source(source, Regex::new(r"^interface").unwrap(), None, None, None);
  assert_eq!(blocks.len(), 0);
}
