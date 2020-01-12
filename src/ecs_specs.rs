use specs::{Component, VecStorage};

impl Component for crate::Position {
    type Storage = VecStorage<Self>;
}

impl Component for crate::Position2D {
    type Storage = VecStorage<Self>;
}

impl Component for crate::Dimension {
    type Storage = VecStorage<Self>;
}

impl Component for crate::Sprite {
    type Storage = VecStorage<Self>;
}
