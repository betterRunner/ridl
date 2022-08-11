use regex::Regex;

#[derive(Debug, Clone)]
pub enum InterfaceFieldType {
  Number,
  Numbers,
  String,
  Strings,
  Boolean,
  Booleans,
  Refer(InterfaceFieldRefer),
  Refers(InterfaceFieldRefer),
  Map(InterfaceFieldMap),
  Unknown,
}

#[derive(Debug, Clone)]
pub struct InterfaceFieldRefer {
  pub refer: String,
  pub namespace: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InterfaceFieldMap {
  pub key: Box<InterfaceFieldType>,
  pub value: Box<InterfaceFieldType>,
}

impl InterfaceFieldType {
  pub fn from_str(s: &str) -> InterfaceFieldType {
    let re_refer = r"((?:[A-Za-z]+\.)*(?:[A-Z][a-z]+)+)";
    let re_refers = format!("list<{}>", re_refer);
    let re_refers = re_refers.as_str();
    let re_map = r"map<(.*?),(.*?)>";

    let res = match s {
      "i32" => InterfaceFieldType::Number,
      "list<i32>" => InterfaceFieldType::Numbers,
      "i64" | "string" => InterfaceFieldType::String,
      "list<i64>" | "list<string>" => InterfaceFieldType::Strings,
      "bool" => InterfaceFieldType::Boolean,
      "list<bool>" => InterfaceFieldType::Booleans,
      _ => {
        fn split_refer_text(refer: String) -> (Vec<String>, String) {
          let parts: Vec<String> = refer.split(".").map(|f| f.to_string()).collect();
          (
            parts[0..parts.len() - 1].to_vec(),
            parts[parts.len() - 1].clone(),
          )
        }
        fn get_refer_capture(s: &str, re: &str) -> InterfaceFieldRefer {
          let text = Regex::new(re)
            .unwrap()
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();
          let (namespace, refer) = split_refer_text(text);
          InterfaceFieldRefer { namespace, refer }
        }

        fn get_map_capture(s: &str, re: &str) -> (String, String) {
          let captures = Regex::new(re).unwrap().captures(s).unwrap();
          let key = captures.get(1).unwrap().as_str().trim();
          let value = captures.get(2).unwrap().as_str().trim();
          return (key.to_string(), value.to_string());
        }

        if Regex::new(re_map).unwrap().is_match(s) {
          let (key, value) = get_map_capture(s, re_map);
          InterfaceFieldType::Map(InterfaceFieldMap {
            key: Box::new(InterfaceFieldType::from_str(key.as_str())),
            value: Box::new(InterfaceFieldType::from_str(value.as_str())),
          })
        } else if Regex::new(re_refers).unwrap().is_match(s) {
          let res = get_refer_capture(s, re_refers);
          InterfaceFieldType::Refers(res)
        } else if Regex::new(re_refer).unwrap().is_match(s) {
          let res = get_refer_capture(s, re_refer);
          InterfaceFieldType::Refer(res)
        } else {
          InterfaceFieldType::Unknown
        }
      }
    };
    res
  }

  fn to_field_type_str(&self) -> String {
    match self {
      InterfaceFieldType::Number => "number".to_string(),
      InterfaceFieldType::Numbers => "number[]".to_string(),
      InterfaceFieldType::String => "string".to_string(),
      InterfaceFieldType::Strings => "string[]".to_string(),
      InterfaceFieldType::Boolean => "boolean".to_string(),
      InterfaceFieldType::Booleans => "boolean[]".to_string(),
      InterfaceFieldType::Refer(refer) => refer.refer.clone(),
      InterfaceFieldType::Refers(refer) => format!("{}[]", refer.refer),
      InterfaceFieldType::Map(map) => format!(
        "Record<{}, {}>",
        map.key.to_field_type_str(),
        map.value.to_field_type_str()
      ),
      InterfaceFieldType::Unknown => "".to_string(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::InterfaceFieldType;

  #[test]
  fn test_field_type_basic() {
    let field_type_b = InterfaceFieldType::from_str("bool").to_field_type_str();
    let field_type_bl = InterfaceFieldType::from_str("list<bool>").to_field_type_str();
    let field_type_n = InterfaceFieldType::from_str("i32").to_field_type_str();
    let field_type_nl = InterfaceFieldType::from_str("list<i32>").to_field_type_str();
    let field_type_s1 = InterfaceFieldType::from_str("i64").to_field_type_str();
    let field_type_s2 = InterfaceFieldType::from_str("string").to_field_type_str();
    let field_type_sl1 = InterfaceFieldType::from_str("list<i64>").to_field_type_str();
    let field_type_sl2 = InterfaceFieldType::from_str("list<string>").to_field_type_str();

    assert_eq!(field_type_b, "boolean".to_string());
    assert_eq!(field_type_bl, "boolean[]".to_string());
    assert_eq!(field_type_n, "number".to_string());
    assert_eq!(field_type_nl, "number[]".to_string());
    assert_eq!(field_type_s1, "string".to_string());
    assert_eq!(field_type_s2, "string".to_string());
    assert_eq!(field_type_sl1, "string[]".to_string());
    assert_eq!(field_type_sl2, "string[]".to_string());
  }

  #[test]
  fn test_field_type_refer() {
    fn no_namespace(refer: &str, answer: &str) {
      let refer_single = InterfaceFieldType::from_str(refer);
      assert_eq!(refer_single.to_field_type_str(), answer);
      match refer_single {
        InterfaceFieldType::Refer(refer) => {
          assert_eq!(refer.refer, answer);
          assert_eq!(refer.namespace.len(), 0);
        }
        _ => (),
      }
    }
    no_namespace("ReferObj", "ReferObj");
    no_namespace("list<ReferObj>", "ReferObj[]");

    fn namespace(text: &str, answer: &str, namespace: Vec<&str>) {
      let refer_single = InterfaceFieldType::from_str(text);
      assert_eq!(refer_single.to_field_type_str(), answer);
      match refer_single {
        InterfaceFieldType::Refer(refer) => {
          assert_eq!(refer.refer, answer);
          for (idx, n) in namespace.iter().enumerate() {
            assert_eq!(n.to_string(), refer.namespace[idx]);
          }
        }
        _ => (),
      }
    }
    namespace("foo.bar.ReferObj", "ReferObj", ["foo", "bar"].to_vec());
    namespace(
      "list<foo.bar.ReferObj>",
      "ReferObj[]",
      ["foo", "bar"].to_vec(),
    );
  }

  #[test]
  fn test_field_type_map() {
    fn test(text: &str, answer: &str, key: &str, value: &str) {
      let f = InterfaceFieldType::from_str(text);
      assert_eq!(f.to_field_type_str(), answer.to_string());
      match f {
        InterfaceFieldType::Map(m) => {
          assert_eq!(m.key.to_field_type_str(), key);
          assert_eq!(m.value.to_field_type_str(), value);
        }
        _ => (),
      }
    }
    test(
      "map<string, string>",
      "Record<string, string>",
      "string",
      "string",
    );
    test(
      "map<foo.bar.ReferObj, string>",
      "Record<ReferObj, string>",
      "ReferObj",
      "string",
    );
    test(
      "map<string, foo.bar.ReferObj>",
      "Record<string, ReferObj>",
      "string",
      "ReferObj",
    );
  }
}
