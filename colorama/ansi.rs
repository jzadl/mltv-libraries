use std::collections::HashMap;

pub fn code_to_chars(code: i64) -> String {
    format!("\x1b[{}m", code)
}

pub fn clear_screen(mode: i64) -> String {
    format!("\x1b[{}J", mode)
}

pub fn clear_line(mode: i64) -> String {
    format!("\x1b[{}K", mode)
}

pub fn cursor_up(n: i64) -> String {
    format!("\x1b[{}A", n)
}

pub fn cursor_down(n: i64) -> String {
    format!("\x1b[{}B", n)
}

pub fn cursor_forward(n: i64) -> String {
    format!("\x1b[{}C", n)
}

pub fn cursor_back(n: i64) -> String {
    format!("\x1b[{}D", n)
}

pub fn cursor_pos(x: i64, y: i64) -> String {
    format!("\x1b[{};{}H", y, x)
}

pub fn fore() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("BLACK".to_string(), "\x1b[30m".to_string());
    m.insert("RED".to_string(), "\x1b[31m".to_string());
    m.insert("GREEN".to_string(), "\x1b[32m".to_string());
    m.insert("YELLOW".to_string(), "\x1b[33m".to_string());
    m.insert("BLUE".to_string(), "\x1b[34m".to_string());
    m.insert("MAGENTA".to_string(), "\x1b[35m".to_string());
    m.insert("CYAN".to_string(), "\x1b[36m".to_string());
    m.insert("WHITE".to_string(), "\x1b[37m".to_string());
    m.insert("RESET".to_string(), "\x1b[39m".to_string());
    m.insert("LIGHTBLACK_EX".to_string(), "\x1b[90m".to_string());
    m.insert("LIGHTRED_EX".to_string(), "\x1b[91m".to_string());
    m.insert("LIGHTGREEN_EX".to_string(), "\x1b[92m".to_string());
    m.insert("LIGHTYELLOW_EX".to_string(), "\x1b[93m".to_string());
    m.insert("LIGHTBLUE_EX".to_string(), "\x1b[94m".to_string());
    m.insert("LIGHTMAGENTA_EX".to_string(), "\x1b[95m".to_string());
    m.insert("LIGHTCYAN_EX".to_string(), "\x1b[96m".to_string());
    m.insert("LIGHTWHITE_EX".to_string(), "\x1b[97m".to_string());
    m
}

pub fn back() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("BLACK".to_string(), "\x1b[40m".to_string());
    m.insert("RED".to_string(), "\x1b[41m".to_string());
    m.insert("GREEN".to_string(), "\x1b[42m".to_string());
    m.insert("YELLOW".to_string(), "\x1b[43m".to_string());
    m.insert("BLUE".to_string(), "\x1b[44m".to_string());
    m.insert("MAGENTA".to_string(), "\x1b[45m".to_string());
    m.insert("CYAN".to_string(), "\x1b[46m".to_string());
    m.insert("WHITE".to_string(), "\x1b[47m".to_string());
    m.insert("RESET".to_string(), "\x1b[49m".to_string());
    m.insert("LIGHTBLACK_EX".to_string(), "\x1b[100m".to_string());
    m.insert("LIGHTRED_EX".to_string(), "\x1b[101m".to_string());
    m.insert("LIGHTGREEN_EX".to_string(), "\x1b[102m".to_string());
    m.insert("LIGHTYELLOW_EX".to_string(), "\x1b[103m".to_string());
    m.insert("LIGHTBLUE_EX".to_string(), "\x1b[104m".to_string());
    m.insert("LIGHTMAGENTA_EX".to_string(), "\x1b[105m".to_string());
    m.insert("LIGHTCYAN_EX".to_string(), "\x1b[106m".to_string());
    m.insert("LIGHTWHITE_EX".to_string(), "\x1b[107m".to_string());
    m
}

pub fn style() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("BRIGHT".to_string(), "\x1b[1m".to_string());
    m.insert("DIM".to_string(), "\x1b[2m".to_string());
    m.insert("NORMAL".to_string(), "\x1b[22m".to_string());
    m.insert("RESET_ALL".to_string(), "\x1b[0m".to_string());
    m
}

pub fn init() {}

pub fn deinit() {}

pub fn just_fix_windows_console() {}
