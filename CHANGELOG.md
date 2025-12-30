# Changelog

## [0.1.0] - 2025-12-29
### 🎉 Initial Release — *OrbitalView Wallpaper*
OrbitalView Wallpaper is a lightweight Windows app that keeps your desktop background synced with live satellite imagery from NOAA/NESDIS.

#### ✨ Key Features
- Automatic wallpaper updates (interval-based, with on/off toggle)
- Favorites dropdown + full catalog selector (Area → Coverage → Region)
- Live preview thumbnails before applying
- High-/Low-resolution toggle (`latest.jpg` / `thumbnail.jpg`)
- Image freshness indicator using Last-Modified headers
- Local file path support (C:\path\image.jpg)
- Detects stale images (>12h old) and warns user
- Config + state stored in `%APPDATA%\Roaming\OrbitalViewWallpaper`
- "Run on Windows Startup" option
- Minimize to / Close to System Tray
- Built with **Tauri v2** (Rust + TypeScript)

#### 🔧 Technical Notes
- Satellite data sourced from NOAA CDN  
  `https://cdn.star.nesdis.noaa.gov/GOES18/...` and `GOES19/...`
- Local cache + wallpaper stored in:  
  `%APPDATA%\Roaming\OrbitalViewWallpaper\wallpaper.jpg`
- Installer built as `.msi` targeting Windows 10/11 x64

---

## Planned for Upcoming Releases
- Configurable image scaling and Windows wallpaper mode
- Additional satellite sources (e.g., Himawari-8, Meteosat)
- Offline cache options
- Windows taskbar icon right-click “Refresh Now”
- Optional “auto-set on new source selection”
