#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use wasm_bindgen::prelude::*;
pub fn hello_world() -> String {
    "Hello World".to_string()
}
#[allow(non_snake_case)]
#[export_name = "hello_world"]
#[allow(clippy::all)]
pub extern "C" fn __wasm_bindgen_generated_hello_world(
) -> <String as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret = { hello_world() };
    <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe_hello_world() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <String as WasmDescribe>::describe();
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_16eac2eb53d04aca: [u8; 121usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b".\x00\x00\x00{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58\"}C\x00\x00\x00\x01\x00\x00\x00\x00\x0bhello_world\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00&wasm-module-with-rust-583f32b0369ea94e\x00"
};
