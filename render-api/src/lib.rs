pub mod state;
#[cfg(feature = "client")]
pub mod client;

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