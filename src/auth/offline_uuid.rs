use md5;
use std::io;

pub fn get_offline_uuid(player_name: &str) -> String {
    let mut hasher = md5::Context::new();
    hasher.consume(b"OfflinePlayer:");
    hasher.consume(player_name.as_bytes());
    let mut bytes = hasher.compute().0;

    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    let uuid = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    );

    if !verify_uuid_format(&uuid) {
        eprintln!("Error:生成的离线UUID格式验证失败!");
    }

    uuid
}

fn verify_uuid_format(uuid: &str) -> bool {
    if uuid.len() != 36 { return false; }
    let parts: Vec<&str> = uuid.split('-').collect();
    if parts.len() != 5 { return false; }
    if parts[0].len() != 8 || parts[1].len() != 4 || 
       parts[2].len() != 4 || parts[3].len() != 4 || 
       parts[4].len() != 12 { return false; }

    let bytes_result: Result<Vec<u8>, _> = parts.join("")
        .as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16))
        .collect();

    if let Ok(bytes) = bytes_result {
        (bytes[6] & 0xf0) == 0x30 && 
        (bytes[8] & 0xc0) == 0x80
    } else {
        false
    }
}

fn main() {
    println!("输入玩家名称:");
    
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)
        .expect("Error:读取数值时出现错误");
    
    let player_name = player_name.trim();
    
    if player_name.is_empty() {
        println!("Error:用户名不能为空");
        return;
    }

    let uuid = get_offline_uuid(player_name);
    
    println!("Player Name: {}", player_name);
    println!("离线 UUID: {}", uuid);
    
    if verify_uuid_format(&uuid) {
        println!("离线 UUID 格式验证通过");
    } else {
        println!("Error:离线 UUID 格式验证失败");
    }
}
