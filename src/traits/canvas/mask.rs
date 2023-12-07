use super::*;

pub trait MaskTraits
{
	fn new(size: usize) -> Self;

	fn render<L>(&self, layer: &L) -> L where L: layer::LayerTraits;
}
