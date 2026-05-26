use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn zlicenser_hello_world() -> *const c_char {
    c"hello from zlicenser".as_ptr()
}
