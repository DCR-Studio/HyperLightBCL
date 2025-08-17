// Open Source BCL-Launcher Core For Rust
//
// By TNTyep520 <sbtntyep520@gmail.com>
//
// The Project uses the GNU General Public License 3.0 Open Source License
//
// Requiring preservation of specified reasonable legal notices or
// author attributions in that material or in the Appropriate Legal
// Notices displayed by works containing it; or
//
// Get's Offline Players UUID Modules
// 
// Copyright ©2024-2025 DCR Studio and contributors. All rights reserved

use md5;

pub fn offline_uuid(name: &str) -> String {
    let mut b = md5::compute(format!("OfflinePlayer:{}", name)).0;
    b[6] = (b[6] & 0x0F) | 0x30;
    b[8] = (b[8] & 0x3F) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
    )
}

// Test Offline_Players,will delete later

fn main() {
    use std::io::{self, Write};
    print!("Please enter the player name:");
    io::stdout().flush().unwrap();

    let mut raw_name = String::new();
    io::stdin().read_line(&mut raw_name).unwrap();
    let name = raw_name.trim();

    let uuid = offline_uuid(name);
    println!("Players {} Offline UUID is:{}", name, uuid);
}