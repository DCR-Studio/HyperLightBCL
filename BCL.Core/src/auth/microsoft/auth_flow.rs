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
// Waiting Microsoft Login  Modules
// 
// Copyright ©2024-2025 DCR Studio and contributors. All rights reserved

use crate::device_code::DeviceCodeResponse;
use reqwest::blocking::Client;
use serde_json::Value;

    pub fn request_code(&mut self) -> Result<DeviceCodeResponse, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let resp = client.post(&self.device_code_endpoint)
            .form(&self.device_code_params())
            .send()?;


        let code_res: DeviceCodeResponse = resp.json()?;
        Ok(code_res)
    }
}
