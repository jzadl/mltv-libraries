pub fn hex_encode(data: Vec<i64>) -> String {
    data.iter().map(|&b| format!("{:02x}", b as u8)).collect()
}

pub fn hex_decode(hex: String) -> Vec<i64> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 { return Vec::new(); }
    (0..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i+2], 16).ok())
        .map(|b| b as i64)
        .collect()
}

const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn b64_encode(data: Vec<i64>) -> String {
    let bytes: Vec<u8> = data.iter().map(|&b| b as u8).collect();
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_CHARS[((triple >> 18) & 0x3F) as usize] as char);
        out.push(B64_CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() >= 2 { out.push(B64_CHARS[((triple >> 6) & 0x3F) as usize] as char); } else { out.push('='); }
        if chunk.len() == 3 { out.push(B64_CHARS[(triple & 0x3F) as usize] as char); } else { out.push('='); }
    }
    out
}

pub fn b64_decode(s: String) -> Vec<i64> {
    let clean: Vec<u8> = s.bytes()
        .filter(|&b| b != b'=' && !b.is_ascii_whitespace())
        .collect();
    let mut out = Vec::new();
    for chunk in clean.chunks(4) {
        if chunk.len() < 4 { break; }
        let mut val = 0u32;
        for &c in chunk {
            let idx = B64_CHARS.iter().position(|&x| x == c).unwrap_or(0) as u32;
            val = (val << 6) | idx;
        }
        out.push(((val >> 16) & 0xFF) as i64);
        out.push(((val >> 8) & 0xFF) as i64);
        out.push((val & 0xFF) as i64);
    }
    out
}
