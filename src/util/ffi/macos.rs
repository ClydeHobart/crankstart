#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe { *dest.add(i) = *src.add(i); }
        i += 1;
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            unsafe { *dest.add(i) = *src.add(i); }
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            unsafe { *dest.add(i) = *src.add(i); }
            i += 1;
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe { memcmp(s1, s2, n) }
}

pub unsafe fn memset_internal(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c: u8 = c as u8;
    let mut i = 0;
    while i < n {
        unsafe { *s.add(i) = c;  }
        i += 1;
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe { memset_internal(s, c, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bzero(s: *mut u8, n: usize) {
    unsafe { memset_internal(s, 0, n); }
}

#[unsafe(no_mangle)]
extern "C" fn _Unwind_Resume() {
    unimplemented!();
}

#[link(name = "System")]
unsafe extern "C" {}
