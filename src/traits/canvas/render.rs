use super::*;

pub trait RenderTrait {
    type LayerImpl: layer::LayerTraits;
    type MaskImpl: mask::MaskTraits;

    fn new(dimension: [usize; 2]) -> Self;
    fn render(&mut self) -> &Self::LayerImpl;
    fn clear_cache(&mut self);

    fn insert(&mut self, index: usize, layer: Self::LayerImpl);
    fn pop(&mut self, layer: Self::LayerImpl);
}

// how to add layers/masks ?
