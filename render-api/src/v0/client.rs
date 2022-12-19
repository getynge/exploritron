#![allow(non_upper_case_globals)]

use std::ptr::null_mut;
use lazy_static::lazy_static;
use libloading::Symbol;
use crate::RenderState;
use super::state::*;
use super::*;
use crate::library;

/// disconnect notifies the server that it is about to be stopped
///
/// # Link Details
/// * The target server must expose the symbol `expr_disconnect` for this function to resolve
/// * This function may be left unimplemented if the server doesn't need it
///
/// # Notes
/// The return value is an error message if an error occurs, null otherwise
#[inline]
pub unsafe fn disconnect(render_state: *mut RenderState) -> *mut i8 {
    lazy_static! {
        static ref disconnect: Option<Symbol<'static, unsafe extern fn(*mut RenderState) -> *mut i8>>
            = unsafe { library.get(b"expr_disconnect").ok() };
    }
    disconnect.as_ref().map(|f| f(render_state)).unwrap_or(null_mut())
}

/// reconnect notifies the server when the client is reconnecting
///
/// # Link Details
/// * The target server must expose the symbol `expr_reconnect` for this function to resolve
/// * This function may be left unimplemented if the server doesn't need it
///
/// # Notes
/// The return value is an error message if an error occurs,
/// a negative pointer if expr_init needs to be rerun,
/// or null
#[inline]
pub unsafe fn reconnect(render_state: *mut RenderState) -> *mut i8 {
    lazy_static! {
        static ref reconnect: Option<Symbol<'static, unsafe extern fn(*mut RenderState) -> *mut i8>>
            = unsafe { library.get(b"expr_reconnect").ok() };
    }
    reconnect.as_ref().map(|f| f(render_state)).unwrap_or(null_mut())
}

/// push_state adds game_state to the list of state changes to be processed by the server
///
/// # Arguments
/// * render_state is an opaque type that the server uses to persist state. (engine managed)
/// * game_state is a type that represents information the server may need for this frame. (server managed)
///
/// # Link Details
/// * The target library must expose the symbol `expr_push_state` for this function to resolve
/// * This function is required to be defined
///
/// # Notes
/// The return value is an error message if an error occurs, null otherwise
pub unsafe fn push_state(render_state: *mut RenderState, game_state: *mut State) -> *mut i8 {
    lazy_static! {
        static ref push_state: Symbol<'static, unsafe extern fn(*mut RenderState, *mut State) -> *mut i8>
            = unsafe { library.get(b"expr_push_state").expect("failed to load expr_push_state") };
    }
    push_state(render_state, game_state)
}

/// frame_callback calls the provided callback after an event occurs while rendering a frame
///
/// # Arguments
/// * render_state is an opaque type that the server uses to persist state. (engine managed)
/// * callback is the callback that is called when an event occurs. (unmanaged)
///
/// # Callback Arguments
/// * The RenderEvent indicates which kind of failure has occured.
/// * The i32 indicates which frame the event occurred on.
/// * The State pointer indicates what state was being processed when the event happened.
/// * The *mut i8 is any message accompanying the event
///
/// # Link Details
/// * The target library must expose the symbol `expr_frame_callback` for this function to resolve
/// * This function is required to be defined
///
/// # Notes
/// The return value is an error message if an error occurs, null otherwise
pub unsafe fn frame_callback(render_state: *mut RenderState, callback: unsafe extern fn(RenderResult, i32, *mut State, *mut i8)) -> *mut i8 {
    lazy_static! {
        static ref frame_callback:
            Symbol<'static, unsafe extern fn(*mut RenderState,
                unsafe extern fn(RenderResult, i32, *mut State, *mut i8)) -> *mut i8>
            = unsafe { library.get(b"expr_frame_callback").expect("failed to load expr_frame_callback") };
    }
    frame_callback(render_state, callback)
}

/// expr_user_callback calls the provided callback upon a user-triggered event
///
/// # Arguments
/// * render_state is an opaque type that the server uses to persist state. (engine managed)
/// * callback is the callback that is called when an event occurs. (unmanaged)
///
/// # Callback Arguments
/// * UserEvent indicates whether the user sent a keystroke or command
/// * If UserEvent is an input, *mut i8 is the button pressed.
///   If UserEvent is a command, *mut i8 is the command text.
///
/// # Link Details
/// * The target library must expose the symbol `expr_user_callback` for this function to resolve
/// * This function is required to be defined
///
/// # Notes
/// The return value is an error message if an error occurs, null otherwise
pub unsafe fn user_callback(render_state: *mut RenderState, callback: unsafe extern fn(UserEvent, *mut i8)) -> *mut i8 {
    lazy_static! {
        static ref user_callback:
            Symbol<'static, unsafe extern fn(*mut RenderState,
                unsafe extern fn(UserEvent, *mut i8)) -> *mut i8>
            = unsafe { library.get(b"expr_user_callback").expect("failed to load expr_user_callback") };
    }
    user_callback(render_state, callback)
}
