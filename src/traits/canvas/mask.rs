use super::*;

pub trait MaskTraits<L, P, C, T>
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn new(matrix: [[f64; 4]; 4]) -> Self;

	fn render(&self, layer: &L) -> L;
}
