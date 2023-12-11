use super::*;

pub trait MaskTraits {
    fn render<L>(&self, layer: &L) -> L
    where
        L: layer::LayerTraits;
}
