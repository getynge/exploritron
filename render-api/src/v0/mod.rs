//! v0 of the render API (subject to change without module name changing)
//!
//! In this document "client" refers to the game engine proper
//! and "server" refers to the rendering engine the client loads.
//!
//! This version provides two submodules: client and state.
//! The client submodule defines what functions a client should expect to be callable.
//! The state submodule is the type used to communicate state from the client to the server.

#[cfg(feature = "client")]
pub mod client;
pub mod state;

/// RenderEvent indicates what event, if any, happened when rendering a frame
///
/// # Variants
/// * FrameSkipped - the frame was skipped, not necessarily a failing error
/// * RenderError - the frame should have been rendered but was not
/// * DisplayError - the frame was rendered but could not be displayed
/// * DeviceError - an error occured in the driver doing the actual rendering
#[repr(C)]
pub enum RenderEvent {
    FrameSkipped,
    RenderError,
    DisplayError,
    DeviceError
}

/// RenderResult indicates event details for a given frame
///
/// # Fields
/// * event is the type of event that occured
/// * critical is whether or not the event indicates an unrecoverable error in the renderer
/// * message is the event message, if any
#[repr(C)]
pub struct RenderResult {
    event: RenderEvent,
    critical: bool,
    message: *mut i8
}

/// UserEvent indicates what kind of user input was received
///
/// # Variants
/// * Input - literal button input, keyboard, joystick, controller button, etc
/// * Command - text command entered in a field of some kind
#[repr(C)]
pub enum UserEvent {
    Input,
    Command
}