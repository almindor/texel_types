use crate::Dimension;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// 3D position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// 2D position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Position2D {
    pub x: i32,
    pub y: i32,
}

///
/// Two dimensional boundary consisting of `Position2D` and `Dimension`
/// Bounds can be either free standing or binding
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bounds {
    /// Binding bounds constrain movement to their area
    Binding(Position2D, Dimension),
    /// Free standing bounds specify an area but do not constrain
    Free(Position2D, Dimension),
}

impl std::iter::IntoIterator for Bounds {
    type Item = Position2D;
    type IntoIter = BoundsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoundsIntoIterator {
            bounds: self,
            index: 0,
        }
    }
}

///
/// Bounds iterator that returns Position2D elements contained in given area
///
pub struct BoundsIntoIterator {
    bounds: Bounds,
    index: usize,
}

impl std::iter::Iterator for BoundsIntoIterator {
    type Item = Position2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bounds.size() {
            return None;
        }

        let old_index = self.index;
        self.index += 1;

        if let Some(pos) = coords_from_index(old_index, *self.bounds.dimension()) {
            Some(pos + *self.bounds.position())
        } else {
            None
        }
    }
}

impl std::ops::Sub<Position2D> for Bounds {
    type Output = Bounds;

    fn sub(self, other: Position2D) -> Self::Output {
        match self {
            Bounds::Binding(pos, dim) => Bounds::Binding(pos - other, dim),
            Bounds::Free(pos, dim) => Bounds::Free(pos - other, dim),
        }
    }
}

impl Bounds {
    /// Empty bounds constructor, sets `Position` to 0, 0
    pub fn empty() -> Self {
        Bounds::Free(Position2D { x: 0, y: 0 }, Dimension::default())
    }

    /// Single point bounds constructor with unit size for given position
    pub fn point(pos: Position2D) -> Self {
        Bounds::Binding(pos, Dimension::unit())
    }

    /// Position component accessor
    pub fn position(&self) -> &Position2D {
        match self {
            Bounds::Binding(p, _) => p,
            Bounds::Free(p, _) => p,
        }
    }

    /// Dimension component accessor
    pub fn dimension(&self) -> &Dimension {
        match self {
            Bounds::Binding(_, d) => d,
            Bounds::Free(_, d) => d,
        }
    }

    /// Area size calculation as dimension.w * dimension.h
    pub fn size(&self) -> usize {
        self.dimension().size()
    }

    /// Right side point for given bounds area
    pub fn right(&self) -> i32 {
        self.position().x + i32::from(self.dimension().w) - 1
    }

    /// Bottom side point for given bounds area
    pub fn bottom(&self) -> i32 {
        self.position().y + i32::from(self.dimension().h) - 1
    }

    /// Checks if given coordinates are inside this bounded area
    pub fn contains(&self, other: Position2D) -> bool {
        let pos = self.position();
        let dim = self.dimension();

        other.x >= pos.x
            && other.x < pos.x + i32::from(dim.w)
            && other.y >= pos.y
            && other.y < pos.y + i32::from(dim.h)
    }

    /// Calculates rectangular intersection
    pub fn intersects(&self, pos: Position2D, dim: Dimension) -> bool {
        let top_edge1 = self.position().y + i32::from(self.dimension().h);
        let right_edge1 = self.position().x + i32::from(self.dimension().w);
        let left_edge1 = self.position().x;
        let bottom_edge1 = self.position().y;
        let top_edge2 = pos.y + i32::from(dim.h);
        let right_edge2 = pos.x + i32::from(dim.w);
        let left_edge2 = pos.x;
        let bottom_edge2 = pos.y;

        left_edge1 < right_edge2
            && right_edge1 > left_edge2
            && bottom_edge1 < top_edge2
            && top_edge1 > bottom_edge2
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0, z: 0 }
    }
}

impl Default for Position2D {
    fn default() -> Self {
        Position2D { x: 0, y: 0 }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.z != 0 {
            write!(f, "{},{},{}", self.x, self.y, self.z)
        } else {
            write!(f, "{},{}", self.x, self.y)
        }
    }
}

impl std::fmt::Display for Position2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl From<&mut Position> for Position2D {
    fn from(pos: &mut Position) -> Position2D {
        Position2D { x: pos.x, y: pos.y }
    }
}

impl From<Position> for Position2D {
    fn from(pos: Position) -> Position2D {
        Position2D { x: pos.x, y: pos.y }
    }
}

impl From<&Position> for Position2D {
    fn from(pos: &Position) -> Position2D {
        Position2D { x: pos.x, y: pos.y }
    }
}

impl std::ops::Add<Position2D> for Position2D {
    type Output = Position2D;

    fn add(self, other: Position2D) -> Self::Output {
        Position2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Position2D> for Position {
    type Output = Position;

    fn add(self, other: Position2D) -> Self::Output {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z,
        }
    }
}

impl std::ops::Add<i32> for Position {
    type Output = Position;

    fn add(self, value: i32) -> Self::Output {
        Position {
            x: self.x + value,
            y: self.y + value,
            z: self.z, // keep z
        }
    }
}

impl std::ops::AddAssign<Position2D> for Position {
    fn add_assign(&mut self, other: Position2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::AddAssign<i32> for Position {
    fn add_assign(&mut self, value: i32) {
        self.x += value;
        self.y += value;
    }
}

impl std::ops::AddAssign<Position2D> for Position2D {
    fn add_assign(&mut self, other: Position2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub for Position2D {
    type Output = Position2D;

    fn sub(self, other: Self) -> Self {
        Position2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Sub<Position> for Position2D {
    type Output = Position2D;

    fn sub(self, other: Position) -> Self {
        Position2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign<Position2D> for Position2D {
    fn sub_assign(&mut self, other: Position2D) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Position {
    ///
    /// Applies given `Translation` to this `Position` with regards to the provided
    /// `Bounds` area. If `Bounds` is binding ensures position does not reach outside.
    ///
    pub fn apply(&mut self, translation: Translation, bounds: Bounds) -> bool {
        match translation {
            Translation::None => {}
            Translation::Relative(x, y, z) => {
                self.x += x;
                self.y += y;
                self.z += z;
            }
            Translation::Absolute(x, y, z) => {
                self.x = x;
                self.y = y;
                if let Some(z) = z {
                    self.z = z;
                }
            }
            Translation::ToEdge(dir) => match dir {
                Direction::Left => self.x = bounds.position().x,
                Direction::Top => self.y = bounds.position().y,
                Direction::Bottom => self.y = bounds.bottom(),
                Direction::Right => self.x = bounds.right(),
            },
        }

        match bounds {
            Bounds::Binding(p, _) => {
                if self.x < p.x {
                    self.x = p.x;
                    false
                } else if self.y < p.y {
                    self.y = p.y;
                    false
                } else if self.x > bounds.right() {
                    self.x = bounds.right();
                    false
                } else if self.y > bounds.bottom() {
                    self.y = bounds.bottom();
                    false
                } else {
                    true
                }
            }
            _ => true,
        }
    }
}

impl Position2D {
    pub fn from_xy(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    ///
    /// Applies given `Translation` to this `Position2D` with regards to the provided
    /// `Bounds` area. If `Bounds` is binding ensures position does not reach outside.
    ///
    pub fn apply(&mut self, translation: Translation, bounds: Bounds) -> bool {
        let mut pos3d = Position {
            x: self.x,
            y: self.y,
            z: 0,
        };

        if pos3d.apply(translation, bounds) {
            self.x = pos3d.x;
            self.y = pos3d.y;
            true
        } else {
            false
        }
    }

    /// Create bounds from two points
    pub fn area(self, other: Position2D) -> Bounds {
        let top_left = Position2D {
            x: std::cmp::min(self.x, other.x),
            y: std::cmp::min(self.y, other.y),
        };
        let bottom_right = Position2D {
            x: std::cmp::max(self.x, other.x),
            y: std::cmp::max(self.y, other.y),
        };

        let dim = Dimension::for_area(top_left, bottom_right);

        Bounds::Binding(top_left, dim)
    }

    /// Create the list of all positions in given area from point with given dimension
    pub fn area_texels(self, dim: Dimension) -> Vec<Position2D> {
        let mut result = Vec::with_capacity(dim.size());

        for x in self.x..self.x + i32::from(dim.w) {
            for y in self.y..self.y + i32::from(dim.h) {
                result.push(Position2D { x, y });
            }
        }

        result
    }
}

///
/// Describes a direction
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Top,
    Bottom,
    Right,
}

///
/// Describes the translation operation
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Translation {
    /// None for avoiding the need for Option<>
    None,
    /// Relative to current `Position`
    Relative(i32, i32, i32),
    /// Absolute with optional Z coordinate
    Absolute(i32, i32, Option<i32>),
    /// To edge of constrained area in given direction
    ToEdge(Direction),
}

impl Default for Translation {
    fn default() -> Self {
        Translation::None
    }
}

fn coords_from_index(index: usize, dim: Dimension) -> Option<Position2D> {
    let i = index as i32;
    let w = i32::from(dim.w);
    let h = i32::from(dim.h);

    if i < w * h {
        Some(Position2D { x: i % w, y: i / w })
    } else {
        None
    }
}
