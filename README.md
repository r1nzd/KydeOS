<div align="center">

# KydeOS — Aurora

**A free, open-source Linux operating system built on Debian**  
Designed with Material 3 Expressive + Fluent design language, powered by a custom shell written in Rust.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Version](https://img.shields.io/badge/Version-0.1--Aurora-purple)]()
[![Status](https://img.shields.io/badge/Status-Early%20Development-orange)]()
[![Base](https://img.shields.io/badge/Base-Debian%20Bookworm-red)]()

</div>

---

## Overview

KydeOS is a free, open-source Linux-based operating system built on Debian Stable. It aims to bridge the gap between rock-solid stability and a clean, modern desktop experience — without the bloat.

KydeOS features a custom desktop environment called **KydeShell**, written from scratch in Rust, with a design language that blends Material 3 Expressive and Microsoft Fluent into something uniquely its own.

---

## Key Features

- **Custom Desktop Environment** — KydeShell, built from scratch in Rust
- **Material 3 Expressive + Fluent** design language
- **Inter** as the system font
- **Self-designed icon set**
- **Wayland** display server with XWayland fallback
- **Debian Bookworm** base with Linux LTS kernel
- **Lightweight** — targeting ~100MB RAM idle, sub-10s boot time
- **UEFI + Legacy BIOS** support (GPT + MBR)
- **x86_64 + ARM** architecture support
- **90+ languages** supported
- **No Snap** — `.deb` native + Flatpak + AppImage
- **Free** — no cost, no ads, no telemetry (opt-in only)

---

## Built-in Apps

| App | Description |
|-----|-------------|
| **Krowser** | Custom browser built by the developer |
| **Terminal** | Built-in terminal emulator |
| **Files** | File manager |
| **Text Editor** | Simple text editor |
| **Settings** | System control panel |
| **KydeStore** | Native app store |

---

## Update Channels

| Channel | Description |
|---------|-------------|
| 🌙 **Nightly** | Automated nightly builds, most experimental |
| 🧪 **Alpha** | Early testing, for developers |
| 🔬 **Beta** | Feature-complete, community testing |
| ✅ **Stable** | Official release for end users |

---

## Technical Specifications

| Component | Choice |
|-----------|--------|
| Base | Debian Bookworm (12) |
| Kernel | Linux LTS |
| Init | systemd |
| Display | Wayland + XWayland |
| Shell | KydeShell (Rust) |
| Audio | PipeWire |
| Packages | .deb + Flatpak + AppImage |
| Boot | UEFI (GPT) + Legacy BIOS (MBR) |
| Arch | x86_64, ARM |
| Font | Inter |
| License | GPL-3.0 |

---

## Build Guide

### Requirements

- Debian/Ubuntu Linux (or WSL2 with Debian)
- `live-build`, `debootstrap`

### Setup

```bash
# Clone the repo
git clone https://github.com/r1nzd/KydeOS.git
cd KydeOS

# Install dependencies
sudo apt install live-build debootstrap -y

# Configure
lb config \
  --distribution bookworm \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --debian-installer none \
  --archive-areas "main contrib non-free non-free-firmware" \
  --bootloaders grub-efi \
  --uefi-secure-boot disable

# Build
sudo lb build
```

### Testing

```bash
# Test with QEMU
qemu-system-x86_64 \
  -cdrom live-image-amd64.hybrid.iso \
  -m 2G \
  -boot d \
  -bios /usr/share/ovmf/OVMF.fd \
  -vga std
```

---

## Roadmap

- [x] Bootable base ISO (Debian + Linux LTS)
- [x] GitHub repository setup
- [ ] KydeShell v0.1 (Rust, Wayland compositor)
- [ ] KydeOS branding (hostname, os-release, GRUB theme)
- [ ] OOBE (Out-of-box experience) setup wizard
- [ ] Custom icon set
- [ ] KydeStore v0.1
- [ ] Krowser integration
- [ ] Installer (custom, not Calamares)
- [ ] ARM build
- [ ] KydeOS 1.0 stable release

---

## Community

- 💬 **Discord** — coming soon
- 🐛 **Issues** — [GitHub Issues](https://github.com/r1nzd/KydeOS/issues)

---

## License

KydeOS is licensed under the **GNU General Public License v3.0**.  
See [LICENSE](LICENSE) for details.

> This project is experimental and provided "as is" without warranty of any kind.  
> Data loss risk exists — always test in a virtual machine first.

---

<div align="center">
Made with ❤️ by <a href="https://github.com/r1nzd">r1nzd</a>
</div>
