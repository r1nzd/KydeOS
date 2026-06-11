# KydeOS

A personal, experimental operating system based on Debian, heavily customized with a Material 3 UI. This project is built and maintained primarily as a solo development endeavor.

---

### IMPORTANT DISCLAIMER / WARNING

USE AT YOUR OWN RISK. This operating system is an experimental, personal project in its early stages of development. 
- No Warranty: The system is provided "as is", without warranty of any kind. 
- Data Loss Risk: Testing or installing this OS may lead to system instability, boot failures, or data loss. Ensure you back up all important data before attempting to run or install KydeOS.
- Not for Production: Do not use this as your primary operating system for daily, critical work. 

---

## Key Features

- Material 3 UI/UX: Comprehensive visual customization designed to provide a modern, sleek, and unified desktop experience.
- Performance Optimization: Stripped of unnecessary bloatware and background services, aiming for a lightweight and smooth operating system.
- Debian Foundation: Inherits the rock-solid stability and vast software repository of Debian Stable.

## Build & Test Guide (For Devs / Contributors)

This project is built directly within a controlled Linux environment:
1. Environment: WSL2 (Debian)
2. Customization: Rootfs direct modification.
3. Testing: Exported rootfs packaged and tested directly inside QEMU.

(Note: Automated CI/CD pipelines are utilized for compiling and managing release builds via GitHub Actions).

## Roadmap

- [x] Finalize the KydeOS 1.0 base build
- [ ] Stabilize Material 3 UI/UX integration
- [ ] Develop exclusive app ecosystem (Planned)
- [ ] Establish a dedicated APT repository for system/app updates

## License

This project is released under the GNU General Public License v3.0 (LICENSE).
