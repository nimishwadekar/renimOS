use psf1::PSF1;

mod psf1;

//================================================
//  TYPES
//================================================

#[derive(Debug)]
pub enum PSF<'font> {
    PSF1(PSF1<'font>),
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl<'font> PSF<'font> {
    pub fn parse(bytes: &'font [u8]) -> Option<Self> {
        if let Some(psf1) = PSF1::parse(bytes) { Some(PSF::PSF1(psf1)) }
        else { None }
    }

    pub fn get_bitmap(&self, ch: char) -> &'font [u8] {
        match self {
            PSF::PSF1(psf) => psf.get_bitmap(ch),
        }
    }
}