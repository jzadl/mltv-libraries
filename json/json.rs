use serde_json::{Value, from_str, to_string, to_string_pretty};
use std::collections::HashMap;

pub fn to_json(val: Value) -> String {
    to_string(&val).unwrap_or_default()
}

pub fn pretty_json(val: Value) -> String {
    to_string_pretty(&val).unwrap_or_default()
}

pub fn parse_json(s: String) -> Value {
    from_str(&s).unwrap_or(Value::Null)
}

pub fn is_null(val: &Value) -> bool {
    val.is_null()
}

pub fn json_get(val: &Value, key: String) -> Value {
    val.get(&key).cloned().unwrap_or(Value::Null)
}

pub fn json_set(mut val: Value, key: String, new_val: Value) -> Value {
    if let Value::Object(ref mut map) = val {
        map.insert(key, new_val);
    }
    val
}

pub fn json_del(mut val: Value, key: String) -> Value {
    if let Value::Object(ref mut map) = val {
        map.remove(&key);
    }
    val
}

pub fn json_keys(val: &Value) -> Vec<String> {
    match val {
        Value::Object(m) => m.keys().cloned().collect(),
        _ => Vec::new(),
    }
}

pub fn json_length(val: &Value) -> i64 {
    match val {
        Value::Array(a) => a.len() as i64,
        Value::Object(m) => m.len() as i64,
        _ => 0,
    }
}

pub fn json_merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut m1), Value::Object(m2)) => {
            m1.extend(m2);
            Value::Object(m1)
        }
        (_, b) => b,
    }
}

pub fn new_obj() -> Value {
    Value::Object(HashMap::new())
}

pub fn new_arr() -> Value {
    Value::Array(Vec::new())
}

pub fn str_val(s: String) -> Value {
    Value::String(s)
}

pub fn int_val(n: i64) -> Value {
    Value::Number(n.into())
}

pub fn float_val(f: f64) -> Value {
    serde_json::json!(f)
}

pub fn bool_val(b: bool) -> Value {
    Value::Bool(b)
}

pub fn null_val() -> Value {
    Value::Null
}
