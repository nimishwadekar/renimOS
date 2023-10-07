use psf1::PSF1;
use psf2::PSF2;

mod psf1;
mod psf2;

//================================================
//  TYPES
//================================================

#[derive(Debug)]
pub struct PSF<'font> {
    pub width: usize,
    pub height: usize,

    psf_type: PSFType<'font>,
}

#[derive(Debug)]
pub enum PSFType<'font> {
    PSF1(PSF1<'font>),
    PSF2(PSF2<'font>),
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl<'font> PSF<'font> {
    pub fn parse(bytes: &'font [u8]) -> Option<Self> {
        let psf_type = PSFType::parse(bytes)?;
        Some(Self {
            width: psf_type.width(),
            height: psf_type.height(),

            psf_type,
        })
    }

    pub fn bitmap(&self, ch: char) -> &'font [u8] { self.psf_type.bitmap(ch) }
}

impl<'font> PSFType<'font> {
    fn parse(bytes: &'font [u8]) -> Option<Self> {
        if let Some(psf1) = PSF1::parse(bytes) { Some(PSFType::PSF1(psf1)) }
        else if let Some(psf2) = PSF2::parse(bytes) { Some(PSFType::PSF2(psf2)) }
        else { None }
    }

    fn bitmap(&self, ch: char) -> &'font [u8] {
        match self {
            Self::PSF1(font) => font.bitmap(ch),
            Self::PSF2(font) => font.bitmap(ch),
        }
    }

    fn width(&self) -> usize {
        match self {
            Self::PSF1(font) => font.width(),
            Self::PSF2(font) => font.width(),
        }
    }

    fn height(&self) -> usize {
        match self {
            Self::PSF1(font) => font.height(),
            Self::PSF2(font) => font.height(),
        }
    }
}