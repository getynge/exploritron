use std::ffi::CString;
use crate::state::*;
use crate::*;

#[link(name = "render", kind = "dylib")]
extern {
    /// expr_init initializes the renderer and then returns its persistent state
    pub fn expr_init() -> *mut RenderState;

    /// expr_disconnect notifies the renderer that it is about to be stopped
    ///
    /// The return value is an error message if an error occurs, null otherwise
    pub fn expr_disconnect(render_state: *mut RenderState) -> *mut i8;

    /// expr_reconnect notifies the renderer
    ///
    /// The return value is an error message if an error occurs,
    /// a negative pointer if expr_init needs to be rerun,
    /// or null
    pub fn expr_reconnect(render_state: *mut RenderState) -> *mut i8;

    /// expr_push_state adds game_state to the list of state changes to be processed by the renderer
    ///
    /// # Arguments
    /// * render_state is an opaque type that the renderer uses to persist state. (engine managed)
    /// * game_state is a type that represents information the renderer may need for this frame. (renderer managed)
    ///
    /// The return value is an error message if an error occurs, null otherwise
    pub fn expr_push_state(render_state: *mut RenderState, game_state: *mut State) -> *mut i8;

    /// expr_frame_callback calls the provided callback after an event occurs while rendering a frame
    ///
    /// # Arguments
    /// * render_state is an opaque type that the renderer uses to persist state. (engine managed)
    /// * callback is the callback that is called when an event occurs. (unmanaged)
    ///
    /// # Callback Arguments
    /// * The RenderResult indicates which kind of failure has occured.
    /// * The i32 indicates which frame the event occurred on.
    /// * The State pointer indicates what state was being processed when the event happened.
    /// * The CString is any message accompanying the event
    ///
    /// The return value is an error message if an error occurs, null otherwise
    pub fn expr_frame_callback(render_state: *mut RenderState, callback: extern fn(RenderResult, i32, *mut State, CString)) -> *mut i8;

    /// expr_user_callback calls the provided callback upon a user-triggered event
    ///
    /// # Arguments
    /// * render_state is an opaque type that the renderer uses to persist state. (engine managed)
    /// * callback is the callback that is called when an event occurs. (unmanaged)
    ///
    /// # Callback Arguments
    /// * EventKind indicates whether the user sent a keystroke or command
    /// * If EventKind is an input, CString is the button pressed.
    ///   If EventKind is a command, CString is the command text.
    ///
    /// The return value is an error message if an error occurs, null otherwise
    pub fn expr_user_callback(render_state: *mut RenderState, callback: extern fn(EventKind, CString)) -> *mut i8;

    /// expr_register_attribute_resolver registers a function for the renderer to call when it needs to read an attribute
    ///
    /// # Arguments
    /// * render_state is an opaque type that the renderer uses to persist state. (engine managed)
    /// * resolver is the function called to resolve attributes (unmanaged)
    ///
    /// # Resolver Arguments
    /// * AttributeSet is the AttributeSet that needs to be read (renderer managed)
    /// * *mut i8 is the key for the attribute set (renderer managed)
    ///
    /// The return value is an error message if an error occurs, null otherwise
    pub fn expr_register_attribute_resolver(render_state: *mut RenderState, resolver: extern fn(*mut AttributeSet, *mut i8) -> Attribute) -> *mut i8;
}