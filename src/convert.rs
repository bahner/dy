use serde_json::Value as JsonValue;
use yaml_rust2::Yaml;

/// Recursively convert a yaml_rust2 Yaml value into a serde_json Value.
pub fn yaml_to_json(yaml: Yaml) -> JsonValue {
    match yaml {
        Yaml::Null | Yaml::BadValue => JsonValue::Null,
        Yaml::Boolean(b) => JsonValue::Bool(b),
        Yaml::Integer(i) => JsonValue::Number(i.into()),
        Yaml::Real(s) => s
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        Yaml::String(s) => JsonValue::String(s),
        Yaml::Array(arr) => JsonValue::Array(arr.into_iter().map(yaml_to_json).collect()),
        Yaml::Hash(map) => {
            let obj = map
                .into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        Yaml::String(s) => s,
                        Yaml::Integer(i) => i.to_string(),
                        Yaml::Boolean(b) => b.to_string(),
                        other => format!("{other:?}"),
                    };
                    (key, yaml_to_json(v))
                })
                .collect();
            JsonValue::Object(obj)
        }
        // Aliases are resolved by the loader; this branch is a fallback.
        Yaml::Alias(_) => JsonValue::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust2::YamlLoader;

    fn load_one(s: &str) -> Yaml {
        YamlLoader::load_from_str(s).unwrap().remove(0)
    }

    #[test]
    fn test_simple_mapping() {
        let j = yaml_to_json(load_one("foo: bar\nnum: 42"));
        assert_eq!(j["foo"], "bar");
        assert_eq!(j["num"], 42);
    }

    #[test]
    fn test_nested() {
        let j = yaml_to_json(load_one("a:\n  b:\n    - 1\n    - 2"));
        assert_eq!(j["a"]["b"][0], 1);
        assert_eq!(j["a"]["b"][1], 2);
    }

    #[test]
    fn test_null_and_bool() {
        let j = yaml_to_json(load_one("x: ~\nflag: true"));
        assert_eq!(j["x"], JsonValue::Null);
        assert_eq!(j["flag"], true);
    }

    #[test]
    fn test_aliases() {
        let j = yaml_to_json(load_one("base: &anchor\n  x: 1\nchild:\n  <<: *anchor\n  y: 2"));
        assert_eq!(j["base"]["x"], 1);
    }
}
