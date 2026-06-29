pub fn progress_bar(current: i64, total: i64, desc: String) {
    let pct = if total > 0 { current * 100 / total } else { 0 };
    let filled = pct * 20 / 100;
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('=');
    }
    for _ in filled..20 {
        bar.push(' ');
    }
    eprint!("\r{} [{}] {}/{} ({}%)", desc, bar, current, total, pct);
}

pub fn done() {
    eprintln!();
}
