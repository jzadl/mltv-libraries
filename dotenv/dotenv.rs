use serde_json::{Value, Map};

pub fn load_dotenv(path: String) -> bool {
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let raw = trimmed[eq_pos + 1..].trim().to_string();
            let val = strip_quotes(&raw);
            std::env::set_var(&key, &val);
        }
    }
    true
}

pub fn dotenv_values(path: String) -> Value {
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Value::Object(Map::new()),
    };
    let mut map = Map::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let raw = trimmed[eq_pos + 1..].trim().to_string();
            let val = strip_quotes(&raw);
            map.insert(key, Value::String(val));
        }
    }
    Value::Object(map)
}

fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}
