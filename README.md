# OrbitalView Wallpaper
A lightweight Windows desktop app that downloads live NOAA satellite imagery and sets it as your desktop wallpaper. Built with **Tauri v2 (Rust + TypeScript)** for minimal resource usage, no accounts, and no telemetry.

## 🚀 Features
- 🌎 Live NOAA satellite wallpapers
- 🖼️ Auto-download + apply as wallpaper
- ⏱️ Auto-refresh toggle (on/off, interval in minutes)
- 💾 Local caching (won’t redownload identical images)
- ⚙️ Configurable via `sources.json` in AppData
- ⭐ Favorites system for a clean main dropdown
- 📍 Multi-step region selector:
  - Area → West / East (no satellite jargon)
  - Coverage → Full Disk / U.S. & Nearby / Regional
  - Region → Specific geographic views
- 🔍 Freshness indicator (flags stale images >12h)
- 🖥️ Works on Windows 10/11

## 🧩 Tech Stack
| Component | Tech |
|-----------|------|
| Desktop Framework | Tauri v2 |
| Frontend | Vanilla HTML + TypeScript + Vite |
| Backend | Rust |
| HTTP | reqwest |
| Wallpaper control | wallpaper crate |
| Config | JSON (`sources.json`) |

## 📂 Project Structure
```
OrbitalView-Wallpaper/
├── src/                   # Frontend (TS)
│   ├── main.ts            # UI logic + Tauri invokes
│   └── index.html         # UI
├── src-tauri/             # Backend (Rust)
│   ├── src/main.rs        # Commands, caching, wallpaper logic
│   ├── Cargo.toml
│   └── tauri.conf.json
└── dist/                  # Built frontend
```

## 📌 Installation (Development)
```sh
git clone git clone https://github.com/StrangeSaFire/OrbitalView-Wallpaper
cd orbitalview-wallpaper
npm install
npm run tauri dev
```

## 🧠 How It Works
```
Select Source → Build Image URL → HEAD request checks Last-Modified
  → If unchanged → Skip download, reuse cached file
  → If changed → Download → Save as wallpaper.jpg → Apply to desktop
```

## 📁 File Storage
```
%APPDATA%\OrbitalViewWallpaper\
  ├── wallpaper.jpg
  ├── wallpaper.tmp
  ├── cache.json
  └── sources.json
```

## ⚙️ Configuration
Add `"favorite": true` to show an item in the main dropdown:

```json
{
  "id": "noaa_goes_west_full_disk_geocolor",
  "favorite": true
}
```

If no favorites exist, the app shows all sources.

## 🛰️ NOAA Imagery
Base URL:
```
https://cdn.star.nesdis.noaa.gov/
```

High-res / low-res URLs:
```
{base_path}/latest.jpg
{base_path}/thumbnail.jpg
```

Example:
```
GOES18/ABI/FD/GEOCOLOR/latest.jpg
```

## 🛠️ Roadmap
- [ ] In-app “Add to Favorites” toggle
- [ ] Editable config UI
- [ ] Offline startup fallback images
- [ ] Optional installer + auto-update
- [ ] Save last selected area + region
- [ ] Export/import custom configurations

## ❤️ Credits
Imagery courtesy of NOAA / NESDIS / STAR  
https://www.nesdis.noaa.gov/
  -All satellite imagery remains the property of its respective providers and is subject to their usage guidelines. This project is not affiliated with or endorsed by NOAA, NESDIS, STAR, or any other agency.

Inspired by KYDronePilot's SpaceEye application.
https://github.com/KYDronePilot/SpaceEye
This program was developed using Tauri with the
assistance of artificial intelligence. 

## 📜 License

OrbitalView Wallpaper is released under the [MIT License](./LICENSE).

The names "OrbitalView" and "OrbitalView Wallpaper" may not be used for commercial promotion of derivative works without explicit permission.
