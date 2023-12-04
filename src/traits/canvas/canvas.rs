use super::*;

pub trait CanvasTraits<L, P, C, T>:
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn new(dimension: [usize; 2]) -> Self;
 	fn render(&mut self) -> L;
}
