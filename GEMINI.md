# KydeOS / KydeShell - Project Knowledge

## Quy tắc làm việc
- Trả lời bằng tiếng Việt, không chào hỏi, không dẫn dắt. Tài liệu hướng ra ngoài (README, docs) viết bằng tiếng Anh.
- Trả lời ngắn: tối đa 5 gạch đầu dòng, không bảng, không tiêu đề, không vẽ cây thư mục trừ khi được yêu cầu.
- Không thêm comment (`//` hoặc `#`) vào code.
- Sửa file bằng `sed` hoặc `python3 -c` inline. Không in lại cả file, không dùng heredoc `cat << EOF` (trừ khi tạo file mới hoàn toàn, và khi đó cũng không có comment).
- Giải thích 2-3 gạch đầu dòng ngắn bên ngoài khối lệnh.
- Làm bản tối giản, tái sử dụng asset có sẵn. Chỉ làm đúng phần được yêu cầu.
- Dự án solo open source: chi phí thay đổi là thời gian và công bảo trì, không phải tiền.

## Dự án
- KydeOS: distro Linux mã nguồn mở của Acee (GitHub `r1nzd`, repo `github.com/r1nzd/KydeOS`), GPL-3.0. Công khai dùng bút danh `r1nzd`.
- Header bản quyền: `Copyright (C) 2025-2026  r1nzd`, không có "All Rights Reserved".
- Nền tảng: Debian 13 Trixie Stable. Một số tài liệu cũ ghi Bookworm, coi là lỗi thời.
- KydeShell: desktop shell + Wayland compositor tự viết bằng Rust, Smithay 0.6.0, Rust 1.96.0, XWayland fallback.
- Bản đầu mang tên Aurora (v0.1.0). Triết lý thương hiệu: "Kyde = kind".
- Sản phẩm đi kèm: Krowser (trình duyệt Rust/Tauri), KydeStore (kho ứng dụng native), KBar (taskbar/dock).
- Logo: hai hình quarter-circle (rỗng = 0, đặc = 1), cố định vĩnh viễn. Mascot: Zee (vòng tròn rỗng = 0) và One (vòng tròn đặc = 1).

## Môi trường
- Build trong WSL2 Debian 13, user `ace`, hostname `Admin`, WSL nằm ở `D:\WSL\Debian`.
- `~/kydeos`: live-build (`lb config` / `lb build`). `~/kydeshell`: code Rust của KydeShell.
- Test ISO bằng QEMU + OVMF, tắt UEFI secure boot.
- CI/CD: GitHub Actions. Giới hạn RAM WSL cấu hình trong `C:\Users\Admin\.wslconfig` (tránh mksquashfs crash).
- Không mở tab PowerShell Admin trong Windows Terminal khi dev (gây lỗi null byte `^@^@^@`).
- Thư viện hệ thống: libwayland-dev, libxkbcommon-dev, libudev-dev, libinput-dev, libgbm-dev, libdrm-dev, libdbus-1-dev, libsystemd-dev, libseat-dev, libpixman-1-dev, libegl1, libegl-mesa0.
- Không đụng vào `chroot/`, `cache/`, `binary/`, `.build/` trong `~` (rootfs live-build).

## Quyết định thiết kế đã chốt
- Ngôn ngữ thiết kế: chỉ Material 3 Expressive (M3E). Bỏ hẳn Fluent.
- Font: Google Sans Flex (variable) + Google Sans Text cho nội dung.
- Icon hệ thống và app có sẵn: Material Symbols Rounded, kiểu filled, kích thước chuẩn hóa. Icon riêng chỉ cho app first-party (Krowser, KydeStore, Terminal, Files, Settings, Text Editor).
- Màu accent lấy động từ wallpaper (Material You / Monet), không hardcode màu.
- Card/button group dùng shape morphing: góc ngoài bo lớn (~28px), góc tiếp giáp nhỏ (~4-6px), có khe nhỏ giữa các phần tử.
- Hover chỉ làm tối nền nhẹ, không scale. Trạng thái disabled dùng grayscale, không ẩn.
- Chỉ báo độ mạnh mật khẩu không tuân thủ M3E, không dùng.
- Text field OOBE: helper text mặc định `*required`, chuyển sang lỗi đỏ + icon `!` chỉ khi phát hiện input sai.

## Quyết định kỹ thuật đã chốt
- Wayland trước, XWayland dự phòng. Kernel Linux LTS, init systemd.
- Gói: `.deb`, Flatpak, AppImage. Không Snap.
- Mục tiêu: RAM idle ~300MB, boot dưới 10 giây. Kiến trúc x86_64 + ARM, UEFI + Legacy BIOS.
- LUKS2 mã hóa mặc định bật. Telemetry opt-in, mặc định tắt.
- Chu kỳ phát hành ~6 tháng; bản lớn ~2 năm theo Debian Stable.
- Kênh cập nhật công khai: Nightly, Beta, Stable. Alpha chỉ nội bộ.
- KydeStore Developer Program hoàn toàn khác kênh cập nhật Alpha của OS. Không nhầm lẫn dù cùng chữ "Alpha".
- Phân vùng mặc định tách `/` và `/home` để nâng bản lớn không mất dữ liệu (format `/`, giữ `/home`). Không có tùy chọn "tất cả trong một phân vùng". Timeshift làm lớp bảo vệ phụ.
- i18n: Fluent (`fluent-rs`, file `.ftl`) trong `/usr/share/kydeos/i18n/`, dùng chung cho OOBE, KydeShell, Krowser, KydeStore. Ngôn ngữ KydeShell đọc từ `~/.config/kydeshell/config.toml`, dự phòng `LANG`. Làm Anh + Việt trước.

## Smithay 0.6.0
- `SeatState`, `SeatHandler` nằm ở `smithay::input::`.
- Dùng `ClientDndGrabHandler`, không phải `WaylandDndGrabHandler`.
- `data_device_state()` nhận `&self` bất biến.
- `CompositorClientState` cần struct bọc `KydeClientState`.
- `BufferHandler` implement thủ công.
- `backend.bind()` destructure thành `(renderer, mut fb)`.
- `renderer.render()` nhận `(&mut framebuffer, size, Transform)`.

## Trạng thái KydeShell
- Xong: Server-Side Decorations, trích màu Material You, vẽ wallpaper (cover-scale), quản lý cửa sổ (Minimize/Maximize/Close, xdg đầy đủ), KBar chuyển giữa dạng pill và flat full-width theo trạng thái maximize.
- App Drawer / Launcher: hoàn thiện panel grid ứng dụng, quét .desktop, click mở app và spawn process theo exec
- Đã sửa: lỗi z-order trong render_output — Smithay render [0]=foreground, [last]=background. Thứ tự đúng: launcher_panel → launcher_button → chips → taskbar → titlebars → windows → wallpaper. Winit 0.30.13 patch: key.saturating_add(8) tránh overflow trên WSL2.
- Control Center (kiểu Quick Settings của Pixel) và App Drawer là hai tính năng khác nhau. Control Center gồm toggle Wifi, Bluetooth, Flashlight, Do Not Disturb, Alarm, Airplane, Device controls, Wallet, thanh sáng, đồng hồ/pin/ngày, nút tắt máy.

## OOBE
- Phân vùng: Guided (tự chia `/` + `/home`, LUKS2 mặc định) hoặc Manual. Không LVM. Guided có hai kiểu: "Separate /home" và "Separate /home, /var and /tmp".
- Tài khoản: màn Hostname, rồi Username + Password, rồi hộp thoại Auto-login. Không có Full name, không Avatar.
- Hiệu ứng loading: morph hình vector M3E bằng code (không video): một hình xoay liên tục và biến đổi qua nhiều hình (sao nhiều cánh, blob, ngũ giác bo tròn) bằng màu accent, nội suy hình học theo % tiến trình.
- Giọng văn ấm, gần gũi (ví dụ "KydeOS sẽ sống ở đâu?"). Thao tác rủi ro không thể hoàn tác (format đĩa, mã hóa) vẫn phải rõ ràng tuyệt đối.
- Chọn ngôn ngữ phải áp dụng ngay trong OOBE; KydeShell có thể cần khởi động lại. OOBE ghi locale và bố cục bàn phím bằng `localectl`.

## Việc sắp tới
- Control Center (Quick Settings Pixel M3E).
- Control Center, rồi toàn bộ OOBE.
- Easter egg Aurora: chạm logo KydeOS 5 lần trong Settings > About, hiện toast với Zee và One, tái dùng SVG có sẵn, không animation mới.
- Website / GitHub Pages (đã có landing page HTML, deploy từng lỗi), Discord cộng đồng, video teaser dưới 15 giây kiểu "vô tình lộ".
