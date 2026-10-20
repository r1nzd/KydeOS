use tracing::{info, warn, error};
use std::os::fd::OwnedFd;
use smithay::{
    backend::session::{Session, libseat::LibSeatSession},
    backend::udev::{UdevBackend, primary_gpu},
    backend::drm::{DrmDevice, DrmDeviceFd},
    backend::allocator::gbm::{GbmDevice, GbmAllocator, GbmBufferFlags},
    reexports::drm::buffer::DrmFourcc,
    backend::egl::{EGLDisplay, EGLContext},
    backend::renderer::gles::GlesRenderer,
    backend::renderer::{Bind, Renderer, Frame, ImportDma},
    backend::drm::GbmBufferedSurface,
    utils::{Transform, Rectangle},
    reexports::{
        calloop::EventLoop,
        gbm::Device as GbmDeviceHandle,
        drm::control::{Device as DrmControlDevice, connector::State as ConnectorState},
    },
};
use rustix::fs::OFlags;
use smithay::backend::libinput::{LibinputInputBackend, LibinputSessionInterface};
use smithay::reexports::input::Libinput;
use smithay::reexports::calloop::LoopHandle;
use crate::input::process_input_event;

use crate::compositor::KydeCompositor;

pub fn run_drm(_comp: KydeCompositor, _event_loop: EventLoop<KydeCompositor>) {
    info!("Starting KydeShell with DRM/KMS backend (bare metal mode)...");

    let (mut session, _session_notifier) = match LibSeatSession::new() {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to initialize libseat session: {}", err);
            error!("Đảm bảo seatd đang chạy và user thuộc nhóm 'seat', hoặc chạy trên logind session thật.");
            return;
        }
    };
    info!("libseat session initialized, seat name: {}", session.seat());

    let udev_backend = match UdevBackend::new(session.seat()) {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to initialize udev backend: {}", err);
            return;
        }
    };
    info!("udev backend initialized for seat: {}", session.seat());

    let mut libinput_context = Libinput::new_with_udev::<LibinputSessionInterface<LibSeatSession>>(
        session.clone().into(),
    );
    if let Err(_) = libinput_context.udev_assign_seat(&session.seat()) {
        error!("Failed to assign udev seat to libinput context");
        return;
    }
    let libinput_backend = LibinputInputBackend::new(libinput_context.clone());
    info!("libinput backend initialized and assigned to seat: {}", session.seat());

    let loop_handle: LoopHandle<KydeCompositor> = _event_loop.handle();
    if let Err(err) = loop_handle.insert_source(libinput_backend, move |event, _, comp| {
        process_input_event(comp, event);
    }) {
        error!("Failed to insert libinput event source into event loop: {}", err);
        return;
    }
    info!("libinput event source registered with event loop");

    let gpu_path = match primary_gpu(&session.seat()) {
        Ok(Some(path)) => {
            info!("Primary GPU found: {:?}", path);
            Some(path)
        }
        Ok(None) => {
            warn!("No primary GPU marked by udev, scanning for first available DRM device...");
            udev_backend
                .device_list()
                .next()
                .map(|(_id, path)| path.to_path_buf())
        }
        Err(err) => {
            error!("Failed to query primary GPU via udev: {}", err);
            None
        }
    };

    let gpu_path = match gpu_path {
        Some(p) => {
            info!("Using GPU device: {:?}", p);
            p
        }
        None => {
            error!("No DRM/GPU device found at all — cannot continue DRM backend init");
            return;
        }
    };

    let open_flags = OFlags::RDWR | OFlags::CLOEXEC | OFlags::NONBLOCK;
    let fd: OwnedFd = match session.open(&gpu_path, open_flags) {
        Ok(fd) => fd,
        Err(err) => {
            error!("Failed to open DRM device {:?} via session: {}", gpu_path, err);
            return;
        }
    };
    let drm_fd = DrmDeviceFd::new(fd.into());

    let (mut drm_device, _drm_notifier) = match DrmDevice::new(drm_fd.clone(), true) {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to create DrmDevice: {}", err);
            return;
        }
    };
    info!("DrmDevice created successfully for {:?}", gpu_path);

    let gbm_device = match GbmDeviceHandle::new(drm_fd.clone()) {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to create GBM device: {}", err);
            return;
        }
    };
    let gbm_allocator = GbmDevice::new(gbm_device.clone()).expect("Failed to create GBM allocator wrapper");
    info!("GBM allocator created successfully");

    let egl_display = match unsafe { EGLDisplay::new(gbm_device.clone()) } {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to create EGL display from GBM device: {}", err);
            return;
        }
    };
    let egl_context = match EGLContext::new(&egl_display) {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to create EGL context: {}", err);
            return;
        }
    };
    info!("EGL display + context created successfully");

    let resource_handles = match drm_device.resource_handles() {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to get DRM resource handles: {}", err);
            return;
        }
    };

    let mut chosen_connector = None;
    let mut chosen_mode = None;

    for &conn_handle in resource_handles.connectors() {
        let conn_info = match drm_device.get_connector(conn_handle, false) {
            Ok(info) => info,
            Err(err) => {
                warn!("Failed to get connector info for {:?}: {}", conn_handle, err);
                continue;
            }
        };

        if conn_info.state() == ConnectorState::Connected {
            info!(
                "Found connected output: {:?} ({:?})",
                conn_info.interface(),
                conn_info.interface_id()
            );

            let modes = conn_info.modes();
            let preferred = modes.iter().find(|m| m.mode_type().contains(smithay::reexports::drm::control::ModeTypeFlags::PREFERRED));
            let selected_mode = preferred.or_else(|| modes.first());

            if let Some(mode) = selected_mode {
                info!(
                    "Selected mode: {}x{} @ {}Hz",
                    mode.size().0,
                    mode.size().1,
                    mode.vrefresh()
                );
                chosen_connector = Some(conn_handle);
                chosen_mode = Some(*mode);
                break;
            } else {
                warn!("Connector {:?} is connected but has no modes", conn_handle);
            }
        }
    }

    let (chosen_connector, chosen_mode) = match (chosen_connector, chosen_mode) {
        (Some(c), Some(m)) => (c, m),
        _ => {
            error!("No connected display output with a valid mode was found");
            return;
        }
    };

    let conn_info = drm_device.get_connector(chosen_connector, false).unwrap();
    let encoder_handle = match conn_info.current_encoder().or_else(|| conn_info.encoders().first().copied()) {
        Some(e) => e,
        None => {
            error!("No encoder found for chosen connector");
            return;
        }
    };
    let encoder_info = match drm_device.get_encoder(encoder_handle) {
        Ok(e) => e,
        Err(err) => {
            error!("Failed to get encoder info: {}", err);
            return;
        }
    };
    let crtc = match encoder_info.crtc().or_else(|| resource_handles.filter_crtcs(encoder_info.possible_crtcs()).first().copied()) {
        Some(c) => c,
        None => {
            error!("No CRTC available for chosen connector/encoder");
            return;
        }
    };
    info!("Selected CRTC: {:?}", crtc);

    let drm_surface = match drm_device.create_surface(crtc, chosen_mode, &[chosen_connector]) {
        Ok(s) => s,
        Err(err) => {
            error!("Failed to create DRM surface: {}", err);
            return;
        }
    };
    info!("DRM surface created successfully");

    let mut gles_renderer = match unsafe { GlesRenderer::new(egl_context) } {
        Ok(r) => r,
        Err(err) => {
            error!("Failed to create GlesRenderer: {}", err);
            return;
        }
    };

    let allocator = GbmAllocator::new(gbm_allocator.clone(), GbmBufferFlags::RENDERING);
    let supported_codes: Vec<DrmFourcc> = vec![DrmFourcc::Argb8888, DrmFourcc::Xrgb8888];

    let mut gbm_surface = match GbmBufferedSurface::new(
        drm_surface,
        allocator,
        &supported_codes,
        gles_renderer.dmabuf_formats().into_iter().collect::<Vec<_>>(),
    ) {
        Ok(s) => s,
        Err(err) => {
            error!("Failed to create GbmBufferedSurface: {}", err);
            return;
        }
    };
    info!("GBM buffered surface ready — starting render loop");

    loop {
        if !_comp.running {
            break;
        }

        let (mut dmabuf, _age) = match gbm_surface.next_buffer() {
            Ok(ret) => ret,
            Err(err) => {
                error!("Failed to get next GBM buffer: {}", err);
                break;
            }
        };

        let size = chosen_mode.size();
        let render_size: smithay::utils::Size<i32, smithay::utils::Physical> = (size.0 as i32, size.1 as i32).into();
        let damage = Rectangle::new((0, 0).into(), render_size);

        match gles_renderer.bind(&mut dmabuf) {
            Ok(mut fb) => {
                if let Ok(mut frame) = gles_renderer.render(&mut fb, render_size, Transform::Normal) {
                    let _ = frame.clear([0.08, 0.08, 0.08, 1.0].into(), &[damage]);
                    let _ = frame.finish();
                }
            }
            Err(err) => {
                error!("Failed to bind dmabuf to renderer: {}", err);
                break;
            }
        }

        if let Err(err) = gbm_surface.queue_buffer(None, None, ()) {
            error!("Failed to queue GBM buffer for page-flip: {}", err);
            break;
        }
    }

    info!("KydeShell DRM backend stopped");


    let _ = udev_backend;
    let _ = gpu_path;

    warn!("DRM backend: session + udev init done, GPU/render pipeline not yet implemented");
}
