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
// Get's Minecraft Players UUID Modules
// 
// Copyright ©2024-2025 DCR Studio and contributors. All rights reserved

use mc_auth::{AuthFlow, device_code::DeviceCodeResponse};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use webbrowser;

async fn verify_ownership(client: &Client, access_token: &str) -> Result<(), String> {
    let response = client
        .get("https://api.minecraftservices.com/entitlements/mcstore")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    match response.status() {
        StatusCode::OK => {
            let entitlements: Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
            let items = entitlements["items"].as_array().unwrap_or(&Vec::new());
            if !items.iter().any(|item| {
                item["name"].as_str() == Some("product_minecraft") ||
                item["name"].as_str() == Some("game_minecraft")
            }) {
                return Err("Account does not own Minecraft".to_string());
            }
            Ok(())
        }
        StatusCode::UNAUTHORIZED => Err("Invalid access token".to_string()),
        StatusCode::FORBIDDEN => Err("Account not authorized for Minecraft services".to_string()),
        status => Err(format!("Ownership verification failed: {}", status)),
    }
}

async fn get_profile(client: &Client, access_token: &str) -> Result<(String, String), String> {
    let response = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    match response.status() {
        StatusCode::OK => {
            let profile: Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
            let uuid = profile["id"].as_str().ok_or("UUID not found")?.to_string();
            let username = profile["name"].as_str().ok_or("Username not found")?.to_string();
            Ok((uuid, username))
        }
        StatusCode::NOT_FOUND => Err("Minecraft Java profile not created".to_string()),
        status => Err(format!("Profile fetch failed: {}", status)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = Client::new();

    let mc_access_token = tokio::task::spawn_blocking(
        || -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
            let client_id = "87acf5d5-124c-4dfc-a9db-6ef7cfa4b955";
            let mut auth = AuthFlow::new(client_id);

            // request_code now returns DeviceCodeResponse with optional user_code
            let code_res: DeviceCodeResponse = auth.request_code()
                .map_err(|e| format!("Failed to request code: {:?}", e))?;

            println!("Verification URI: {}", code_res.verification_uri);
            match &code_res.user_code {
                Some(code) => println!("User code: {}", code),
                None => {
                    println!("No user_code provided by server.");
                    if let Some(msg) = &code_res.message {
                        println!("Server message: {}", msg);
                    }
                }
            }

            if let Err(e) = webbrowser::open(&code_res.verification_uri) {
                println!("Failed to open browser: {}. Please visit the link manually.", e);
            }

            auth.wait_for_login().map_err(|e| format!("Failed to wait for login: {:?}", e))?;
            auth.login_in_xbox_live().map_err(|e| format!("Failed to login to Xbox Live: {:?}", e))?;
            let minecraft = auth.login_in_minecraft().map_err(|e| format!("Failed to login to Minecraft: {:?}", e))?;
            Ok(minecraft.access_token.clone())
        }
    ).await??;

    verify_ownership(&client, &mc_access_token).await?;
    let (uuid, username) = get_profile(&client, &mc_access_token).await?;

    println!("Username: {}", username);
    println!("UUID: {}", uuid);
    Ok(())
}
