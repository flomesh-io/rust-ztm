/// a test demo for pipy
use libc::{c_char, c_int};

#[link(name = "pipy", kind = "dylib")]
extern "C" {
    pub fn pipy_main(argc: c_int, argv: *const *const c_char) -> c_int;

    pub fn pipy_exit(force: c_int);
}