use render_api::v0::client;
use std::ptr::null_mut;

fn main() {
    unsafe {
        if client::expr_push_state(null_mut(), null_mut()).is_null() {
            println!("it works!")
        } else {
            println!("it doesn't work :C")
        }
    }
}
