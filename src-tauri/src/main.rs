#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem, MenuEvent};
use tauri::tray::TrayIconBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ImageSource {
    id: String,
    name: String,

    #[serde(default)]
    base_path: Option<String>,

    #[serde(default)]
    image_url: Option<String>,

    #[serde(default)]
    satellite: Option<String>,

    #[serde(default)]
    sector: Option<String>,

    #[serde(default)]
    product: Option<String>,

    #[serde(default)]
    region: Option<String>,

    #[serde(default)]
    resolution_hint_high: Option<String>,

    #[serde(default)]
    resolution_hint_low: Option<String>,

    #[serde(default)]
    default_refresh_minutes: Option<u64>,

    #[serde(default)]
    attribution: Option<String>,

    #[serde(default)]
    favorite: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SourcesConfig {
    version: u32,
    sources: Vec<ImageSource>,
}

#[tauri::command]
fn ping() -> String {
    "pong from Rust".to_string()
}

#[tauri::command]
fn read_local_json(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Failed to read {path}: {e}"))
}

#[tauri::command]
fn get_sources_config() -> Result<SourcesConfig, String> {
    use std::{fs, path::PathBuf};

    let appdata = std::env::var("APPDATA")
        .map_err(|e| format!("APPDATA env var not set: {e}"))?;

    let mut path = PathBuf::from(appdata);
    path.push("OrbitalViewWallpaper");
    path.push("sources.json");

    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read sources.json at {path:?}: {e}"))?;

    let cfg: SourcesConfig = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse sources.json: {e}"))?;

    Ok(cfg)
}

#[tauri::command]
fn set_wallpaper(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        wallpaper::set_from_path(&path).map_err(|e| format!("Failed to set wallpaper: {e}"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("set_wallpaper is only implemented on Windows".to_string())
    }
}

#[tauri::command]
fn download_image_and_set_wallpaper(url: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::{
            fs::{self, File},
            io::Write,
            path::PathBuf,
        };

        if url.trim().is_empty() {
            return Err("URL is empty".to_string());
        }

        let resp = reqwest::blocking::get(&url)
            .map_err(|e| format!("HTTP request failed: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP error: {}", resp.status()));
        }

        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .ok_or("Server did not provide a Content-Type header")?
            .to_str()
            .map_err(|_| "Content-Type has invalid characters")?
            .to_lowercase();

        if !content_type.starts_with("image/") {
            return Err(format!("Not an image: Content-Type was `{content_type}`"));
        }

        let bytes = resp
            .bytes()
            .map_err(|e| format!("Failed to read response body: {e}"))?;

        let appdata = std::env::var("APPDATA")
            .map_err(|e| format!("APPDATA env var not set: {e}"))?;

        let mut dir = PathBuf::from(appdata);
        dir.push("OrbitalViewWallpaper");

        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create app data dir {dir:?}: {e}"))?;

        let mut staging = dir.clone();
        staging.push("wallpaper.tmp");

        let mut final_path = dir.clone();
        final_path.push("wallpaper.jpg");

        {
            let mut f = File::create(&staging)
                .map_err(|e| format!("Failed to create staging file {staging:?}: {e}"))?;
            f.write_all(&bytes)
                .map_err(|e| format!("Failed to write image to staging file: {e}"))?;
        }

        fs::rename(&staging, &final_path)
            .map_err(|e| format!("Failed to finalize wallpaper file: {e}"))?;

        let final_path_str = final_path
            .to_str()
            .ok_or_else(|| format!("Final path is not valid UTF-8: {final_path:?}"))?
            .to_string();

        wallpaper::set_from_path(&final_path_str)
            .map_err(|e| format!("Failed to set wallpaper: {e}"))?;

        Ok(final_path_str)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("download_image_and_set_wallpaper is only implemented on Windows".to_string())
    }
}

#[tauri::command]
fn set_run_on_startup(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::path::PathBuf;
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
        let (key, _) = hkcu
            .create_subkey(path)
            .map_err(|e| format!("Failed to open Run key: {e}"))?;

        let exe_path: PathBuf = std::env::current_exe()
            .map_err(|e| format!("Failed to get exe path: {e}"))?;

        if enabled {
            let value = format!("\"{}\"", exe_path.display());
            key.set_value("OrbitalView Wallpaper", &value)
                .map_err(|e| format!("Failed to set startup value: {e}"))?;
        } else {
            let _ = key.delete_value("OrbitalView Wallpaper");
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Run-on-startup is only implemented on Windows.".to_string())
    }
}

fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &quit])?;

    let icon = app.default_window_icon().cloned();

    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app: &AppHandle, event: MenuEvent| {
            let id = event.id().as_ref();
            match id {
                "show" => {
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        });

    if let Some(icon) = icon {
        builder = builder.icon(icon);
    }

    builder.build(app)?;

    Ok(())
}

fn main() {
    println!("OrbitalView Wallpaper: starting main()");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            create_tray(&handle)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap_or_default();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            ping,
            read_local_json,
            get_sources_config,
            set_wallpaper,
            download_image_and_set_wallpaper,
            set_run_on_startup,
        ])
        .run(tauri::generate_context!())
        .expect("OrbitalView Wallpaper: error while running app");
}
