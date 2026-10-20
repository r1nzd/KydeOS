use tracing::info;
use smithay::{
    backend::input::{
        InputEvent, InputBackend, Event, KeyboardKeyEvent,
        PointerButtonEvent, PointerMotionAbsoluteEvent, ButtonState,
        AbsolutePositionEvent,
    },
    input::{keyboard::FilterResult, pointer::{ButtonEvent, MotionEvent}},
    utils::{SERIAL_COUNTER, Point, Logical},
    wayland::seat::WaylandFocus,
};
use crate::compositor::KydeCompositor;
use smithay::utils::Size;

pub fn process_input_event<B: InputBackend>(comp: &mut KydeCompositor, event: InputEvent<B>) {
    match event {
        InputEvent::Keyboard { event } => {
            let keycode = event.key_code();
            let state = event.state();
            let serial = SERIAL_COUNTER.next_serial();
            let time = event.time_msec();

            if let Some(keyboard) = comp.seat.get_keyboard() {
                keyboard.input::<(), _>(
                    comp,
                    keycode,
                    state,
                    serial,
                    time,
                    |_, _, _| FilterResult::Forward,
                );
            }
        }
        InputEvent::PointerMotionAbsolute { event } => {
            let output_size = comp
                .space
                .outputs()
                .next()
                .and_then(|o| o.current_mode())
                .map(|m| (m.size.w as f64, m.size.h as f64))
                .unwrap_or((1280.0, 800.0));

            let pos: Point<f64, Logical> = event.position_transformed(
                (output_size.0 as i32, output_size.1 as i32).into(),
            );
            comp.pointer_location = pos;

            if let Some((window, start_pointer, start_window_pos)) = comp.moving_window.clone() {
                let delta = pos - start_pointer;
                let mut new_x = start_window_pos.x + delta.x.round() as i32;
                let mut new_y = start_window_pos.y + delta.y.round() as i32;

                let output_size = comp
                    .space
                    .outputs()
                    .next()
                    .and_then(|o| o.current_mode())
                    .map(|m| (m.size.w, m.size.h))
                    .unwrap_or((1280, 800));

                let win_w = window.geometry().size.w;
                let titlebar_height = 32;
                let min_visible: i32 = 100;

                if new_y < titlebar_height {
                    new_y = titlebar_height;
                }
                if new_x + win_w < min_visible {
                    new_x = min_visible - win_w;
                }
                if new_x > output_size.0 - min_visible {
                    new_x = output_size.0 - min_visible;
                }
                if new_y > output_size.1 - 40 {
                    new_y = output_size.1 - 40;
                }

                comp.space.map_element(window, (new_x, new_y), false);
            }

            if let Some((window, start_pointer, (start_w, start_h))) = comp.resizing_window.clone() {
                let delta = pos - start_pointer;
                let new_w = (start_w + delta.x.round() as i32).max(100);
                let new_h = (start_h + delta.y.round() as i32).max(100);

                if let Some(toplevel) = window.toplevel() {
                    toplevel.with_pending_state(|state| {
                        state.size = Some(Size::from((new_w, new_h)));
                    });
                    toplevel.send_configure();
                }
            }

            if comp.moving_window.is_none() && comp.resizing_window.is_none() {
                let under = comp
                    .space
                    .element_under(pos)
                    .map(|(window, loc)| (window.wl_surface().unwrap().into_owned(), loc));

                if let Some(pointer) = comp.seat.get_pointer() {
                    let serial = SERIAL_COUNTER.next_serial();
                    let time = event.time_msec();
                    pointer.motion(
                        comp,
                        under.map(|(surf, loc)| (surf, loc.to_f64())),
                        &MotionEvent { location: pos, serial, time },
                    );
                }
            }
        }
        InputEvent::PointerButton { event } => {
            let serial = SERIAL_COUNTER.next_serial();
            let button = event.button_code();
            let state = event.state();
            let time = event.time_msec();
            if state == ButtonState::Pressed {
                let pos = comp.pointer_location;

                let pos_i32 = pos.to_i32_round();
                let pos_tuple = (pos_i32.x, pos_i32.y);
                if comp.control_center_button_rect.contains(pos_tuple) {
                    comp.control_center_open = !comp.control_center_open;
                    if comp.control_center_open {
                        comp.launcher_open = false;
                    }
                    return;
                }
                if comp.control_center_open {
                    if let Some((_, action)) = comp.control_center_action_rects.iter().find(|(r, _)| r.contains(pos_tuple)).cloned() {
                        if action == "power" {
                            std::process::Command::new("systemctl").arg("poweroff").spawn().ok();
                        } else if action == "settings" {
                            let display_env = std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-1".into());
                            std::process::Command::new("foot").arg("-e").arg("htop").env("WAYLAND_DISPLAY", display_env).spawn().ok();
                        }
                        comp.control_center_open = false;
                        return;
                    }
                    comp.control_center_open = false;
                }
                if comp.launcher_button_rect.contains(pos_tuple) {
                    comp.launcher_open = !comp.launcher_open;
                    if comp.launcher_open {
                        comp.control_center_open = false;
                    }
                    return;
                }
                if comp.launcher_open {
                    if let Some((_, exec)) = comp.launcher_app_rects.iter().find(|(r, _)| r.contains(pos_tuple)).cloned() {
                        let parts: Vec<&str> = exec.split_whitespace().collect();
                        if !parts.is_empty() {
                            let mut cmd = std::process::Command::new(parts[0]);
                            if parts.len() > 1 {
                                cmd.args(&parts[1..]);
                            }
                            let display_env = std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-1".into());
                            cmd.env("WAYLAND_DISPLAY", display_env).spawn().ok();
                        }
                        comp.launcher_open = false;
                        return;
                    }
                    comp.launcher_open = false;
                }
                let titlebar_hit = comp
                    .titlebar_buttons
                    .iter()
                    .find(|(rect, _window, _kind)| rect.contains(pos.to_i32_round()))
                    .map(|(_rect, window, kind)| (window.clone(), *kind));

                if let Some((window, kind)) = titlebar_hit {
                    match kind {
                        crate::compositor::TitlebarButton::Close => {
                            if let Some(toplevel) = window.toplevel() {
                                toplevel.send_close();
                            }
                        }
                        crate::compositor::TitlebarButton::Minimize => {
                            if let Some(window_pos) = comp.space.element_location(&window) {
                                comp.minimized_windows.push((window.clone(), window_pos));
                                comp.space.unmap_elem(&window);
                            }
                        }
                        crate::compositor::TitlebarButton::Maximize => {
                            if let Some(idx) = comp.maximized_windows.iter().position(|(w, _, _)| w == &window) {
                                let (_, old_pos, old_size) = comp.maximized_windows.remove(idx);
                                if let Some(toplevel) = window.toplevel() {
                                    toplevel.with_pending_state(|state| {
                                        state.size = Some(old_size.into());
                                        state.states.unset(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Maximized);
                                    });
                                    toplevel.send_configure();
                                }
                                comp.space.map_element(window.clone(), old_pos, false);
                            } else {
                                let win_pos = comp.space.element_location(&window).unwrap_or((0, 0).into());
                                let win_geo = window.geometry();
                                let output_size = comp
                                    .space
                                    .outputs()
                                    .next()
                                    .and_then(|o| o.current_mode())
                                    .map(|m| (m.size.w, m.size.h))
                                    .unwrap_or((1280, 800));

                                comp.maximized_windows.push((window.clone(), win_pos, (win_geo.size.w, win_geo.size.h)));

                                let titlebar_h = 32;
                                let kbar_reserved = 46;
                                let new_size = (output_size.0, output_size.1 - titlebar_h - kbar_reserved);
                                if let Some(toplevel) = window.toplevel() {
                                    toplevel.with_pending_state(|state| {
                                        state.size = Some(new_size.into());
                                        state.states.set(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Maximized);
                                    });
                                    toplevel.send_configure();
                                }
                                comp.space.map_element(window.clone(), (0, titlebar_h), false);
                            }
                        }
                    }
                    return;
                }

                let chip_hit = comp
                    .taskbar_chips
                    .iter()
                    .find(|(rect, _window)| rect.contains(pos.to_i32_round()))
                    .map(|(_rect, window)| window.clone());

                if let Some(window) = chip_hit {
                    if let Some(idx) = comp.minimized_windows.iter().position(|(w, _)| w == &window) {
                        let (_, saved_pos) = comp.minimized_windows.remove(idx);
                        comp.space.map_element(window.clone(), saved_pos, false);
                    }
                    comp.space.raise_element(&window, true);
                    for w in comp.space.elements() {
                        if let Some(toplevel) = w.toplevel() {
                            let activated = w == &window;
                            toplevel.with_pending_state(|state| {
                                if activated {
                                    state.states.set(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated);
                                } else {
                                    state.states.unset(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated);
                                }
                            });
                            toplevel.send_configure();
                        }
                    }
                    if let Some(wl_surface) = window.wl_surface() {
                        if let Some(keyboard) = comp.seat.get_keyboard() {
                            keyboard.set_focus(comp, Some(wl_surface.into_owned()), serial);
                        }
                    }
                    return;
                }

                let under = comp
                    .space
                    .element_under(pos)
                    .map(|(window, _loc)| window.clone());

                if let Some(window) = under {
                    comp.space.raise_element(&window, true);
                    for w in comp.space.elements() {
                        if let Some(toplevel) = w.toplevel() {
                            let activated = w == &window;
                            toplevel.with_pending_state(|state| {
                                if activated {
                                    state.states.set(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated);
                                } else {
                                    state.states.unset(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated);
                                }
                            });
                            toplevel.send_configure();
                        }
                    }
                    if let Some(wl_surface) = window.wl_surface() {
                        let keyboard = comp.seat.get_keyboard();
                        if let Some(keyboard) = keyboard {
                            let ctrl_held = keyboard.modifier_state().ctrl;
                            keyboard.set_focus(comp, Some(wl_surface.into_owned()), serial);

                            if ctrl_held && button == 272 {
                                if let Some(window_pos) = comp.space.element_location(&window) {
                                    comp.moving_window = Some((window.clone(), pos, window_pos));
                                }
                            } else if ctrl_held && button == 273 {
                                let geo = window.geometry();
                                comp.resizing_window = Some((window.clone(), pos, (geo.size.w, geo.size.h)));
                            }
                        }
                    }
                }
            } else if state == ButtonState::Released {
                comp.moving_window = None;
                comp.resizing_window = None;
            }

            if let Some(pointer) = comp.seat.get_pointer() {
                pointer.button(
                    comp,
                    &ButtonEvent { button, state, serial, time },
                );
            }
        }
        _ => {}
    }
}

