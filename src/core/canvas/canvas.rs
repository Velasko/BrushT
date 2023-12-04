use super::traits::*;

pub struct Canvas<R>
{
	content: R
}

impl<R, L, P, C, T> canvas::CanvasTraits<L, P, C, T> for Canvas<R>
where
	R: render::RenderTrait<L, P, C, T>,
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn new(dimension: [usize; 2]) -> Self {
		Self {
			content: R::new(dimension)
		}
	}

	fn render(&mut self) -> L {
		self.content.render().clone()
	}
}
