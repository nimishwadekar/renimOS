
//================================================
//  TYPES
//================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl Colour {
    #[allow(dead_code)]
    pub const BLACK: Colour = Colour::new(0, 0, 0);
    #[allow(dead_code)]
    pub const WHITE: Colour = Colour::new(0xFF, 0xFF, 0xFF);
    #[allow(dead_code)]
    pub const RED: Colour = Colour::new(0xFF, 0, 0);
    #[allow(dead_code)]
    pub const BLUE: Colour = Colour::new(0, 0, 0xFF);
    #[allow(dead_code)]
    pub const GREEN: Colour = Colour::new(0, 0xFF, 0);
    #[allow(dead_code)]
    pub const CYAN: Colour = Colour::new(0, 0xFF, 0xFF);
    #[allow(dead_code)]
    pub const MAGENTA: Colour = Colour::new(0xFF, 0, 0xFF);
    #[allow(dead_code)]
    pub const YELLOW: Colour = Colour::new(0xFF, 0xFF, 0);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}