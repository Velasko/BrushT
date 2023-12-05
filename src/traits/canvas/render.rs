use super::*;

pub trait RenderTrait<L, P, C, T>
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn new(dimension: [usize; 2]) -> Self;
	fn render(&mut self) -> &L;
	fn clear_cache(&mut self);

	fn insert(&mut self, index: usize, layer: L);
	fn pop(&mut self, layer: L);
}

// how to add layers/masks ?
