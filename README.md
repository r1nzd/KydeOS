<div align="center">

# KydeOS

A personal, experimental operating system based on Debian, heavily customized with a Material 3 Expressive UI. This project is built and maintained primarily as a solo development endeavor.

![License](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Version](https://img.shields.io/badge/Version-0.1--Aurora-purple)
![Status](https://img.shields.io/badge/Status-Early%20Development-orange)
![Base](https://img.shields.io/badge/Base-Debian%20Bookworm-red)
![Arch](https://img.shields.io/badge/Arch-x86__64%20%7C%20ARM-green)

</div>

---

## IMPORTANT DISCLAIMER / WARNING

USE AT YOUR OWN RISK. This operating system is an experimental, personal project in its early stages of development.

- **No Warranty:** The system is provided "as is", without warranty of any kind.
- **Data Loss Risk:** Testing or installing this OS may lead to system instability, boot failures, or data loss. Ensure you back up all important data before attempting to run or install KydeOS.
- **Not for Production:** Do not use this as your primary operating system for daily, critical work.

## Key Features

- **Material 3 Expressive UI/UX:** Comprehensive visual customization designed to provide a modern, sleek, and unified desktop experience.
- **KydeShell:** Custom desktop environment built from scratch in Rust, using the Smithay framework for a Wayland compositor with XWayland fallback.
- **Performance Optimization:** Stripped of unnecessary bloatware and background services, targeting ~300MB idle RAM and sub-10s boot time.
- **Debian Foundation:** Inherits the rock-solid stability and vast software repository of Debian Stable (Bookworm), running a Linux LTS kernel.
- **Typography & Icons:** Google Sans Flex system font paired with a custom-designed icon set.
- **Packages:** Native `.deb` + Flatpak + AppImage support. Snap is intentionally not included.
- **Localization:** 96 languages supported. Free forever, no ads, minimal telemetry (opt-in only).

## Built-in Apps

- **Krowser** — custom browser built by the developer
- **Terminal**, **Files**, **Text Editor**, **Settings**
- **KydeStore** — native app store (planned)

## Update Channels

- **Nightly** — automated nightly builds, most experimental
- **Alpha** — early testing, for developers
- **Beta** — feature-complete, community testing
- **Stable** — official release for end users

## Build & Test Guide (For Devs / Contributors)

This project is built directly within a controlled Linux environment:

- **Environment:** WSL2 (Debian)
- **Toolchain:** `live-build`, `debootstrap` (`lb config` / `lb build`)
- **Testing:** ISO packaged and tested directly inside QEMU with OVMF (UEFI firmware)

*(Note: Automated CI/CD pipelines are utilized for compiling and managing release builds via GitHub Actions.)*

## Roadmap

- [x] Bootable KydeOS base ISO
- [ ] Finalize the KydeOS 1.0 base build
- [ ] KydeShell v0.1 — custom Wayland compositor in Rust
- [ ] Stabilize Material 3 Expressive UI/UX integration
- [ ] OOBE (Out-of-Box Experience) setup wizard
- [ ] Develop exclusive app ecosystem (Planned)
- [ ] Establish a dedicated APT repository for system/app updates

## License

This project is released under the **GNU General Public License v3.0** ([LICENSE](LICENSE)).
