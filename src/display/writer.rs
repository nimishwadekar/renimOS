use core::ptr::write_volatile;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use crate::{spinlock::SpinLock, init_guard, prelude::serial_panic};
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

impl core::ops::Deref for Pixel {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Pixel::U32(bytes) => bytes,
        }
    }
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl Writer {
    pub fn colour_bg(&self) -> Colour { self.colour_bg }
    pub fn colour_fg(&self) -> Colour { self.colour_fg }
    pub fn set_colour_bg(&mut self, colour: Colour) { self.colour_bg = colour; }
    pub fn set_colour_fg(&mut self, colour: Colour) { self.colour_fg = colour; }

    pub fn clear_screen(&mut self) {
        let pixel_bg = self.buffer.get_pixel(self.colour_bg);
        for offset in (0..self.buffer.buffer.len()).step_by(pixel_bg.len()) {
            self.buffer.write_pixel(offset, pixel_bg);
        }
    }

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
                let bytes_per_row = (width + 7) >> 3; // divide by 8
                let partial_bits = width & 7; // modulo 8

                for (r, row_chunk) in bitmap.chunks(bytes_per_row).enumerate() {
                    let (row_chunk, rem_byte) = if partial_bits == 0 {
                        (row_chunk, None)
                    } else {
                        (&row_chunk[..bytes_per_row - 1], Some(row_chunk[bytes_per_row - 1]))
                    };

                    let row = row + r;
                    for (byte_id, byte) in row_chunk.iter().enumerate() {
                        let col = col + (byte_id << 3); // multiply by 8
                        for c in 0..8 {
                            if (0b1000_0000 >> c) & byte != 0 {
                                self.buffer.draw(row, col + c, self.colour_fg);
                            } else {
                                self.buffer.draw(row, col + c, self.colour_bg);
                            }
                        }
                    }

                    if let Some(rem_byte) = rem_byte {
                        let col = col + (row_chunk.len() << 3);
                        for c in 0..partial_bits {
                            if (0b1000_0000 >> c) & rem_byte != 0 {
                                self.buffer.draw(row, col + c, self.colour_fg);
                            } else {
                                self.buffer.draw(row, col + c, self.colour_bg);
                            }
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
        self.buffer[byte_offset..byte_offset + pixel.len()].copy_from_slice(&pixel);
        /* match pixel {
            Pixel::U32(pixel) => self.buffer[byte_offset..byte_offset + 4].copy_from_slice(&pixel),
        }; */
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

//================================================
//  GLOBAL FUNCTIONS
//================================================

#[inline(always)]
fn writer() -> &'static SpinLock<Writer> {
    unsafe { WRITER.as_ref().unwrap_or_else(|| serial_panic("display not initialised")) }
}

pub fn clear_screen() {
    writer().lock().clear_screen();
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::display::_kprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprint_with_colour {
    ($colour:expr, $($arg:tt)*) => ($crate::display::_kprint_with_colour($colour, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln_with_colour {
    ($colour:expr, $($arg:tt)*) => ($crate::kprint_with_colour!($colour, "{}\n", format_args!($($arg)*)));
}

pub fn init(fb: &FrameBuffer) {
    init_guard!();

    let buffer_start = fb.buffer().as_ptr() as usize;
    let buffer_byte_len = fb.buffer().len();
    
    // Replace with dynamically allocated memory when possible.
    static mut BACKBUFFER: [u8; 800 * 600 * 4] = [0; 800 * 600 * 4];
    
    unsafe { WRITER = Some(SpinLock::new(Writer {
        column: 0,
        colour_fg: Colour::new(0xd4, 0xd4, 0xd4),
        colour_bg: Colour::new(0x1e, 0x1e, 0x1e),
        buffer: Buffer {
            framebuffer: core::slice::from_raw_parts_mut(buffer_start as *mut u8, buffer_byte_len),
            buffer: &mut BACKBUFFER[..buffer_byte_len],
            info: fb.info()
        },
        font: PSF::parse(include_bytes!("fonts/files/zap-light16.psf")).unwrap_or_else(|| serial_panic("PSF::parse() failed")),
    })); }
}

//================================================
//  PRIVATE GLOBAL FUNCTIONS
//================================================

#[doc(hidden)]
pub fn _kprint(args: core::fmt::Arguments) {
    use core::fmt::Write;
    writer().lock().write_fmt(args).unwrap_or_else(|_| serial_panic("writer.write_fmt() failed"));
}

#[doc(hidden)]
pub fn _kprint_with_colour(colour: Colour, args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = writer().lock();
    let fg = writer.colour_fg();
    writer.set_colour_fg(colour);
    writer.write_fmt(args).unwrap_or_else(|_| serial_panic("writer.write_fmt() failed"));
    writer.set_colour_fg(fg);
}

//================================================
//  UNIT TESTS
//================================================

#[cfg(test)]
mod tests {
    #[test_case]
    fn kprintln_simple() {
        kprintln!("kprintln_simple output");
    }

    #[test_case]
    fn kprintln_many() {
        for _ in 0..30 {
            kprintln!("kprintln_many output");
        }
    }

    #[test_case]
    fn kprintln_output() {
        let s = "Some test string that fits on a single line";
        kprintln!("\n{}", s);

        let writer = super::writer().lock();
        let buf = &writer.buffer.buffer;
        
        let pixel_fg = writer.buffer.get_pixel(writer.colour_fg);
        let pixel_bg = writer.buffer.get_pixel(writer.colour_bg);
        let pixel_len = pixel_fg.len();

        let width = writer.font.width;

        let row = writer.buffer.info.vertical_resolution - 2 * writer.font.height;
        let mut col = 0;

        for c in s.chars() {
            let bitmap = writer.font.bitmap(c);

            let bytes_per_row = (width + 7) >> 3; // divide by 8
            let partial_bits = width & 7; // modulo 8

            use core::ops::Deref;
            for (r, row_chunk) in bitmap.chunks(bytes_per_row).enumerate() {
                let (row_chunk, rem_byte) = if partial_bits == 0 {
                    (row_chunk, None)
                } else {
                    (&row_chunk[..bytes_per_row - 1], Some(row_chunk[bytes_per_row - 1]))
                };

                let row = row + r;
                for (byte_id, byte) in row_chunk.iter().enumerate() {
                    let col = col + (byte_id << 3); // multiply by 8
                    for c in 0..8 {
                        let offset = writer.buffer.offset(row, col + c) * writer.buffer.info.bytes_per_pixel;
                        if (0b1000_0000 >> c) & byte != 0 {
                            assert_eq!(&buf[offset..offset + pixel_len], pixel_fg.deref());
                        } else {
                            assert_eq!(&buf[offset..offset + pixel_len], pixel_bg.deref());
                        }
                    }
                }

                if let Some(rem_byte) = rem_byte {
                    let col = col + (row_chunk.len() << 3);
                    for c in 0..partial_bits {
                        let offset = writer.buffer.offset(row, col + c) * writer.buffer.info.bytes_per_pixel;
                        if (0b1000_0000 >> c) & rem_byte != 0 {
                            assert_eq!(&buf[offset..offset + pixel_len], pixel_fg.deref());
                        } else {
                            assert_eq!(&buf[offset..offset + pixel_len], pixel_bg.deref());
                        }
                    }
                }
            }

            col += width;
        }
    }

    #[test_case]
    fn clear_screen() {
        super::clear_screen();

        let writer = super::writer().lock();
        let pixel_bg = writer.buffer.get_pixel(writer.colour_bg);

        use core::ops::Deref;
        for buffer_pixel in writer.buffer.buffer.chunks(pixel_bg.len()) {
            assert_eq!(buffer_pixel, pixel_bg.deref());
        }
    }
}