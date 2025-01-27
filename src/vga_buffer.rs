//0xb8000
//在 x86 架构 的 BIOS 或操作系统中通常被用作 视频内存（VGA缓冲区）

// 0xB8000 是 VGA 显示缓冲区的起始地址，在大多数 x86 架构计算机上，0xB8000 是映射到物理内存的一个特殊区域。这个区域用于存储显示的文本内容。
// 每个字符占用两个字节：
// 第一个字节存储字符的 ASCII 值（或者Unicode字符的低字节）。
// 第二个字节用于存储该字符的属性（如颜色）。

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

use core::fmt;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    //字符颜色
    character_color_code: ColorCode,
    //这个借用应该在整个程序的运行期间有效
    buffer: &'static mut Buffer,
}

impl Writer {
    fn clear_row(&mut self, row: usize) {
        let black = ScreenChar {
            ascii_character: b' ',
            color_code: self.character_color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(black);
        }
    }

    fn cheng_character_color(&mut self, color_code: ColorCode) {
        self.character_color_code = color_code;
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for clo in 0..BUFFER_WIDTH {
                let val = self.buffer.chars[row][clo].read();
                self.buffer.chars[row - 1][clo].write(val);
            }
        }
        self.column_pos = 0;
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;
                let color_code = self.character_color_code;

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

use core::fmt::Write;
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

///TODO：双缓冲区，无锁并发，高效内存复制
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_pos: 0,
        character_color_code: ColorCode::new(Color::EnWhite, Color::EnBlack),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {$crate::vga_buffer::_print(format_args!($($arg)*))};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };

    ($($arg:tt)*) => {
        ($crate::print!("{}\n",format_args!($($arg)*)) );
    };
}

#[test_case]
fn test_println_sth() {
    print!("some ...");
    for i in 0..1000 {
        println!("i:{}", i);
    }
}

#[test_case]
fn test_println_out_put() {
    let s = "test println out put";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
