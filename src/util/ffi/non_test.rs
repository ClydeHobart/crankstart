use {
    crate::{CrankstartAPI, breakpoint_nop, eprintln, util::singleton::Singleton},
    core::{
        alloc::{GlobalAlloc, Layout},
        intrinsics::abort,
        panic::PanicInfo,
        ptr,
    },
};

fn abort_with_addr(addr: usize) -> ! {
    let p = addr as *mut i32;

    unsafe {
        *p = 0;
    }

    abort()
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        eprintln!(
            "panic: {} @ {}:{}",
            panic_info.message(),
            location.file(),
            location.line(),
        );
    } else {
        eprintln!("panic");
    }

    breakpoint_nop!();

    abort_with_addr(0xdeadbeef);
}

pub(crate) struct CrankstartAllocator;

unsafe impl Sync for CrankstartAllocator {}

unsafe impl GlobalAlloc for CrankstartAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        CrankstartAPI::get()
            .system
            .realloc(ptr::null_mut(), layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        CrankstartAPI::get().system.realloc(ptr, 0);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        CrankstartAPI::get().system.realloc(ptr, new_size)
    }
}

#[global_allocator]
pub(crate) static mut A: CrankstartAllocator = CrankstartAllocator;

// define what happens in an Out Of Memory (OOM) condition

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    eprintln!("Out of Memory");
    abort_with_addr(0xDEADFA11);
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {
    unimplemented!();
}
