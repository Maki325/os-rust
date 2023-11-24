#![no_std] // don't link the Rust standard library
#![no_main]
// disable all Rust-level entry points
// Added so I can access char to utf-8 array conversion
// Because converting back from &str to &[u8] seems like a hassle
#![feature(char_internals)]

mod vga;
use core::panic::PanicInfo;

use vga::{Color, VGAWriter};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let mut writer = VGAWriter::new();

  writer.write_string("Testing!\n");
  writer.set_color(vga::ColorCode::new(Color::Red, Color::Black));
  writer.write_string("Hellou!");

  loop {}
}
