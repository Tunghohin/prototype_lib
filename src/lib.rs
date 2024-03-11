#![no_std]
#![feature(linkage)]

pub mod console;
mod lang_items;
pub mod prelude;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    // clear bss segment
    unsafe {
        extern "C" {
            fn start_bss();
            fn end_bss();
        }
        core::slice::from_raw_parts_mut(
            start_bss as *mut u8,
            end_bss as usize - start_bss as usize,
        )
        .fill(0);
    }

    exit(main());
    unreachable!();
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Can't find main!");
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    syscall::sys_write(fd, buffer)
}

pub fn exit(exit_code: i32) -> isize {
    syscall::sys_exit(exit_code)
}

pub fn r#yield() -> isize {
    syscall::sys_yield()
}
pub fn get_time() -> isize {
    syscall::sys_get_time()
}
