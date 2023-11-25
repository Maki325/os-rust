#![no_std] // don't link the Rust standard library
#![no_main]
// disable all Rust-level entry points
// Added so I can access char to utf-8 array conversion
// Because converting back from &str to &[u8] seems like a hassle
#![feature(char_internals)]

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("asm.s"));

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn kernel_main() {
  // static HELLO: &[u8] = b"Hello OS!";

  // let vga_buffer = 0xb8000 as *mut u8;

  // for (i, &byte) in HELLO.iter().enumerate() {
  //   unsafe {
  //     *vga_buffer.offset(i as isize * 2) = byte;
  //     *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
  //   }
  // }

  // loop {}
}
