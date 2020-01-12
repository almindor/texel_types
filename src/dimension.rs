use crate::{Position2D, Sprite};
use std::convert::TryInto;

/// 2D dimension of up to 16bit size
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimension {
    /// width
    pub w: u16,
    /// height
    pub h: u16,
}

impl From<(u16, u16)> for Dimension {
    fn from(source: (u16, u16)) -> Self {
        Dimension {
            w: source.0,
            h: source.1,
        }
    }
}

impl std::ops::Sub for Dimension {
    type Output = Dimension;

    fn sub(self, other: Self) -> Self::Output {
        let w = if other.w > self.w {
            0
        } else {
            self.w - other.w
        };
        let h = if other.h > self.h {
            0
        } else {
            self.h - other.h
        };

        Dimension { w, h }
    }
}

impl Dimension {
    /// Single unit size dimension constructor (w = 1, h = 1)
    pub fn unit() -> Self {
        Dimension { w: 1, h: 1 }
    }

    /// Returns area size as w * h
    pub fn size(self) -> usize {
        usize::from(self.w * self.h)
    }

    /// Calculates dimension between two 2D points, unit size for same point!
    pub fn for_area(top_left: Position2D, bottom_right: Position2D) -> Self {
        Dimension {
            w: (bottom_right.x - top_left.x + 1) as u16,
            h: (bottom_right.y - top_left.y + 1) as u16,
        }
    }

    /// Arbitrary width and height constructor
    pub fn from_wh(w: u16, h: u16) -> Self {
        Dimension { w, h }
    }

    /// Calculates dimension for a `Sprite`
    pub fn for_sprite(sprite: &Sprite) -> Self {
        let mut w32 = 0i32;
        let mut h32 = 0i32;

        for t in sprite.all_iter() {
            if t.pos.x > w32 {
                w32 = t.pos.x;
            }
            if t.pos.y > h32 {
                h32 = t.pos.y;
            }
        }

        w32 += 1;
        h32 += 1;

        Dimension {
            w: w32.try_into().unwrap_or_else(|_| 0),
            h: h32.try_into().unwrap_or_else(|_| 0),
        }
    }
}
