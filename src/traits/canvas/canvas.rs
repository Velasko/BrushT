use super::*;

pub trait CanvasTraits<L, P, C, T>:
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn new() -> Self;

// 	fn get_preview(&self) -> L;
// 	fn render(&self) -> L;
//
// 	fn refresh(&mut self);
}
