use super::*;

pub trait RenderTrait<L, P, C, T>
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn render(&self) -> L;
}
