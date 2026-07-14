use smithay::reexports::wayland_server::Resource;
use smithay::{
    delegate_compositor, delegate_shm, delegate_xdg_shell,
    delegate_data_device,
    input::{SeatState, SeatHandler},
    reexports::wayland_server::{
        DisplayHandle,
        protocol::wl_seat::WlSeat,
    },
    wayland::{
        compositor::{CompositorState, CompositorHandler, CompositorClientState},
        shell::xdg::{XdgShellState, XdgShellHandler, ToplevelSurface, PopupSurface, PositionerState},
        shm::{ShmState, ShmHandler},
        selection::{data_device::{DataDeviceState, DataDeviceHandler, ClientDndGrabHandler, ServerDndGrabHandler}, SelectionHandler},
    },
};
use tracing::info;

#[derive(Default)]
pub struct KydeClientState {
    pub compositor_state: CompositorClientState,
}
impl smithay::reexports::wayland_server::backend::ClientData for KydeClientState {}

pub struct KydeCompositor {
    pub running: bool,
    pub display_handle: DisplayHandle,
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
}

impl KydeCompositor {
    pub fn new(display_handle: DisplayHandle) -> Self {
        info!("Creating KydeShell compositor...");

        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let shm_state = ShmState::new::<Self>(&display_handle, vec![]);
        let seat_state = SeatState::new();
        let data_device_state = DataDeviceState::new::<Self>(&display_handle);

        KydeCompositor {
            running: true,
            display_handle,
            compositor_state,
            xdg_shell_state,
            shm_state,
            seat_state,
            data_device_state,
        }
    }

    pub fn stop(&mut self) {
        info!("Stopping KydeShell compositor...");
        self.running = false;
    }
}

impl CompositorHandler for KydeCompositor {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a smithay::reexports::wayland_server::Client) -> &'a CompositorClientState {
        &client.get_data::<KydeClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, _surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface) {
        info!("Surface committed");
    }
}

impl XdgShellHandler for KydeCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        info!("New window opened: {:?}", surface.wl_surface().id());
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        info!("New popup opened");
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: smithay::utils::Serial) {}

    fn reposition_request(&mut self, _surface: PopupSurface, _positioner: PositionerState, _token: u32) {}
}

impl ShmHandler for KydeCompositor {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for KydeCompositor {
    type KeyboardFocus = smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
    type PointerFocus = smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
    type TouchFocus = smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

impl DataDeviceHandler for KydeCompositor {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}
impl SelectionHandler for KydeCompositor {
    type SelectionUserData = ();
}
impl ClientDndGrabHandler for KydeCompositor {}
impl ServerDndGrabHandler for KydeCompositor {}

delegate_compositor!(KydeCompositor);
delegate_xdg_shell!(KydeCompositor);
delegate_shm!(KydeCompositor);
delegate_data_device!(KydeCompositor);

impl smithay::wayland::buffer::BufferHandler for KydeCompositor {
    fn buffer_destroyed(&mut self, _buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer) {}
}
