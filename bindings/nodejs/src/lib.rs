use napi_derive::napi;

#[napi]
pub fn hello_world() -> String {
    zlicenser::hello_world().to_string()
}
