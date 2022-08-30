use std::collections::HashMap;

use super::field_comment::Comment;
use super::field_type_interface::InterfaceFieldType;
use regex::Regex;
use ridl_utils::types::{IdlBlocksMap, IdlType};

/// The regex to find the field of idl
struct IdlFieldRe {
  re: Regex,
}

impl From<IdlType> for IdlFieldRe {
  fn from(idl_type: IdlType) -> Self {
    IdlFieldRe {
      re: match idl_type {
        IdlType::Enum => Regex::new(r"\s*([A-Z][A-Za-z0-9]*) = (\d+)").unwrap(),
        IdlType::Interface => Regex::new(
          r"\s*\d+:\s+(?:(optional|required)\s+)?((?:\w|\.)+|(?:map|list)<.+>)\s+(\w+)(?:\s|\(|,|;|$)",
        )
        .unwrap(),
      },
    }
  }
}

#[derive(Debug, Clone)]
pub struct InterfaceField {
  pub name: String,
  pub optional: bool,
  pub r#type: InterfaceFieldType,
  pub comment: Option<Comment>,
}

#[derive(Debug, Clone)]
pub struct EnumField {
  pub key: String,
  pub value: String,
}

#[derive(Debug, Clone)]
pub enum IdlMetaField {
  Interface(InterfaceField),
  Enum(EnumField),
}

#[derive(Debug, Clone)]
pub struct IdlMeta {
  pub name: String,
  pub fields: Vec<IdlMetaField>,
}

pub type IdlMetaMap = HashMap<IdlType, Vec<IdlMeta>>;
pub type NamespaceIdlMetaMap = HashMap<String, IdlMetaMap>;

// Iterator lines and use regex to parse field meta from each line.
fn parse_field_from_line(itype: IdlType, lines: &Vec<String>) -> Vec<IdlMetaField> {
  let re = IdlFieldRe::from(itype.clone()).re;
  lines
    .iter()
    .map(|line| {
      let field = match itype {
        IdlType::Enum => match re.captures(line.as_str()) {
          Some(captures) => Some(IdlMetaField::Enum(EnumField {
            key: captures.get(1).map_or("", |m| m.as_str()).to_string(),
            value: captures.get(2).map_or("", |m| m.as_str()).to_string(),
          })),
          None => None,
        },
        IdlType::Interface => match re.captures(line.as_str()) {
          Some(captures) => Some(IdlMetaField::Interface(InterfaceField {
            name: captures.get(3).map_or("", |m| m.as_str()).to_string(),
            r#type: InterfaceFieldType::from_str(captures.get(2).unwrap().as_str()),
            optional: captures.get(1).map_or("", |m| m.as_str()).eq("optional"),
            comment: None,
          })),
          None => None,
        },
      };
      field
    })
    .filter(|f| f.is_some())
    .map(|f| f.unwrap())
    .collect()
}

pub fn parser(blocks_map: &IdlBlocksMap) -> IdlMetaMap {
  let mut res = IdlMetaMap::new();
  for (k, v) in blocks_map {
    let idl_metas = v
      .iter()
      .map(|b| {
        let fields = parse_field_from_line(k.clone(), &b.lines);
        IdlMeta {
          name: b.name.clone(), // IdlMeta's name is equal to CodeBlock's name.
          fields,
        }
      })
      .collect();
    res.insert(k.clone(), idl_metas);
  }
  res
}

mod test {
  use crate::thrift::{
    field_type_interface::{InterfaceFieldRefer, InterfaceFieldType},
    parser::{EnumField, IdlMetaField},
  };

  use super::parser;
  use ridl_utils::{
    code_block::CodeBlock,
    types::{IdlBlocksMap, IdlType},
  };

  #[test]
  fn test_parser() {
    let mut blocks_map = IdlBlocksMap::new();
    blocks_map.insert(
      IdlType::Interface,
      [CodeBlock {
        name: "interface1".to_string(),
        lines: [
          "struct AddCourseScheduleRulesReq {",
          "1: i64 term_id",
          "2: list<ScheduleRule> rules",
          "",
          "30: i64 org_id //orgid",
          "31: i64 job_key",
          "32: i64 user_id",
          "255: optional base.Base Base",
          "}",
        ]
        .map(|s| s.to_string())
        .to_vec(),
      }]
      .to_vec(),
    );
    blocks_map.insert(
      IdlType::Enum,
      [CodeBlock {
        name: "enum1".to_string(),
        lines: [
          "enum ScheduleRuleType {",
          "    Soft = 0",
          "    Hard = 1",
          "}",
        ]
        .map(|s| s.to_string())
        .to_vec(),
      }]
      .to_vec(),
    );
    let idl_meta_map = parser(&blocks_map);
    // enum
    let idl_metas_enum = idl_meta_map.get(&IdlType::Enum).unwrap();
    assert_eq!(idl_metas_enum.len(), 1);
    assert_eq!(idl_metas_enum[0].name, "enum1");
    let enum_fields = &idl_metas_enum[0].fields;
    assert_eq!(enum_fields.len(), 2);
    fn test_enum_field(field: &IdlMetaField, key: &str, value: &str) {
      match field {
        IdlMetaField::Enum(f) => {
          assert_eq!(f.key, key);
          assert_eq!(f.value, value);
        }
        IdlMetaField::Interface(_) => {}
      }
    }
    test_enum_field(&enum_fields[0], "Soft", "0");
    test_enum_field(&enum_fields[1], "Hard", "1");

    // interface
    let idl_metas_interface = idl_meta_map.get(&IdlType::Interface).unwrap();
    assert_eq!(idl_metas_interface.len(), 1);
    assert_eq!(idl_metas_interface[0].name, "interface1");
    let interface_fields = &idl_metas_interface[0].fields;
    assert_eq!(interface_fields.len(), 6);
    fn test_interface_field(
      field: &IdlMetaField,
      name: &str,
      optional: bool,
      r#type: InterfaceFieldType,
    ) {
      match field {
        IdlMetaField::Enum(_) => {}
        IdlMetaField::Interface(f) => {
          assert_eq!(f.name, name);
          assert_eq!(f.optional, optional);
          assert_eq!(f.r#type, r#type);
          assert_eq!(f.comment, None);
        }
      }
    }
    test_interface_field(
      &interface_fields[0],
      "term_id",
      false,
      InterfaceFieldType::String,
    );
    test_interface_field(
      &interface_fields[1],
      "rules",
      false,
      InterfaceFieldType::Refers(InterfaceFieldRefer {
        refer: "ScheduleRule".to_string(),
        namespace: [].to_vec(),
      }),
    );
    test_interface_field(
      &interface_fields[2],
      "org_id",
      false,
      InterfaceFieldType::String,
    );
    test_interface_field(
      &interface_fields[3],
      "job_key",
      false,
      InterfaceFieldType::String,
    );
    test_interface_field(
      &interface_fields[4],
      "user_id",
      false,
      InterfaceFieldType::String,
    );
    test_interface_field(
      &interface_fields[5],
      "Base",
      true,
      InterfaceFieldType::Refer(InterfaceFieldRefer {
        refer: "Base".to_string(),
        namespace: ["base".to_string()].to_vec(),
      }),
    );
  }
}
