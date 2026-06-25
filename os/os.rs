pub fn getcwd() -> String {
    std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()
}
pub fn chdir(path: String) -> bool {
    std::env::set_current_dir(path).is_ok()
}
pub fn listdir(path: String) -> Vec<String> {
    std::fs::read_dir(path)
        .map(|entries| entries.filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string())).collect())
        .unwrap_or_default()
}
pub fn mkdir(path: String) -> bool {
    std::fs::create_dir(path).is_ok()
}
pub fn makedirs(path: String) -> bool {
    std::fs::create_dir_all(path).is_ok()
}
pub fn remove(path: String) -> bool {
    std::fs::remove_file(path).is_ok()
}
pub fn rmdir(path: String) -> bool {
    std::fs::remove_dir(path).is_ok()
}
pub fn rename(src: String, dst: String) -> bool {
    std::fs::rename(src, dst).is_ok()
}
pub fn path_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}
pub fn is_dir(path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}
pub fn is_file(path: String) -> bool {
    std::path::Path::new(&path).is_file()
}
pub fn getenv(key: String) -> String {
    std::env::var(key).unwrap_or_default()
}
pub fn setenv(key: String, val: String) -> bool {
    std::env::set_var(key, val); true
}
pub fn unsetenv(key: String) -> bool {
    std::env::remove_var(key); true
}
pub fn home() -> String {
    std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_default()
}
pub fn tempdir() -> String {
    std::env::temp_dir().to_string_lossy().to_string()
}
pub fn sep() -> String {
    std::path::MAIN_SEPARATOR.to_string()
}
pub fn linesep() -> String {
    "\n".to_string()
}
