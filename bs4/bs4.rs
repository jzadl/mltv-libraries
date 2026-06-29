use serde_json::{Value, Map};

const VOID_ELEMENTS: &[&str] = &[
    "area","base","br","col","embed","hr","img","input","link","meta",
    "param","source","track","wbr",
];
const RAW_TEXT_ELEMENTS: &[&str] = &["script", "style", "pre", "textarea"];

// -- Public API --

pub fn from_str(html: String) -> Value {
    let chars: Vec<char> = html.chars().collect();
    let mut pos = 0;
    let children = parse_nodes(&chars, &mut pos, None);
    let mut root = Map::new();
    root.insert("type".to_string(), Value::from("root"));
    root.insert("children".to_string(), Value::Array(children));
    Value::Object(root)
}

pub fn find_all(tree: Value, tag: String) -> Value {
    let mut results = Vec::new();
    find_all_recursive(&tree, &tag, &mut results);
    Value::Array(results)
}

pub fn find(tree: Value, tag: String) -> Value {
    let mut results = Vec::new();
    find_all_recursive(&tree, &tag, &mut results);
    results.into_iter().next().unwrap_or(Value::Null)
}

pub fn get_text(tree: Value) -> String {
    let mut out = String::new();
    get_text_recursive(&tree, &mut out);
    out
}

pub fn get_attr(tree: Value, name: String) -> String {
    if let Value::Object(ref obj) = tree {
        if let Some(Value::Object(ref attrs)) = obj.get("attrs") {
            if let Some(Value::String(val)) = attrs.get(&name) {
                return val.clone();
            }
        }
    }
    String::new()
}

pub fn get_name(tree: Value) -> String {
    if let Value::Object(ref obj) = tree {
        if let Some(Value::String(name)) = obj.get("name") {
            return name.clone();
        }
    }
    String::new()
}

pub fn children(tree: Value) -> Value {
    if let Value::Object(ref obj) = tree {
        if let Some(Value::Array(ref c)) = obj.get("children") {
            return Value::Array(c.clone());
        }
    }
    Value::Array(Vec::new())
}

// -- Internal --

fn parse_nodes(chars: &[char], pos: &mut usize, stop_tag: Option<&str>) -> Vec<Value> {
    let mut nodes = Vec::new();
    loop {
        if *pos >= chars.len() {
            break;
        }
        if chars[*pos] == '<' {
            if *pos + 1 < chars.len() && chars[*pos + 1] == '/' {
                let saved = *pos;
                if let Some(name) = parse_closing_tag(chars, pos) {
                    if stop_tag.map_or(false, |s| s.eq_ignore_ascii_case(&name)) {
                        break;
                    }
                } else {
                    *pos = saved + 1;
                }
            } else if *pos + 1 < chars.len() && chars[*pos + 1] == '!' {
                skip_markup(chars, pos);
            } else {
                if let Some(node) = parse_element(chars, pos) {
                    nodes.push(node);
                } else {
                    *pos += 1;
                }
            }
        } else {
            let text = parse_text(chars, pos);
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                let mut node = Map::new();
                node.insert("type".to_string(), Value::from("text"));
                node.insert("text".to_string(), Value::from(text));
                nodes.push(Value::Object(node));
            }
        }
    }
    nodes
}

fn parse_element(chars: &[char], pos: &mut usize) -> Option<Value> {
    let start = *pos;
    if *pos >= chars.len() || chars[*pos] != '<' {
        return None;
    }
    *pos += 1;
    skip_whitespace(chars, pos);
    let name = parse_name(chars, pos);
    if name.is_empty() {
        *pos = start + 1;
        return None;
    }
    let tag = name.to_lowercase();
    skip_whitespace(chars, pos);
    let attrs = parse_attributes(chars, pos);
    skip_whitespace(chars, pos);
    let self_close = if *pos < chars.len() && chars[*pos] == '/' {
        *pos += 1;
        true
    } else {
        false
    };
    if *pos < chars.len() && chars[*pos] == '>' {
        *pos += 1;
    }
    let is_void = self_close || VOID_ELEMENTS.contains(&tag.as_str());
    let is_raw = RAW_TEXT_ELEMENTS.contains(&tag.as_str());
    let mut elem = Map::new();
    elem.insert("type".to_string(), Value::from("element"));
    elem.insert("name".to_string(), Value::from(tag.clone()));
    elem.insert("attrs".to_string(), attrs_to_value(&attrs));
    if is_raw {
        let text = parse_raw_text(chars, pos, &tag);
        let mut text_node = Map::new();
        text_node.insert("type".to_string(), Value::from("text"));
        text_node.insert("text".to_string(), Value::from(text));
        elem.insert("children".to_string(), Value::Array(vec![Value::Object(text_node)]));
    } else if !is_void {
        let children = parse_nodes(chars, pos, Some(&tag));
        elem.insert("children".to_string(), Value::Array(children));
    } else {
        elem.insert("children".to_string(), Value::Array(Vec::new()));
    }
    Some(Value::Object(elem))
}

fn parse_closing_tag(chars: &[char], pos: &mut usize) -> Option<String> {
    if *pos + 1 >= chars.len() || chars[*pos] != '<' || chars[*pos + 1] != '/' {
        return None;
    }
    *pos += 2;
    skip_whitespace(chars, pos);
    let name = parse_name(chars, pos);
    if name.is_empty() {
        return None;
    }
    skip_whitespace(chars, pos);
    if *pos < chars.len() && chars[*pos] == '>' {
        *pos += 1;
    }
    Some(name.to_lowercase())
}

fn parse_name(chars: &[char], pos: &mut usize) -> String {
    let mut name = String::new();
    while *pos < chars.len() && (chars[*pos].is_alphanumeric() || chars[*pos] == '-' || chars[*pos] == '_' || chars[*pos] == ':') {
        name.push(chars[*pos]);
        *pos += 1;
    }
    name
}

fn parse_attributes(chars: &[char], pos: &mut usize) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    loop {
        skip_whitespace(chars, pos);
        if *pos >= chars.len() || chars[*pos] == '>' || chars[*pos] == '/' {
            break;
        }
        let name = parse_name(chars, pos);
        if name.is_empty() {
            break;
        }
        skip_whitespace(chars, pos);
        let value = if *pos < chars.len() && chars[*pos] == '=' {
            *pos += 1;
            skip_whitespace(chars, pos);
            parse_attr_value(chars, pos)
        } else {
            String::new()
        };
        attrs.push((name.to_lowercase(), value));
    }
    attrs
}

fn parse_attr_value(chars: &[char], pos: &mut usize) -> String {
    if *pos >= chars.len() {
        return String::new();
    }
    let quote = chars[*pos];
    if quote == '"' || quote == '\'' {
        *pos += 1;
        let mut val = String::new();
        while *pos < chars.len() && chars[*pos] != quote {
            if chars[*pos] == '&' {
                val.push_str(&parse_entity(chars, pos));
            } else {
                val.push(chars[*pos]);
                *pos += 1;
            }
        }
        if *pos < chars.len() {
            *pos += 1;
        }
        val
    } else {
        let mut val = String::new();
        while *pos < chars.len() && !chars[*pos].is_whitespace() && chars[*pos] != '>' && chars[*pos] != '/' {
            val.push(chars[*pos]);
            *pos += 1;
        }
        val
    }
}

fn parse_entity(chars: &[char], pos: &mut usize) -> String {
    if *pos >= chars.len() || chars[*pos] != '&' {
        return String::new();
    }
    *pos += 1;
    let mut entity = String::new();
    while *pos < chars.len() && chars[*pos] != ';' && entity.len() < 20 {
        entity.push(chars[*pos]);
        *pos += 1;
    }
    if *pos < chars.len() && chars[*pos] == ';' {
        *pos += 1;
    }
    match entity.as_str() {
        "amp" => "&".to_string(),
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        "quot" => "\"".to_string(),
        "apos" => "'".to_string(),
        "nbsp" => " ".to_string(),
        _ if entity.starts_with('#') => {
            let num_str = entity.trim_start_matches('#').trim_start_matches('x');
            if let Ok(code) = if entity.starts_with("#x") || entity.starts_with("#X") {
                u32::from_str_radix(num_str, 16)
            } else {
                u32::from_str_radix(num_str, 10)
            } {
                char::from_u32(code).map(|c| c.to_string()).unwrap_or(entity)
            } else {
                entity
            }
        }
        _ => entity,
    }
}

fn parse_text(chars: &[char], pos: &mut usize) -> String {
    let mut text = String::new();
    while *pos < chars.len() && chars[*pos] != '<' {
        if chars[*pos] == '&' {
            text.push_str(&parse_entity(chars, pos));
        } else {
            text.push(chars[*pos]);
            *pos += 1;
        }
    }
    text
}

fn parse_raw_text(chars: &[char], pos: &mut usize, tag: &str) -> String {
    let end_tag = format!("</{}", tag);
    let mut text = String::new();
    while *pos + 2 < chars.len() {
        if chars[*pos] == '<'
            && chars[*pos + 1] == '/'
            && chars[*pos + 2..]
                .iter()
                .collect::<String>()
                .to_lowercase()
                .starts_with(&end_tag.to_lowercase())
        {
            break;
        }
        text.push(chars[*pos]);
        *pos += 1;
    }
    text
}

fn skip_markup(chars: &[char], pos: &mut usize) {
    if *pos + 3 < chars.len()
        && chars[*pos + 1] == '!'
        && chars[*pos + 2] == '-'
        && chars[*pos + 3] == '-'
    {
        while *pos + 2 < chars.len() {
            if chars[*pos] == '-' && chars[*pos + 1] == '-' && chars[*pos + 2] == '>' {
                *pos += 3;
                return;
            }
            *pos += 1;
        }
    } else {
        while *pos < chars.len() && chars[*pos] != '>' {
            *pos += 1;
        }
        if *pos < chars.len() {
            *pos += 1;
        }
    }
}

fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

fn attrs_to_value(attrs: &[(String, String)]) -> Value {
    let mut map = Map::new();
    for (k, v) in attrs {
        if !map.contains_key(k) {
            map.insert(k.clone(), Value::from(v.clone()));
        }
    }
    Value::Object(map)
}

fn find_all_recursive(tree: &Value, tag: &str, results: &mut Vec<Value>) {
    if let Value::Object(obj) = tree {
        if obj.get("type").and_then(|v| v.as_str()) == Some("element") {
            if obj.get("name").and_then(|v| v.as_str()) == Some(tag) {
                results.push(tree.clone());
            }
            if let Some(Value::Array(children)) = obj.get("children") {
                for child in children {
                    find_all_recursive(child, tag, results);
                }
            }
        } else if let Some(Value::Array(children)) = obj.get("children") {
            for child in children {
                find_all_recursive(child, tag, results);
            }
        }
    }
}

fn get_text_recursive(tree: &Value, out: &mut String) {
    if let Value::Object(obj) = tree {
        match obj.get("type").and_then(|v| v.as_str()) {
            Some("text") => {
                if let Some(Value::String(t)) = obj.get("text") {
                    out.push_str(t);
                }
            }
            Some("element") => {
                if let Some(Value::Array(children)) = obj.get("children") {
                    for child in children {
                        get_text_recursive(child, out);
                    }
                }
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
            }
            _ => {
                if let Some(Value::Array(children)) = obj.get("children") {
                    for child in children {
                        get_text_recursive(child, out);
                    }
                }
            }
        }
    }
}
