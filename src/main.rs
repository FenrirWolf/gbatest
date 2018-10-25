#![feature(start)]
#![no_std]

use core::ptr::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
  unsafe {
    write_volatile(0x04000000 as *mut u16, 0x0403);
    let p = 0x06000000 as *mut u16;
    write_volatile(p.offset(120 + 80 * 240), 0x001F);
    write_volatile(p.offset(136 + 80 * 240), 0x03E0);
    write_volatile(p.offset(120 + 96 * 240), 0x7C00);
  }
  loop {}
}
