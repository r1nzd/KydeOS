<div align="center">

# KydeOS

A free, open-source operating system based on Debian, heavily customized with a Material 3 Expressive + Fluent UI. This project is built and maintained primarily as a solo development endeavor.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Version](https://img.shields.io/badge/Version-0.1--Aurora-purple)]()
[![Status](https://img.shields.io/badge/Status-Early%20Development-orange)]()
[![Base](https://img.shields.io/badge/Base-Debian%20Bookworm-red)]()
[![Arch](https://img.shields.io/badge/Arch-x86__64%20%7C%20ARM-green)]()

</div>

---

## IMPORTANT DISCLAIMER / WARNING
USE AT YOUR OWN RISK. This operating system is an experimental, personal project in its early stages of development.

- **No Warranty:** The system is provided "as is", without warranty of any kind.
- **Data Loss Risk:** Testing or installing this OS may lead to system instability, boot failures, or data loss. Ensure you back up all important data before attempting to run or install KydeOS.
- **Not for Production:** Do not use this as your primary operating system for daily, critical work.

## Key Features
- **Material 3 Expressive + Fluent UI/UX:** Comprehensive visual customization designed to provide a modern, sleek, and unified desktop experience.
- **KydeShell:** Custom desktop environment built from scratch in Rust.
- **Performance Optimization:** Stripped of unnecessary bloatware and background services, targeting ~300MB RAM idle and sub-10s boot time.
- **Debian Foundation:** Inherits the rock-solid stability and vast software repository of Debian Stable (Bookworm), running a Linux LTS kernel.
- **Wayland** display server with XWayland fallback. Supports x86_64 and ARM.
- **Inter** system font with a custom-designed icon set.
- **No Snap** — Native `.deb` + Flatpak + AppImage support only.
- **90+ languages** supported. Free forever, no ads, minimal telemetry (opt-in only).

## Built-in Apps
- **Krowser** — Custom browser built by the developer
- **Terminal** — Built-in terminal emulator
- **Files** — File manager
- **Text Editor** — Simple text editor
- **Settings** — System control panel
- **KydeStore** — Native app store

## Update Channels
- 🌙 **Nightly** — Automated nightly builds, most experimental
- 🧪 **Alpha** — Early testing, for developers
- 🔬 **Beta** — Feature-complete, community testing
- ✅ **Stable** — Official release for end users

## Build & Test Guide (For Devs / Contributors)
This project is built directly within a controlled Linux environment:

- **Environment:** WSL2 (Debian Bookworm)
- **Toolchain:** `live-build`, `debootstrap`
- **Testing:** ISO packaged and tested directly inside QEMU with OVMF (UEFI firmware)

```bash
git clone https://github.com/r1nzd/KydeOS.git
cd KydeOS
sudo apt install live-build debootstrap -y

lb config \
  --distribution bookworm \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --debian-installer none \
  --archive-areas "main contrib non-free non-free-firmware" \
  --bootloaders grub-efi \
  --uefi-secure-boot disable

sudo lb build
```

*(Note: Automated CI/CD pipelines are utilized for compiling and managing release builds via GitHub Actions.)*

## Roadmap
- [x] Bootable KydeOS base ISO
- [ ] Finalize the KydeOS 1.0 base build
- [ ] KydeShell v0.1 — custom Wayland compositor in Rust
- [ ] Stabilize Material 3 Expressive + Fluent UI/UX integration
- [ ] OOBE setup wizard
- [ ] Custom icon set
- [ ] KydeStore v0.1
- [ ] Krowser integration
- [ ] Custom installer (not Calamares)
- [ ] Establish a dedicated APT repository for system/app updates
- [ ] Develop exclusive app ecosystem
- [ ] ARM architecture support

## License
This project is released under the **GNU General Public License v3.0** ([LICENSE](LICENSE)).
