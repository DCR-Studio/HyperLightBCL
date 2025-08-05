use md5;
use std::io::{self, Write};

fn main() {
    print!("请输入玩家名称: ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    if name.is_empty() {
        println!("\x1b[31mError: 你输入的玩家名称不能为空\x1b[0m");
        return;
    }

    println!("\nPlayer Name: {}", name);
    
    let mut bytes = md5::compute(format!("OfflinePlayer:{}", name)).0;
    bytes[6] = (bytes[6] & 0x0F) | 0x30;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    println!(
        "离线 UUID: {:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    );
}