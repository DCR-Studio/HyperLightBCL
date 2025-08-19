// HyperLightBCL-Core For Rust
//
// By TNTyep520 <sbtntyep520@gmail.com>
//
// The Project uses the GNU General Public License 3.0 Open Source License
//
// Requiring preservation of specified reasonable legal notices or
// author attributions in that material or in the Appropriate Legal
// Notices displayed by works containing it; or
//
// Get's Microsoft Devices Codes Modules
// 
// Copyright ©2024-2025 DCR Studio and contributors. All rights reserved

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeviceCodeResponse {
    pub verification_uri: String,
    /// Accept missing or null user_code
    #[serde(default)]
    pub user_code: Option<String>,

    #[serde(default)]
    pub device_code: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
    #[serde(default)]
    pub interval: Option<u64>,
    #[serde(default)]
    pub message: Option<String>,
}
