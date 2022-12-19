use std::mem::MaybeUninit;
use render_api::{*, v0::*, v0::state::*};
use std::ptr::null_mut;

#[no_mangle]
pub extern "C" fn expr_init() -> InitResult {
    unsafe {
        // TODO: this is undefined behavior, fix it once we actually start implementing logic
        MaybeUninit::uninit().assume_init()
    }
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
pub extern "C" fn expr_user_callback(render_state: *mut RenderState, callback: extern fn(UserEvent, *mut i8)) -> *mut i8 {
    null_mut()
}