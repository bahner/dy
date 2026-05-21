use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

/// Recursively convert a serde_yaml Value into a serde_json Value.
pub fn yaml_to_json(yaml: YamlValue) -> JsonValue {
    match yaml {
        YamlValue::Null => JsonValue::Null,
        YamlValue::Bool(b) => JsonValue::Bool(b),
        YamlValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                JsonValue::Number(i.into())
            } else if let Some(u) = n.as_u64() {
                JsonValue::Number(u.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(JsonValue::Number)
                    .unwrap_or(JsonValue::Null)
            } else {
                JsonValue::Null
            }
        }
        YamlValue::String(s) => JsonValue::String(s),
        YamlValue::Sequence(seq) => JsonValue::Array(seq.into_iter().map(yaml_to_json).collect()),
        YamlValue::Mapping(map) => {
            let obj = map
                .into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        YamlValue::String(s) => s,
                        YamlValue::Number(n) => n.to_string(),
                        YamlValue::Bool(b) => b.to_string(),
                        other => format!("{other:?}"),
                    };
                    (key, yaml_to_json(v))
                })
                .collect();
            JsonValue::Object(obj)
        }
        YamlValue::Tagged(tagged) => yaml_to_json(tagged.value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::from_str;

    #[test]
    fn test_simple_mapping() {
        let y: YamlValue = from_str("foo: bar\nnum: 42").unwrap();
        let j = yaml_to_json(y);
        assert_eq!(j["foo"], "bar");
        assert_eq!(j["num"], 42);
    }

    #[test]
    fn test_nested() {
        let y: YamlValue = from_str("a:\n  b:\n    - 1\n    - 2").unwrap();
        let j = yaml_to_json(y);
        assert_eq!(j["a"]["b"][0], 1);
        assert_eq!(j["a"]["b"][1], 2);
    }

    #[test]
    fn test_null_and_bool() {
        let y: YamlValue = from_str("x: ~\nflag: true").unwrap();
        let j = yaml_to_json(y);
        assert_eq!(j["x"], JsonValue::Null);
        assert_eq!(j["flag"], true);
    }
}
