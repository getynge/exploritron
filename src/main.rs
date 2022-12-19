use std::mem::MaybeUninit;
use render_api::v0::client::push_state;
use std::ptr::{null, null_mut};

fn main() {
    unsafe {
        let _ = render_api::init(MaybeUninit::uninit().assume_init());
        let _ = push_state(null_mut(), null_mut());
    }

    println!("no panic!")
}
