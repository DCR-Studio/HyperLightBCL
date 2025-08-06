use md5;
use std::io::{self, Write};

fn main() {
    print!("请输入玩家名称（仅限字母、数字、下划线）：");
    io::stdout().flush().unwrap();

    let mut raw_name = String::new();
    io::stdin().read_line(&mut raw_name).unwrap();
    let name = raw_name.trim();

    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        eprintln!("\x1b[31m错误：玩家名称只能包含字母、数字或下划线。\x1b[0m");
        return;
    }

    let mut b = md5::compute(format!("OfflinePlayer:{}", name)).0;
    b[6] = (b[6] & 0x0F) | 0x30;
    b[8] = (b[8] & 0x3F) | 0x80;
    println!(
        "玩家 {} 的离线 UUID: {:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        name,
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
    );
}