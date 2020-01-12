use crate::{
    Bounds, ColorMode, Dimension, Position2D, SymbolStyle, SymbolStyles, Texel, Texels, Which,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// Default background color for sprites
pub const DEFAULT_BG_U8: u8 = 16;
/// Default foreground color for sprites
pub const DEFAULT_FG_U8: u8 = 0xE8 + 16;

/// 256 * 256 ascii chars maximum
pub const SPRITE_MAX_BYTES: usize = u16::max_value() as usize;

///
/// Sprite represents a 2D ASCII art picture with frame animation
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Sprite {
    /// List of Frame data consisting of texels
    pub frames: Vec<Texels>,
    /// Current Frame Index
    pub index: usize,
    /// Optional ID number to identify sprite in a scene
    pub id: Option<u32>,
    /// Optional list of labels for grouping sprites in a scene
    pub labels: Vec<String>,
}

///
/// Previous version of the sprite for re-import in scene only
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SpriteV1 {
    pub frames: Vec<Texels>,
    pub index: usize,
}

impl From<SpriteV1> for Sprite {
    fn from(old: SpriteV1) -> Self {
        Sprite {
            frames: old.frames,
            index: old.index,
            id: None,
            labels: Vec::new(),
        }
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            frames: vec![Texels::new()],
            index: 0,
            id: None,
            labels: Vec::new(),
        }
    }
}

impl IntoIterator for Sprite {
    type Item = Texel;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    // turn sprite into active frame contents
    fn into_iter(mut self) -> Self::IntoIter {
        if self.frames.is_empty() {
            Vec::new().into_iter()
        } else {
            self.frames.remove(self.index).into_iter()
        }
    }
}

impl Sprite {
    /// Current frame index for this sprite
    pub fn frame_index(&self) -> usize {
        self.index
    }

    /// Sprite frame count
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Creates a new frame copying contents of current frame
    pub fn new_frame(&mut self) {
        self.frames
            .insert(self.index, self.frames[self.index].clone());
        self.apply_frame_change(Which::Next);
    }

    /// Deletes current frame
    pub fn delete_frame(&mut self) -> bool {
        if self.frames.len() > 1 {
            self.frames.remove(self.index);
            self.apply_frame_change(Which::Previous);
            true
        } else {
            false
        }
    }

    /// Applies given frame change according to the `which` argument
    pub fn apply_frame_change(&mut self, which: Which<usize>) -> usize {
        match which {
            Which::All => self.index, // invalid
            Which::Next => self
                .set_frame(self.index + 1)
                .unwrap_or_else(|_| std::cmp::max(self.frames.len(), 1) - 1),
            Which::Previous => self
                .set_frame(std::cmp::max(self.index, 1) - 1)
                .unwrap_or_else(|_| 0),
            Which::At(index) => self.set_frame(index).unwrap_or_else(|_| self.index),
        }
    }

    /// Sets frae to given index, empty Error if out of bounds
    fn set_frame(&mut self, index: usize) -> Result<usize, ()> {
        self.index = if index >= self.frames.len() {
            return Err(());
        } else {
            index
        };

        Ok(self.index)
    }

    /// Copies an area of given frame in the `area: Bounds` as Vec<Texel>
    pub fn copy_area(&self, area: Bounds) -> Texels {
        let mut result = Texels::new();
        for texel in self.frame_iter().filter(|t| area.contains(t.pos)) {
            result.push(texel.moved_from(*area.position()));
        }

        result
    }

    /// Iterator for list of `Texel` for all frames in the sprite
    pub fn all_iter(&self) -> impl Iterator<Item = &Texel> {
        self.frames.iter().flatten()
    }

    /// Mutable iterator for list of `Texel` for all frames in the sprite
    pub fn all_iter_mut(&mut self) -> impl Iterator<Item = &mut Texel> {
        self.frames.iter_mut().flatten()
    }

    /// Iterator over current frame's list of `Texel`
    pub fn frame_iter(&self) -> impl Iterator<Item = &Texel> {
        self.frames[self.index].iter()
    }

    /// Mutable iterator over current frame's list of `Texel`
    pub fn frame_iter_mut(&mut self) -> impl Iterator<Item = &mut Texel> {
        self.frames[self.index].iter_mut()
    }

    /// Creates a sprite from given text file with default styles and colors
    pub fn from_txt_file(abs_path: &Path) -> Result<Self, std::io::Error> {
        let mut f = File::open(abs_path)?;
        let mut buf: String = String::with_capacity(SPRITE_MAX_BYTES);
        let byte_size = f.read_to_string(&mut buf)?;

        if byte_size > SPRITE_MAX_BYTES {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        }

        let mut texels = Vec::new();

        let mut x = 0;
        let mut y = 0;
        for c in buf.chars() {
            match c {
                ' ' => x += 1,
                '\n' => {
                    x = 0;
                    y += 1;
                }
                _ => {
                    texels.push(Texel {
                        pos: Position2D::from_xy(x, y),
                        symbol: c,
                        styles: SymbolStyles::new(),
                        fg: DEFAULT_FG_U8,
                        bg: DEFAULT_BG_U8,
                    });
                    x += 1;
                }
            }
        }

        Ok(Sprite::from_texels(texels))
    }

    /// Creates a sprite from list of texels, single frame
    pub fn from_texels(texels: Texels) -> Sprite {
        Sprite {
            frames: vec![texels],
            index: 0,
            id: None,
            labels: Vec::new(),
        }
    }

    /// Fills entire frame with color according to the `ColorMode`
    pub fn fill_color(&mut self, cm: ColorMode, color: u8) -> bool {
        let bounds = self.calculate_bounds();

        self.apply_color(cm, color, bounds)
    }

    /// Fills entire frame with given `SymbolStyle`
    pub fn fill_style(&mut self, style: SymbolStyle) -> bool {
        let bounds = self.calculate_bounds();

        self.apply_style(style, bounds)
    }

    /// Applies *symbol* with *bg/fg* color combination in given `Bounds` *area*
    pub fn apply_symbol(&mut self, symbol: char, bg: u8, fg: u8, area: Bounds) -> Bounds {
        // remove texels in bounds
        self.frames[self.index].retain(|t| !area.contains(t.pos));

        // re-add them with new setup
        for pos in area.into_iter() {
            self.frames[self.index].push(Texel {
                symbol,
                bg,
                fg,
                pos,
                styles: SymbolStyles::new(),
            });
        }

        self.calculate_bounds()
    }

    /// Applies *texels* starting at given *pos* `Position2D`
    pub fn apply_texels(&mut self, texels: Texels, pos: Position2D) -> Bounds {
        for texel in texels.into_iter() {
            let mut localized = texel.clone();
            localized.pos += pos;

            if let Some(existing) = self.frames[self.index]
                .iter_mut()
                .find(|t| t.pos == localized.pos)
            {
                *existing = localized;
            } else {
                self.frames[self.index].push(localized);
            }
        }

        self.calculate_bounds()
    }

    /// Applies *color* according to `ColorMode` in the given `Bounds` *area*
    pub fn apply_color(&mut self, cm: ColorMode, color: u8, area: Bounds) -> bool {
        let mut changed = false;
        let mut new_texels = Vec::with_capacity(self.frames[self.index].capacity());

        for pos in area.into_iter() {
            if let Some(texel) = self.frame_iter_mut().find(|t| t.pos == pos) {
                match cm {
                    ColorMode::Bg => texel.bg = color,
                    ColorMode::Fg => texel.fg = color,
                }
                changed = true;
            } else {
                let (bg, fg) = match cm {
                    ColorMode::Bg => (color, DEFAULT_FG_U8),
                    ColorMode::Fg => (DEFAULT_BG_U8, color),
                };
                // add each missing "background" texel
                new_texels.push(Texel {
                    pos,
                    fg,
                    bg,
                    styles: SymbolStyles::new(),
                    symbol: ' ',
                });

                changed = true;
            }
        }

        // apply the new texel list
        self.apply_texels(new_texels, Position2D::from_xy(0, 0));

        changed
    }

    /// Applies a single *style* for the given `Bounds` *area*
    pub fn apply_style(&mut self, style: SymbolStyle, area: Bounds) -> bool {
        let mut changed = false;

        for t in self.frame_iter_mut().filter(|t| area.contains(t.pos)) {
            if t.styles.contains(style) {
                t.styles.remove(style);
            } else {
                t.styles.insert(style);
            }

            changed = true;
        }

        changed
    }

    /// Removes texels in given `Bounds` *area*
    pub fn clear_symbol(&mut self, area: Bounds) -> Option<Bounds> {
        let count = self.frames[self.index].len();
        self.frames[self.index].retain(|t| !area.contains(t.pos));

        if count != self.frames[self.index].len() {
            return Some(self.calculate_bounds());
        }

        None
    }

    /// Empty check, true if all frames empty
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
            || self
                .frames
                .iter()
                .map(|inner| inner.is_empty())
                .min()
                .unwrap_or_else(|| false)
    }

    // goes through texels so we can calculate dimension and move position if
    // needed. TODO: optimize, we're doing 3 loops here for no good reason
    fn calculate_bounds(&mut self) -> Bounds {
        if self.is_empty() {
            return Bounds::empty();
        }

        let mut min_x = i32::max_value();
        let mut min_y = i32::max_value();

        // get new top/left
        for t in self.all_iter() {
            if t.pos.x < min_x {
                min_x = t.pos.x;
            }
            if t.pos.y < min_y {
                min_y = t.pos.y;
            }
        }

        // shift texels as needed
        if min_x != 0 || min_y != 0 {
            for t in self.all_iter_mut() {
                if min_x != 0 {
                    t.pos.x -= min_x;
                }
                if min_y != 0 {
                    t.pos.y -= min_y;
                }
            }
        }

        Bounds::Free(
            Position2D { x: min_x, y: min_y },
            Dimension::for_sprite(self),
        )
    }
}
