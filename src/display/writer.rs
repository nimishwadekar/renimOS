use core::ptr::write_volatile;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use crate::spinlock::SpinLock;
use super::{Colour, fonts::PSF};

//================================================
//  STATICS
//================================================

static mut WRITER: Option<SpinLock<Writer>> = None;

//================================================
//  TYPES
//================================================

pub struct Writer {
    column: usize,
    colour_bg: Colour,
    colour_fg: Colour,
    buffer: Buffer,
    font: PSF<'static>,
}

struct Buffer {
    framebuffer: &'static mut [u8],
    buffer: &'static mut [u8], // backbuffer
    info: FrameBufferInfo,
}

// Byte-level representation of a pixel.
#[derive(Debug, Clone, Copy)]
enum Pixel {
    U32([u8; 4]),
}

//================================================
//  TRAIT IMPLEMENTATIONS
//================================================

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.write_char(ch);
        }
        Ok(())
    }
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl Writer {
    pub fn write_char(&mut self, ch: char) {
        let width = self.font.width;
        let height = self.font.height;

        let buffer_width = self.buffer.info.horizontal_resolution;
        let buffer_height =  self.buffer.info.vertical_resolution;

        match ch {
            '\n' => self.newline(),
            ch => {
                if self.column + width >= buffer_width {
                    self.newline();
                }

                let row = buffer_height - height;
                let col = self.column;

                // Print character (TODO: If slow, check if can be optimized more)

                let bitmap = self.font.bitmap(ch);
                /* let bytes_per_row = (width + 7) / 8;
                let whole_bytes = width / 8;
                let partial_bits = width % 8; */
                
                // Assumes WIDTH = 8 for current implementation.
                for (r, &byte) in bitmap.iter().enumerate() {
                    for c in 0..8 {
                        if (1 << (width - c - 1)) & byte != 0 {
                            self.buffer.draw(row + r, col + c, self.colour_fg);
                        } else {
                            self.buffer.draw(row + r, col + c, self.colour_bg);
                        }
                    }
                }

                self.column += width;
                self.buffer.flush(row, col, self.font.width, self.font.height);
            },
        };
    }

    fn newline(&mut self) {
        let start_offset = self.buffer.offset(self.font.height, 0) * self.buffer.info.bytes_per_pixel;
        let move_count = self.buffer.buffer.len() - start_offset;

        // Move everything up one line.
        unsafe { core::ptr::copy(
            self.buffer.buffer[start_offset..].as_ptr(),
            self.buffer.buffer[0..move_count].as_mut_ptr(),
            move_count
        ); }

        // Clear last line.
        let bg_pixel = self.buffer.get_pixel(self.colour_bg);
        for offset in (move_count..self.buffer.buffer.len()).step_by(bg_pixel.len()) {
            self.buffer.write_pixel(offset, bg_pixel);
        }

        self.column = 0;
        self.buffer.flush_all();
    }
}

impl Buffer {
    /// Flushes a rectangular area from the buffer to screen.
    fn flush(&mut self, row: usize, column: usize, width: usize, height: usize) {
        let mut begin = self.offset(row, column) * self.info.bytes_per_pixel;
        let mut end = begin + width * self.info.bytes_per_pixel;

        for _ in 0..height {
            for (byte, &backbyte) in self.framebuffer[begin..end].iter_mut().zip(self.buffer[begin..end].iter()) {
                unsafe { write_volatile(byte as *mut u8, backbyte); };
            }

            begin += self.info.stride * self.info.bytes_per_pixel;
            end += self.info.stride * self.info.bytes_per_pixel;
        }
    }

    /// Flushes the entire buffer to screen.
    fn flush_all(&mut self) {
        // Volatile
        /* for (byte, &backbyte) in self.framebuffer.iter_mut().zip(self.buffer.iter()) {
            unsafe { write_volatile(byte as *mut u8, backbyte); };
        } */

        // Non-volatile
        self.framebuffer.copy_from_slice(self.buffer);
    }

    fn draw(&mut self, row: usize, column: usize, colour: Colour) {
        let byte_offset = self.offset(row, column) * self.info.bytes_per_pixel;
        self.write_pixel(byte_offset, self.get_pixel(colour));
    }

    #[inline]
    fn write_pixel(&mut self, byte_offset: usize, pixel: Pixel) {
        match pixel {
            Pixel::U32(pixel) => self.buffer[byte_offset..byte_offset + 4].copy_from_slice(&pixel),
        };
    }

    fn get_pixel(&self, colour: Colour) -> Pixel {
        use bootloader::boot_info::PixelFormat;
        match self.info.pixel_format {
            PixelFormat::RGB => Pixel::U32([colour.r, colour.g, colour.b, 0]),
            PixelFormat::BGR => Pixel::U32([colour.b, colour.g, colour.r, 0]),
            PixelFormat::U8 => unimplemented!("PixelFormat::U8"),
            _ => unimplemented!("Unsupported pixel format"),
        }
    }

    /// All units in pixels.
    #[inline]
    fn offset(&self, row: usize, column: usize) -> usize {
        row * self.info.stride + column
    }
}

impl Pixel {
    fn len(&self) -> usize {
        match self {
            Self::U32(..) => 4,
        }
    }
}

//================================================
//  GLOBAL FUNCTIONS
//================================================

fn writer() -> &'static SpinLock<Writer> {
    unsafe { WRITER.as_ref().expect("Framebuffer does not exist") }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut a = writer().lock();
    let mut b = writer().lock();
    a.write_fmt(args).unwrap();
    
}

/// Returns true if the framebuffer was initialized.
pub fn init_display(framebuffer: Option<&FrameBuffer>) -> bool {
    let fb = match framebuffer {
        Some(fb) => fb,
        None => return false,
    };

    let buffer_start = fb.buffer().as_ptr() as usize;
    let buffer_byte_len = fb.buffer().len();
    
    // Replace with dynamically allocated memory when possible.
    static mut BACKBUFFER: [u8; 640 * 480 * 4] = [0; 640 * 480 * 4];

    unsafe { WRITER = Some(SpinLock::new(Writer {
        column: 0,
        colour_fg: Colour::WHITE,
        colour_bg: Colour::BLACK,
        buffer: Buffer {
            framebuffer: core::slice::from_raw_parts_mut(buffer_start as *mut u8, buffer_byte_len),
            buffer: &mut BACKBUFFER,
            info: fb.info()
        },
        font: PSF::parse(include_bytes!("fonts/files/zap-light16.psf")).unwrap(),
    })); }

    true
}