static MAX_WITDH: usize = 80;
static MAX_HEIGHT: usize = 25;

#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
  pub fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[repr(C)]
struct VGAChar {
  char: u8,
  color: ColorCode,
}

pub struct VGAWriter {
  row: usize,
  column: usize,
  color: ColorCode,
}

impl VGAWriter {
  pub fn new() -> VGAWriter {
    return VGAWriter {
      row: 0,
      column: 0,
      color: ColorCode::new(Color::White, Color::Black),
    };
  }

  pub fn write_ascii(&mut self, ch: u8) {
    let index = self.row * MAX_WITDH + self.column;
    const VGA_BUFFER: *mut VGAChar = 0xb8000 as *mut VGAChar;

    if ch == '\n' as u8 {
      self.column = 0;
      self.row += 1;

      if self.row >= MAX_HEIGHT {
        self.row = 0;
      }

      return;
    }

    unsafe {
      *VGA_BUFFER.offset(index as isize) = VGAChar {
        char: ch,
        color: self.color,
      };

      // *VGA_BUFFER.offset(index as isize * 2) = ch as u8;
      // *VGA_BUFFER.offset(index as isize * 2 + 1) = color;
    }

    self.column += 1;
    if self.column >= MAX_WITDH {
      self.column = 0;
      self.row += 1;

      if self.row >= MAX_HEIGHT {
        self.row = 0;
      }
    }
  }

  pub fn write_char(&mut self, ch: char) {
    let mut dst = [0; 4];
    let dst = core::char::encode_utf8_raw(ch as u32, &mut dst);

    for ch in dst {
      self.write_ascii(*ch);
    }
  }

  pub fn write_string(&mut self, data: &str) {
    for ch in data.chars() {
      self.write_char(ch);
    }
  }

  pub fn set_color(&mut self, color: ColorCode) {
    self.color = color;
  }
}
