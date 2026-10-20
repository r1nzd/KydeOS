use std::sync::Arc;
use smithay::{
    backend::winit::{self, WinitEvent},
    backend::renderer::{gles::GlesRenderer, Renderer, Frame},
    backend::renderer::damage::OutputDamageTracker,
    backend::renderer::element::texture::TextureRenderElement,
    backend::renderer::element::solid::SolidColorRenderElement,
    backend::renderer::Color32F,
    backend::renderer::element::{Kind, Id},
    backend::allocator::Fourcc,
    backend::renderer::ImportMem,
    desktop::space::render_output as space_render_output,
    desktop::space::{space_render_elements, SpaceRenderElements},
    backend::renderer::element::Wrap,
    backend::renderer::element::surface::WaylandSurfaceRenderElement,
    output::{Output, PhysicalProperties, Mode, Subpixel},
    reexports::calloop::EventLoop,
    reexports::wayland_server::{Display, ListeningSocket, backend::{ClientData, ClientId, DisconnectReason}},
    utils::{Rectangle, Transform, Point, Physical},
    wayland::compositor::CompositorClientState,
};
use crate::shell::taskbar::generate_taskbar_texture;
use crate::shell::titlebar::{generate_titlebar_texture, titlebar_button_rects};
use crate::compositor::TitlebarButton;
use image::GenericImageView;
use std::collections::HashMap;
use smithay::reexports::wayland_server::Resource;
use tracing::{info, warn};

use crate::compositor::{KydeCompositor, KydeClientState};
use crate::input::process_input_event;

smithay::backend::renderer::element::render_elements! {
    CustomElement<=GlesRenderer>;
    Texture=TextureRenderElement<smithay::backend::renderer::gles::GlesTexture>,
    Chip=SolidColorRenderElement,
    Space=SpaceRenderElements<GlesRenderer, WaylandSurfaceRenderElement<GlesRenderer>>,
}

pub fn run_winit(mut comp: KydeCompositor, mut display: Display<KydeCompositor>, mut event_loop: EventLoop<KydeCompositor>, accent_color_hex: String) {
    info!("Starting KydeShell with winit backend (windowed mode)...");

    let (mut backend, winit_source) = match winit::init::<GlesRenderer>() {
        Ok(ret) => ret,
        Err(err) => {
            warn!("Failed to initialize winit backend: {}", err);
            return;
        }
    };

    info!("Winit window created — KydeShell running in windowed mode");

    let listener = ListeningSocket::bind_auto("wayland", 1..32)
        .expect("Failed to bind wayland listening socket");
    let socket_name = listener.socket_name().unwrap().to_string_lossy().to_string();
    std::env::set_var("WAYLAND_DISPLAY", &socket_name);
    info!("Wayland socket bound: {}  (set WAYLAND_DISPLAY={} to connect a client)", socket_name, socket_name);

    let mut display_handle = display.handle();

    let initial_size = backend.window_size();

    let output = Output::new(
        "KYDE-1".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "KydeOS".into(),
            model: "KydeShell winit output".into(),
        },
    );
    let mode = Mode {
        size: (initial_size.w, initial_size.h).into(),
        refresh: 60_000,
    };
    output.change_current_state(Some(mode), Some(Transform::Flipped180), None, Some((0, 0).into()));
    output.set_preferred(mode);
    let _output_global = output.create_global::<KydeCompositor>(&display_handle);

    comp.space.map_output(&output, (0, 0));
    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    let taskbar_height: i32 = 46;
    let accent_rgb = {
        let hex = accent_color_hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0x67);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0x50);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0xA4);
        [r, g, b]
    };
    let taskbar_color: [u8; 4] = [accent_rgb[0], accent_rgb[1], accent_rgb[2], 0xEE];
    let mut taskbar_width: i32 = initial_size.w - (12.5 * 2.0) as i32;

    let taskbar_pixels = generate_taskbar_texture(taskbar_width as u32, taskbar_height as u32, taskbar_color);
    let mut taskbar_texture = backend
        .renderer()
        .import_memory(
            &taskbar_pixels,
            Fourcc::Abgr8888,
            (taskbar_width, taskbar_height).into(),
            false,
        )
        .expect("Failed to import taskbar texture");

    let titlebar_height: i32 = 32;
    let titlebar_max_width: i32 = 4000;
    let titlebar_bg: [u8; 4] = [0x2A, 0x2A, 0x2A, 0xFF];

    let wallpaper_img = image::open("/home/ace/kydeshell/assets/wallpaper.png")
        .expect("Failed to load wallpaper");
    let (wp_w, wp_h) = wallpaper_img.dimensions();
    let wallpaper_rgba = wallpaper_img.to_rgba8().into_raw();
    let wallpaper_texture = backend
        .renderer()
        .import_memory(
            &wallpaper_rgba,
            Fourcc::Abgr8888,
            (wp_w as i32, wp_h as i32).into(),
            false,
        )
        .expect("Failed to import wallpaper texture");

    let handle = event_loop.handle();
    handle
        .insert_source(winit_source, move |event, _, state| {
            match event {
                WinitEvent::Resized { size, .. } => {
                    info!("Window resized: {:?}", size);
                }
                WinitEvent::CloseRequested => {
                    info!("Close requested — stopping KydeShell");
                    state.stop();
                }
                WinitEvent::Input(event) => {
                    process_input_event(state, event);
                }
                _ => {}
            }
        })
        .expect("Failed to insert winit event source");

    info!("Entering KydeShell event loop...");
    let mut titlebar_cache: HashMap<u32, (i32, smithay::backend::renderer::gles::GlesTexture)> = HashMap::new();
    loop {
        if !comp.running {
            break;
        }

        if let Some(stream) = listener.accept().unwrap_or(None) {
            info!("New Wayland client connected: {:?}", stream);
            let _ = display_handle.insert_client(stream, Arc::new(KydeClientState::default()));
        }

        let size = backend.window_size();
        let has_maximized = !comp.maximized_windows.is_empty();
        let taskbar_x: i32 = if has_maximized { 0 } else { 12.5_f64.round() as i32 };
        let taskbar_y: i32 = if has_maximized {
            size.h - taskbar_height
        } else {
            ((size.h as f64) - (taskbar_height as f64) - 14.8).round() as i32
        };
        let current_taskbar_width: i32 = if has_maximized { size.w } else { size.w - (12.5 * 2.0) as i32 };
        if current_taskbar_width != taskbar_width {
            taskbar_width = current_taskbar_width;
            let new_pixels = if has_maximized {
                let mut buf = vec![0u8; (taskbar_width.max(1) * taskbar_height * 4) as usize];
                for i in (0..buf.len()).step_by(4) {
                    buf[i] = taskbar_color[0];
                    buf[i+1] = taskbar_color[1];
                    buf[i+2] = taskbar_color[2];
                    buf[i+3] = taskbar_color[3];
                }
                buf
            } else {
                generate_taskbar_texture(taskbar_width.max(1) as u32, taskbar_height as u32, taskbar_color)
            };
            taskbar_texture = backend
                .renderer()
                .import_memory(&new_pixels, Fourcc::Abgr8888, (taskbar_width.max(1), taskbar_height).into(), false)
                .expect("Failed to re-import taskbar texture after resize");
        }

        if let Ok((renderer, mut fb)) = backend.bind() {
            let taskbar_element = TextureRenderElement::from_static_texture(
                Id::new(),
                renderer.context_id(),
                Point::<f64, Physical>::from((taskbar_x as f64, taskbar_y as f64)),
                taskbar_texture.clone(),
                1,
                Transform::Normal,
                Some(1.0),
                None,
                None,
                None,
                Kind::Unspecified,
            );

            let chip_size: i32 = 32;
            let chip_gap: i32 = 8;
            let chip_y = taskbar_y + (taskbar_height - chip_size) / 2;
            let launcher_size: i32 = 32;
            let windows_snapshot: Vec<_> = comp.space.elements().cloned().collect();
            let mut all_windows_for_taskbar: Vec<_> = windows_snapshot.clone();
            all_windows_for_taskbar.extend(comp.minimized_windows.iter().map(|(w, _)| w.clone()));
            let group_width = launcher_size + chip_gap + all_windows_for_taskbar.len() as i32 * (chip_size + chip_gap);
            let launcher_x = taskbar_x + (taskbar_width - group_width) / 2;
            comp.launcher_button_rect = Rectangle::new((launcher_x, chip_y).into(), (launcher_size, launcher_size).into());
            let chip_start_x = launcher_x + launcher_size + chip_gap;
            comp.taskbar_chips = all_windows_for_taskbar
                .iter()
                .enumerate()
                .map(|(i, window)| {
                    let chip_x = chip_start_x + i as i32 * (chip_size + chip_gap);
                    let rect = Rectangle::new(
                        smithay::utils::Point::from((chip_x, chip_y)),
                        smithay::utils::Size::from((chip_size, chip_size)),
                    );
                    (rect, window.clone())
                })
                .collect();

            let launcher_button_element = SolidColorRenderElement::new(
                Id::new(),
                comp.launcher_button_rect,
                0usize,
                Color32F::from([1.0, 1.0, 1.0, 1.0]),
                smithay::backend::renderer::element::Kind::Unspecified,
            );

            let cc_btn_w: i32 = 42;
            let cc_btn_h: i32 = 28;
            let cc_btn_x = taskbar_x + taskbar_width - cc_btn_w - 12;
            let cc_btn_y = taskbar_y + (taskbar_height - cc_btn_h) / 2;
            comp.control_center_button_rect = Rectangle::new((cc_btn_x, cc_btn_y).into(), (cc_btn_w, cc_btn_h).into());
            let cc_button_color = if comp.control_center_open {
                Color32F::from([0.20, 0.22, 0.26, 1.0])
            } else {
                Color32F::from([0.36, 0.38, 0.44, 0.90])
            };
            let cc_button_element = SolidColorRenderElement::new(
                Id::new(),
                comp.control_center_button_rect,
                0usize,
                cc_button_color,
                smithay::backend::renderer::element::Kind::Unspecified,
            );

            let mut launcher_panel_elements: Vec<SolidColorRenderElement> = Vec::new();
            comp.launcher_app_rects.clear();
            if comp.launcher_open {
                let apps = crate::shell::launcher::get_default_apps();
                let cols = 4;
                let cell_size = 56;
                let icon_size = 44;
                let gap_x = 16;
                let gap_y = 16;
                let pad_x = 20;
                let pad_top = 36;
                let pad_bottom = 20;

                let rows = (apps.len() as i32 + cols - 1) / cols;
                let panel_w = pad_x * 2 + cols * cell_size + (cols - 1) * gap_x;
                let panel_h = pad_top + rows * cell_size + (rows - 1).max(0) * gap_y + pad_bottom;

                let panel_x = (taskbar_x + (taskbar_width - panel_w) / 2).max(12);
                let panel_y = taskbar_y - panel_h - 12;

                let border = 1;
                let border_rect = Rectangle::new(
                    (panel_x - border, panel_y - border).into(),
                    (panel_w + border * 2, panel_h + border * 2).into(),
                );
                let panel_rect = Rectangle::new((panel_x, panel_y).into(), (panel_w, panel_h).into());

                for (idx, app) in apps.iter().enumerate() {
                    let col = idx as i32 % cols;
                    let row = idx as i32 / cols;
                    let cell_x = panel_x + pad_x + col * (cell_size + gap_x);
                    let cell_y = panel_y + pad_top + row * (cell_size + gap_y);
                    let icon_x = cell_x + (cell_size - icon_size) / 2;
                    let icon_y = cell_y + (cell_size - icon_size) / 2;
                    let app_rect = Rectangle::new((icon_x, icon_y).into(), (icon_size, icon_size).into());

                    launcher_panel_elements.push(SolidColorRenderElement::new(
                        Id::new(),
                        app_rect,
                        0usize,
                        Color32F::from(app.color),
                        smithay::backend::renderer::element::Kind::Unspecified,
                    ));
                    comp.launcher_app_rects.push((app_rect, app.exec.clone()));
                }

                let search_bar_h = 14;
                launcher_panel_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    Rectangle::new((panel_x + pad_x, panel_y + 12).into(), (panel_w - pad_x * 2, search_bar_h).into()),
                    0usize,
                    Color32F::from([0.30, 0.32, 0.38, 0.95]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));

                launcher_panel_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    panel_rect,
                    0usize,
                    Color32F::from([0.18, 0.19, 0.23, 0.98]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));

                launcher_panel_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    border_rect,
                    0usize,
                    Color32F::from([0.38, 0.40, 0.48, 0.90]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));
            }

            let mut control_center_elements: Vec<SolidColorRenderElement> = Vec::new();
            comp.control_center_action_rects.clear();
            if comp.control_center_open {
                let cc_w = 340;
                let cc_h = 440;
                let cc_x = (taskbar_x + taskbar_width - cc_w).max(12);
                let cc_y = taskbar_y - cc_h - 12;

                let border = 1;
                let border_rect = Rectangle::new(
                    (cc_x - border, cc_y - border).into(),
                    (cc_w + border * 2, cc_h + border * 2).into(),
                );
                let panel_rect = Rectangle::new((cc_x, cc_y).into(), (cc_w, cc_h).into());

                let header_y = cc_y + 18;
                let clock_rect = Rectangle::new((cc_x + 20, header_y).into(), (84, 40).into());
                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    clock_rect,
                    0usize,
                    Color32F::from([0.16, 0.17, 0.20, 1.0]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));

                let battery_rect = Rectangle::new((cc_x + cc_w - 20 - 64, header_y + 6).into(), (64, 22).into());
                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    battery_rect,
                    0usize,
                    Color32F::from([0.16, 0.17, 0.20, 1.0]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));

                let slider_y = header_y + 50;
                let slider_h = 36;
                let slider_track_w = cc_w - 40;
                let slider_fill_w = ((slider_track_w as f32) * 0.7) as i32;
                let slider_fill_rect = Rectangle::new((cc_x + 20, slider_y).into(), (slider_fill_w, slider_h).into());
                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    slider_fill_rect,
                    0usize,
                    Color32F::from([0.18, 0.19, 0.22, 1.0]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));
                let slider_track_rect = Rectangle::new((cc_x + 20, slider_y).into(), (slider_track_w, slider_h).into());
                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    slider_track_rect,
                    0usize,
                    Color32F::from([0.62, 0.63, 0.66, 1.0]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));

                let tiles_y = slider_y + slider_h + 16;
                let tile_cols = 2;
                let tile_w = (cc_w - 40 - 12) / 2;
                let tile_h = 56;
                let tile_gap_x = 12;
                let tile_gap_y = 12;

                let tiles = crate::shell::control_center::get_default_tiles();
                for (idx, tile) in tiles.iter().enumerate() {
                    let col = idx as i32 % tile_cols;
                    let row = idx as i32 / tile_cols;
                    let tx = cc_x + 20 + col * (tile_w + tile_gap_x);
                    let ty = tiles_y + row * (tile_h + tile_gap_y);
                    let tile_rect = Rectangle::new((tx, ty).into(), (tile_w, tile_h).into());

                    let dot_rect = Rectangle::new((tx + 12, ty + 18).into(), (20, 20).into());
                    let dot_color = if tile.active {
                        Color32F::from([0.95, 0.95, 0.95, 1.0])
                    } else if tile.disabled {
                        Color32F::from([0.50, 0.51, 0.54, 1.0])
                    } else {
                        Color32F::from([0.80, 0.81, 0.84, 1.0])
                    };
                    control_center_elements.push(SolidColorRenderElement::new(
                        Id::new(),
                        dot_rect,
                        0usize,
                        dot_color,
                        smithay::backend::renderer::element::Kind::Unspecified,
                    ));

                    let tile_color = if tile.active {
                        Color32F::from([0.18, 0.19, 0.22, 1.0])
                    } else if tile.disabled {
                        Color32F::from([0.72, 0.73, 0.75, 1.0])
                    } else {
                        Color32F::from([0.55, 0.56, 0.59, 1.0])
                    };
                    control_center_elements.push(SolidColorRenderElement::new(
                        Id::new(),
                        tile_rect,
                        0usize,
                        tile_color,
                        smithay::backend::renderer::element::Kind::Unspecified,
                    ));
                    comp.control_center_action_rects.push((tile_rect, tile.id.to_string()));
                }

                let footer_y = cc_y + cc_h - 48;
                let settings_rect = Rectangle::new((cc_x + cc_w - 20 - 36 - 12 - 76, footer_y).into(), (36, 36).into());
                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    settings_rect,
                    0usize,
                    Color32F::from([0.55, 0.56, 0.59, 1.0]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));
                comp.control_center_action_rects.push((settings_rect, "settings".into()));

                let power_rect = Rectangle::new((cc_x + cc_w - 20 - 76, footer_y).into(), (76, 36).into());
                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    power_rect,
                    0usize,
                    Color32F::from([0.18, 0.19, 0.22, 1.0]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));
                comp.control_center_action_rects.push((power_rect, "power".into()));

                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    panel_rect,
                    0usize,
                    Color32F::from([0.80, 0.81, 0.83, 0.98]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));

                control_center_elements.push(SolidColorRenderElement::new(
                    Id::new(),
                    border_rect,
                    0usize,
                    Color32F::from([0.60, 0.62, 0.66, 0.90]),
                    smithay::backend::renderer::element::Kind::Unspecified,
                ));
            }

            let chip_elements: Vec<SolidColorRenderElement> = all_windows_for_taskbar
                .iter()
                .enumerate()
                .map(|(i, _window)| {
                    let chip_x = chip_start_x + i as i32 * (chip_size + chip_gap);
                    SolidColorRenderElement::new(
                        Id::new(),
                        Rectangle::new((chip_x, chip_y).into(), (chip_size, chip_size).into()),
                        0usize,
                        Color32F::from([0.65, 0.65, 1.0, 1.0]),
                        smithay::backend::renderer::element::Kind::Unspecified,
                    )
                })
                .collect();

            comp.titlebar_buttons.clear();
            let mut titlebar_elements: Vec<TextureRenderElement<_>> = Vec::new();

            for window in windows_snapshot.iter() {
                let win_geo = window.geometry();
                let win_pos = comp.space.element_location(window).unwrap_or((0, 0).into());
                let win_w = win_geo.size.w.min(titlebar_max_width);

                let clamped_x = win_pos.x;
                let clamped_y = win_pos.y - titlebar_height;

                let win_key = window.toplevel().map(|t| t.wl_surface().id().protocol_id()).unwrap_or(0);
                let need_regen = match titlebar_cache.get(&win_key) {
                    Some((cached_w, _)) => *cached_w != win_w,
                    None => true,
                };
                if need_regen {
                    let tb_pixels = generate_titlebar_texture(win_w.max(1) as u32, titlebar_height as u32, titlebar_bg);
                    let tb_texture = renderer
                        .import_memory(&tb_pixels, Fourcc::Abgr8888, (win_w.max(1), titlebar_height).into(), false)
                        .expect("Failed to import per-window titlebar texture");
                    titlebar_cache.insert(win_key, (win_w, tb_texture));
                }
                let tb_texture = titlebar_cache.get(&win_key).unwrap().1.clone();

                let tb_element = TextureRenderElement::from_static_texture(
                    Id::new(),
                    renderer.context_id(),
                    Point::<f64, Physical>::from((clamped_x as f64, clamped_y as f64)),
                    tb_texture,
                    1,
                    Transform::Normal,
                    Some(1.0),
                    None,
                    None,
                    None,
                    Kind::Unspecified,
                );
                titlebar_elements.push(tb_element);

                let btn_rects = titlebar_button_rects(win_w, titlebar_height);
                let btn_kinds = [TitlebarButton::Minimize, TitlebarButton::Maximize, TitlebarButton::Close];
                for (rect, kind) in btn_rects.iter().zip(btn_kinds.iter()) {
                    let abs_rect = Rectangle::new(
                        (clamped_x + rect.0, clamped_y + rect.1).into(),
                        (rect.2, rect.3).into(),
                    );
                    comp.titlebar_buttons.push((abs_rect, window.clone(), *kind));
                }
            }

            let img_aspect = wp_w as f64 / wp_h as f64;
            let screen_aspect = size.w as f64 / size.h as f64;

            let (src_w, src_h, src_x, src_y) = if img_aspect > screen_aspect {
                let visible_w = wp_h as f64 * screen_aspect;
                let crop_x = (wp_w as f64 - visible_w) / 2.0;
                (visible_w, wp_h as f64, crop_x, 0.0)
            } else {
                let visible_h = wp_w as f64 / screen_aspect;
                let crop_y = (wp_h as f64 - visible_h) / 2.0;
                (wp_w as f64, visible_h, 0.0, crop_y)
            };

            let wallpaper_element = TextureRenderElement::from_static_texture(
                Id::new(),
                renderer.context_id(),
                Point::<f64, Physical>::from((0.0, 0.0)),
                wallpaper_texture.clone(),
                1,
                Transform::Normal,
                Some(1.0),
                Some(Rectangle::new((src_x, src_y).into(), (src_w, src_h).into())),
                Some((size.w, size.h).into()),
                None,
                Kind::Unspecified,
            );

            let space_elements = space_render_elements(renderer, [&comp.space], &output, 1.0).unwrap_or_default();
            let mut all_elements: Vec<CustomElement> = Vec::new();
            all_elements.extend(control_center_elements.into_iter().map(CustomElement::Chip));
            all_elements.push(CustomElement::Chip(cc_button_element));
            all_elements.extend(launcher_panel_elements.into_iter().map(CustomElement::Chip));
            all_elements.push(CustomElement::Chip(launcher_button_element));
            all_elements.extend(chip_elements.into_iter().map(CustomElement::Chip));
            all_elements.push(CustomElement::Texture(taskbar_element));
            all_elements.extend(titlebar_elements.into_iter().map(CustomElement::Texture));
            all_elements.extend(space_elements.into_iter().map(CustomElement::Space));
            all_elements.push(CustomElement::Texture(wallpaper_element));

            let render_result = damage_tracker.render_output(
                renderer,
                &mut fb,
                0,
                &all_elements,
                [0.08, 0.08, 0.08, 1.0],
            );

            if render_result.is_err() {
                warn!("render_output failed");
            }

            comp.space.elements().for_each(|window| {
                window.send_frame(&output, comp.start_time.elapsed(), Some(std::time::Duration::ZERO), |_, _| Some(output.clone()));
            });
        }

        let _ = backend.submit(None);

        comp.space.refresh();
        let _ = display.dispatch_clients(&mut comp);
        let _ = display.flush_clients();

        if let Err(e) = event_loop.dispatch(std::time::Duration::from_millis(16), &mut comp) {
            warn!("Event loop error: {}", e);
            break;
        }
    }

    info!("KydeShell winit backend stopped");
}
