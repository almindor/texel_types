use crate::{Position2D, SymbolStyles, DEFAULT_BG_U8, DEFAULT_FG_U8};

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
    pub styles: SymbolStyles, // u8
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

/// Create a Texels vector from &str
pub fn texels_from_str(s: &str, start: Position2D) -> Texels {
    let mut result = Vec::with_capacity(s.len());

    for (i, c) in s.chars().enumerate() {
        result.push(Texel {
            symbol: c,
            pos: Position2D { x: start.x + i as i32, y: start.y },
            styles: SymbolStyles::new(),
            bg: DEFAULT_BG_U8,
            fg: DEFAULT_FG_U8,
        });
    }

    result
}

/// 
/// Writes given &str to Texels list starting at given position
/// *NOTE* - does not expand the list, if EOL would be reached false it returned
/// 
pub fn write_to_texels(s: &str, texels: &mut Texels, start_x: usize) -> bool {
    if start_x + s.len() > texels.len() {
        return false; // no expansion
    }

    for (i, c) in s.chars().enumerate() {
        texels[start_x + i].symbol = c;
    }

    true
}