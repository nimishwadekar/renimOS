
//================================================
//  CONSTANTS
//================================================

const MAGIC: u16 = 0x0436;

const MODE_512: u8 = 0x01;
//const MODE_HAS_TAB: u8 = 0x02;
//const MODE_HAS_SEQ: u8 = 0x04;
//const MAX_MODE: u8 = 0x05;

//const SEPARATOR: u16 = 0xFFFF;
//const START_SEQ: u16 = 0xFFFE;

const WIDTH: usize = 8;

//================================================
//  TYPES
//================================================

#[derive(Debug)]
pub struct PSF1<'font> {
    charsize: usize,
    num_glyphs: usize,
    glyphs: &'font [u8],
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl<'font> PSF1<'font> {
    pub fn parse(bytes: &'font [u8]) -> Option<Self> {
        if u16::from_le_bytes(bytes[0..2].try_into().unwrap()) != MAGIC { return None }

        let mode = bytes[2];
        let charsize = bytes[3];

        Some(Self {
            charsize: charsize as usize,
            num_glyphs: if mode & MODE_512 != 0 { 512 } else { 256 },
            glyphs: &bytes[4..],
        })
    }

    /// Returns invalid character glyph for non-supported characters.
    pub fn bitmap(&self, ch: char) -> &'font [u8] {
        let glyph = if (ch as usize) < self.num_glyphs {
            ch as usize
        } else {
            0
        };

        &self.glyphs[glyph * self.charsize .. (glyph + 1) * self.charsize]
    }

    pub fn width(&self) -> usize { WIDTH }

    pub fn height(&self) -> usize { self.charsize }
}