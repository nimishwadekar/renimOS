
//================================================
//  CONSTANTS
//================================================

const HEADER_SIZE: usize = 32;

const MAGIC: u32 = 0x864ab572;

//const FLAG_HAS_TABLE: u32 = 0x01;
//const SEPARATOR: u8 = 0xFF;
//const START_SEQ: u8 = 0xFE;

//================================================
//  TYPES
//================================================

#[derive(Debug)]
pub struct PSF2<'font> {
    num_glyphs: usize,
    bytes_per_glyph: usize,
    height: usize,
    width: usize,
    glyphs: &'font [u8],
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl<'font> PSF2<'font> {
    pub fn parse(bytes: &'font [u8]) -> Option<Self> {
        let mut header_iter = bytes[0..HEADER_SIZE]
            .chunks_exact(4)
            .map(|e| u32::from_le_bytes(e.try_into().unwrap()));

        if header_iter.next().unwrap() != MAGIC { return None }
        header_iter.next().unwrap(); // version
        assert_eq!(header_iter.next().unwrap() as usize, HEADER_SIZE);
        header_iter.next().unwrap(); // flags
        let num_glyphs = header_iter.next().unwrap() as usize;
        let bytes_per_glyph = header_iter.next().unwrap() as usize;
        let height = header_iter.next().unwrap() as usize;
        let width = header_iter.next().unwrap() as usize;
        assert_eq!(header_iter.next(), None);

        Some(Self {
            num_glyphs,
            bytes_per_glyph,
            height,
            width,
            glyphs: &bytes[HEADER_SIZE..],
        })
    }

    /// Returns invalid character glyph for non-supported characters.
    pub fn bitmap(&self, ch: char) -> &'font [u8] {
        let glyph = if (ch as usize) < self.num_glyphs {
            ch as usize
        } else {
            0
        };

        &self.glyphs[glyph * self.bytes_per_glyph .. (glyph + 1) * self.bytes_per_glyph]
    }

    pub fn width(&self) -> usize { self.width }

    pub fn height(&self) -> usize { self.height }
}