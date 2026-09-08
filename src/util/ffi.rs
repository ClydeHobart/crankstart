#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(not(any(test, doctest)))]
pub mod non_test;

#[cfg(not(target_os = "windows"))]
pub mod non_windows;

#[unsafe(no_mangle)]
pub extern "C" fn _sbrk() {}

#[unsafe(no_mangle)]
pub extern "C" fn _fstat() {}

#[unsafe(no_mangle)]
pub extern "C" fn _isatty() {}

#[unsafe(no_mangle)]
pub extern "C" fn _open() {}

#[unsafe(no_mangle)]
pub extern "C" fn _kill() {}

#[unsafe(no_mangle)]
pub extern "C" fn _getpid() {}

#[unsafe(no_mangle)]
extern "C" fn __exidx_start() {
    unimplemented!();
}

#[unsafe(no_mangle)]
extern "C" fn __exidx_end() {
    unimplemented!();
}
