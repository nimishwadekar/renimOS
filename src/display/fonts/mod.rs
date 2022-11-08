use psf1::PSF1;

mod psf1;

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
        else { None }
    }

    fn bitmap(&self, ch: char) -> &'font [u8] {
        match self {
            Self::PSF1(psf) => psf.bitmap(ch),
        }
    }

    fn width(&self) -> usize {
        match self {
            Self::PSF1(font) => font.width(),
        }
    }

    fn height(&self) -> usize {
        match self {
            Self::PSF1(font) => font.height(),
        }
    }
}