# Installation Guide

## Prerequisites

Before installing Strategos, ensure you have:

- **Operating System**: Windows 10/11, macOS 10.15+, or Linux (Ubuntu 20.04+)
- **Disk Space**: ~100 MB for the application

## Download

Visit the [Releases page](https://github.com/ChrisClements1987/strategos/releases) and download the latest version for your platform:

### Windows
- Download: `Strategos_x.x.x_x64_en-US.msi`
- Double-click the installer and follow the prompts
- The app will be installed to `C:\Program Files\Strategos`

### macOS
- Download: `Strategos_x.x.x_x64.dmg`
- Open the DMG file
- Drag Strategos to your Applications folder
- First launch: Right-click → Open (to bypass Gatekeeper)

### Linux
**Debian/Ubuntu (.deb):**
```bash
sudo dpkg -i strategos_x.x.x_amd64.deb
```

**AppImage:**
```bash
chmod +x Strategos_x.x.x_amd64.AppImage
./Strategos_x.x.x_amd64.AppImage
```

## First Launch

1. Launch Strategos from your applications menu
2. The app will create a local database at:
   - **Windows**: `%APPDATA%\com.chrisclements.strategos\strategos.db`
   - **macOS**: `~/Library/Application Support/com.chrisclements.strategos/strategos.db`
   - **Linux**: `~/.local/share/com.chrisclements.strategos/strategos.db`

3. Start by creating your first portfolio!

## Updating

Download and install the latest version. Your data is preserved between updates.

## Uninstalling

- **Windows**: Use "Add or Remove Programs"
- **macOS**: Drag Strategos from Applications to Trash
- **Linux**: `sudo apt remove strategos` (or delete AppImage)

**Note**: Your database file is NOT deleted during uninstall. Delete manually if desired.
