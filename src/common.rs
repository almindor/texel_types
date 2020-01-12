use big_enum_set::{BigEnumSet, BigEnumSetType};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

///
/// Symbol styles enum
///
#[derive(Debug, BigEnumSetType)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum SymbolStyle {
    Bold,
    Italic,
    Underline,
}

///
/// ColorMode enum for background/foreground selection
///
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum ColorMode {
    /// Background mode
    Bg,
    /// Foreground mode
    Fg,
}

///
/// Generic "which" selector for selections etc.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Which<P> {
    /// All
    All,
    /// Next selection
    Next,
    /// Previous selection
    Previous,
    /// Specific index selection
    At(P),
}

/// Set of `SymbolStyle`
pub type SymbolStyles = BigEnumSet<SymbolStyle>;
