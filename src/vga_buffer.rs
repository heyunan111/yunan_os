#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
    EnBlack = 0,
    EnBlue = 1,
    EnGreen = 2,
    EnCyan = 3,
    EnRed = 4,
    EnMagenta = 5,
    EnBrown = 6,
    EnLightGray = 7,
    EnDarkGray = 8,
    EnLightBlue = 9,
    EnLightGreen = 10,
    EnLightCyan = 11,
    EnLightRed = 12,
    EnPink = 13,
    EnYellow = 14,
    EnWhite = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    color_code: ColorCode,
    //这个借用应该在整个程序的运行期间有效
    buffer: &'static mut Buffer,
}

impl Writer {
    fn new_line(&mut self) {}

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                _ => self.write_byte(0xfe),
            }
        }
    }
}

pub fn print_sth() {
    let mut writer = Writer {
        column_pos: 0,
        color_code: ColorCode::new(Color::EnBlue, Color::EnCyan),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'h');
    writer.write_str("ello word!");
    writer.write_str("你好"); //unicode 不可打印
}
