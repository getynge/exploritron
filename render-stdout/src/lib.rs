use render_api::{*, state::*};
use std::ptr::null_mut;

#[no_mangle]
pub extern "C" fn expr_init() -> *mut RenderState {
    null_mut()
}

#[no_mangle]
pub extern "C" fn expr_disconnect(render_state: *mut RenderState) -> *mut i8 {
    null_mut()
}

#[no_mangle]
pub extern "C" fn expr_reconnect(render_state: *mut RenderState) -> *mut i8 {
    null_mut()
}

#[no_mangle]
pub extern "C" fn expr_push_state(render_state: *mut RenderState, game_state: *mut State) -> *mut i8 {
    null_mut()
}

#[no_mangle]
pub extern "C" fn expr_frame_callback(render_state: *mut RenderState, callback: extern fn(RenderResult, i32, *mut State, *mut i8)) -> *mut i8 {
    null_mut()
}

#[no_mangle]
pub extern "C" fn expr_user_callback(render_state: *mut RenderState, callback: extern fn(EventKind, *mut i8)) -> *mut i8 {
    null_mut()
}

#[no_mangle]
pub extern "C" fn expr_register_attribute_resolver(render_state: *mut RenderState, resolver: extern fn(*mut AttributeSet, *mut i16) -> Attribute) -> *mut i8 {
    null_mut()
}