#![no_std]

use core::slice;
use core::ffi::c_int;

/// Get random bytes; exposed for PQClean implementations.
///
/// # Safety
/// Assumes inputs are valid and may panic over FFI boundary if rng failed.
#[no_mangle]
pub unsafe extern "C" fn PQCRYPTO_RUST_randombytes(buf: *mut u8, len: usize) -> c_int {
    let buf = slice::from_raw_parts_mut(buf, len);
    getrandom::getrandom(buf).expect("RNG Failed");
    0
}
