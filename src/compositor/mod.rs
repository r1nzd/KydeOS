use smithay::reexports::wayland_server::Resource;
use smithay::{
    delegate_compositor, delegate_shm, delegate_xdg_shell,
    delegate_data_device,
    input::{Seat, SeatState, SeatHandler, pointer::CursorImageStatus},
    desktop::{Space, Window},
    wayland::seat::WaylandFocus,
    wayland::shell::xdg::decoration::{XdgDecorationState, XdgDecorationHandler},
    reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode as DecorationMode,
    utils::{Point, Logical},
    wayland::output::OutputHandler,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitlebarButton {
    Minimize,
    Maximize,
    Close,
}

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
    pub seat: Seat<Self>,
    pub space: Space<Window>,
    pub pointer_location: Point<f64, Logical>,
    pub start_time: std::time::Instant,
    pub moving_window: Option<(Window, Point<f64, Logical>, Point<i32, Logical>)>,
    pub resizing_window: Option<(Window, Point<f64, Logical>, (i32, i32))>,
    pub taskbar_chips: Vec<(smithay::utils::Rectangle<i32, Logical>, Window)>,
    pub xdg_decoration_state: XdgDecorationState,
    pub titlebar_buttons: Vec<(smithay::utils::Rectangle<i32, Logical>, Window, TitlebarButton)>,
    pub minimized_windows: Vec<(Window, Point<i32, Logical>)>,
    pub maximized_windows: Vec<(Window, Point<i32, Logical>, (i32, i32))>,
    pub launcher_open: bool,
    pub launcher_button_rect: smithay::utils::Rectangle<i32, smithay::utils::Physical>,
    pub launcher_app_rects: Vec<(smithay::utils::Rectangle<i32, smithay::utils::Physical>, String)>,
    pub control_center_open: bool,
    pub control_center_button_rect: smithay::utils::Rectangle<i32, smithay::utils::Physical>,
    pub control_center_action_rects: Vec<(smithay::utils::Rectangle<i32, smithay::utils::Physical>, String)>,
}

impl KydeCompositor {
    pub fn new(display_handle: DisplayHandle) -> Self {
        info!("Creating KydeShell compositor...");

        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let shm_state = ShmState::new::<Self>(&display_handle, vec![]);
        let mut seat_state = SeatState::new();
        let data_device_state = DataDeviceState::new::<Self>(&display_handle);

        let xdg_decoration_state = XdgDecorationState::new::<Self>(&display_handle);

        let mut seat: Seat<Self> = seat_state.new_wl_seat(&display_handle, "seat0");
        seat.add_keyboard(Default::default(), 200, 25).expect("Failed to add keyboard");
        seat.add_pointer();

        KydeCompositor {
            running: true,
            display_handle,
            compositor_state,
            xdg_shell_state,
            shm_state,
            seat_state,
            data_device_state,
            seat,
            space: Space::default(),
            pointer_location: (0.0, 0.0).into(),
            start_time: std::time::Instant::now(),
            moving_window: None,
            resizing_window: None,
            taskbar_chips: Vec::new(),
            xdg_decoration_state,
            titlebar_buttons: Vec::new(),
            minimized_windows: Vec::new(),
            maximized_windows: Vec::new(),
            launcher_open: false,
            launcher_button_rect: smithay::utils::Rectangle::from_size((0, 0).into()),
            launcher_app_rects: Vec::new(),
            control_center_open: false,
            control_center_button_rect: smithay::utils::Rectangle::from_size((0, 0).into()),
            control_center_action_rects: Vec::new(),
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

    fn commit(&mut self, surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface) {
        info!("Surface committed");
        smithay::backend::renderer::utils::on_commit_buffer_handler::<Self>(surface);
        self.space.elements().find(|w| w.wl_surface().map(|s| &*s == surface).unwrap_or(false)).cloned().map(|w| w.on_commit());
    }
}

impl XdgShellHandler for KydeCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        info!("New window opened: {:?}", surface.wl_surface().id());
        surface.send_configure();
        let window = Window::new_wayland_window(surface);
        self.space.map_element(window, (100, 100), true);
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

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {}
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

impl XdgDecorationHandler for KydeCompositor {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(DecorationMode::ServerSide);
        });
        toplevel.send_configure();
    }

    fn request_mode(&mut self, toplevel: ToplevelSurface, _mode: DecorationMode) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(DecorationMode::ServerSide);
        });
        toplevel.send_configure();
    }

    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(DecorationMode::ServerSide);
        });
        toplevel.send_configure();
    }
}

delegate_compositor!(KydeCompositor);
delegate_xdg_shell!(KydeCompositor);
delegate_shm!(KydeCompositor);
delegate_data_device!(KydeCompositor);
smithay::delegate_seat!(KydeCompositor);
impl OutputHandler for KydeCompositor {}
smithay::delegate_output!(KydeCompositor);
smithay::delegate_xdg_decoration!(KydeCompositor);

impl smithay::wayland::buffer::BufferHandler for KydeCompositor {
    fn buffer_destroyed(&mut self, _buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer) {}
}
