pub fn slugify(s: String) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

pub fn truncate(s: String, max_len: i64) -> String {
    let max = max_len.max(3) as usize;
    if s.len() <= max {
        s
    } else {
        format!("{}...", &s[..max-3])
    }
}

pub fn pad_left(s: String, width: i64, ch: String) -> String {
    let w = width.max(0) as usize;
    let c = ch.chars().next().unwrap_or(' ');
    if s.len() >= w { s } else { format!("{}{}", c.to_string().repeat(w - s.len()), s) }
}

pub fn pad_right(s: String, width: i64, ch: String) -> String {
    let w = width.max(0) as usize;
    let c = ch.chars().next().unwrap_or(' ');
    if s.len() >= w { s } else { format!("{}{}", s, c.to_string().repeat(w - s.len())) }
}

pub fn pad_center(s: String, width: i64, ch: String) -> String {
    let w = width.max(0) as usize;
    let c = ch.chars().next().unwrap_or(' ');
    if s.len() >= w { return s; }
    let pad_total = w - s.len();
    let left = pad_total / 2;
    let right = pad_total - left;
    format!("{}{}{}", c.to_string().repeat(left), s, c.to_string().repeat(right))
}

pub fn camel_case(s: String) -> String {
    let mut out = String::new();
    let mut upper = false;
    for c in s.chars() {
        if c == '-' || c == '_' || c == ' ' {
            upper = true;
        } else if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            if out.is_empty() { out.push(c.to_ascii_lowercase()); }
            else { out.push(c); }
        }
    }
    out
}

pub fn snake_case(s: String) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !out.is_empty() { out.push('_'); }
            out.push(c.to_ascii_lowercase());
        } else if c == '-' || c == ' ' {
            out.push('_');
        } else {
            out.push(c);
        }
    }
    out
}

pub fn kebab_case(s: String) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !out.is_empty() { out.push('-'); }
            out.push(c.to_ascii_lowercase());
        } else if c == '_' || c == ' ' {
            out.push('-');
        } else {
            out.push(c);
        }
    }
    out
}

pub fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => s,
        Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
    }
}

pub fn capitalize_words(s: String) -> String {
    s.split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn swap_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_uppercase() { c.to_ascii_lowercase() }
            else if c.is_ascii_lowercase() { c.to_ascii_uppercase() }
            else { c }
        })
        .collect()
}

pub fn repeat(s: String, count: i64) -> String {
    if count <= 0 { return String::new(); }
    s.repeat(count as usize)
}

pub fn reverse(s: String) -> String {
    s.chars().rev().collect()
}

pub fn count_substr(s: String, substr: String) -> i64 {
    if substr.is_empty() { return 0; }
    s.matches(&substr).count() as i64
}

pub fn mask(s: String, visible: i64, mask_char: String) -> String {
    let vis = visible.max(0) as usize;
    let c = mask_char.chars().next().unwrap_or('*');
    if s.len() <= vis { return s; }
    let masked_len = s.len() - vis;
    format!("{}{}", c.to_string().repeat(masked_len), &s[vis..])
}

pub fn excerpt(s: String, max_len: i64, suffix: String) -> String {
    let max = max_len.max(3) as usize;
    if s.len() <= max { return s; }
    let truncated = &s[..max-3];
    let last_space = truncated.rfind(' ');
    match last_space {
        Some(pos) if pos > 0 => format!("{}{}", &truncated[..pos], suffix),
        _ => format!("{}{}", truncated, suffix),
    }
}

pub fn ordinal(n: i64) -> String {
    let suffix = match n % 100 {
        11 | 12 | 13 => "th",
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", n, suffix)
}

pub fn comma_sep(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let neg = chars[0] == '-';
    let start = if neg { 1 } else { 0 };
    let len = chars.len() - start;
    for (i, &c) in chars.iter().enumerate().skip(start) {
        if (len - (i - start)) % 3 == 0 && i > start {
            result.push(',');
        }
        result.push(c);
    }
    if neg { format!("-{}", result) } else { result }
}

pub fn pluralize(count: i64, singular: String, plural: String) -> String {
    if count == 1 { singular } else { plural }
}
