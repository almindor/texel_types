use crate::{Position2D, SymbolStyle};
use big_enum_set::BigEnumSet;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

///
/// Base building block, "textual pixel" consisting of coordinates, symbol, styles and colors
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Texel {
    pub pos: Position2D,
    pub symbol: char,
    /// Set of `SymbolStyle` styles
    pub styles: BigEnumSet<SymbolStyle>, // u8
    /// Foreground color as `termion::color::AsciiValue.0` u8
    pub fg: u8,
    /// Background color as `termion::color::AsciiValue.0` u8
    pub bg: u8,
}

/// Vector of Texels
pub type Texels = Vec<Texel>;

impl Texel {
    /// Clones this texel moved to a new position
    pub fn moved_from(&self, pos: Position2D) -> Self {
        let mut result = self.clone();

        result.pos -= pos;

        result
    }
}
