pub mod client;
pub mod state;

#[repr(C)]
pub enum RenderResult {
    FrameSkipped,
    RenderError,
    DisplayError,
    DeviceError
}

#[repr(C)]
pub enum EventKind {
    Input,
    Command
}