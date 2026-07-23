use smithay::{
    backend::winit::{self, WinitEvent},
    backend::renderer::{gles::GlesRenderer, Renderer, Frame},
    reexports::calloop::EventLoop,
    utils::{Rectangle, Transform},
};
use tracing::{info, warn};

use crate::compositor::KydeCompositor;

pub fn run_winit(mut comp: KydeCompositor, mut event_loop: EventLoop<KydeCompositor>) {
    info!("Starting KydeShell with winit backend (windowed mode)...");

    let (mut backend, winit_source) = match winit::init::<GlesRenderer>() {
        Ok(ret) => ret,
        Err(err) => {
            warn!("Failed to initialize winit backend: {}", err);
            return;
        }
    };

    info!("Winit window created — KydeShell running in windowed mode");

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
                _ => {}
            }
        })
        .expect("Failed to insert winit event source");

    info!("Entering KydeShell event loop...");
    loop {
        if !comp.running {
            break;
        }

        let size = backend.window_size();
        let damage = Rectangle::from_loc_and_size((0, 0), (size.w, size.h));

        if let Ok((renderer, mut fb)) = backend.bind() {
            let render_result = renderer.render(&mut fb, size, Transform::Normal);
            if let Ok(mut frame) = render_result {
                let _ = frame.clear([0.08, 0.08, 0.08, 1.0].into(), &[damage]);
                let _ = frame.finish();
            }
        }

        let _ = backend.submit(None);

        if let Err(e) = event_loop.dispatch(std::time::Duration::from_millis(16), &mut comp) {
            warn!("Event loop error: {}", e);
            break;
        }
    }

    info!("KydeShell winit backend stopped");
}
