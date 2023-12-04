use super::traits::*;

enum CanvasContent<P, C> {
 	Canvas(Canvas<P, C>),
 	Layer(super::layer::Layer<P, C>),
}

pub struct Canvas<P, C>
{
	content: Vec<CanvasContent<P, C>>
}

impl<L, P, C, T> canvas::CanvasTraits<L, P, C, T> for Canvas<P, C>
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>
{
	fn new() -> Self {
		Self {
			content: Vec::new()
		}
	}
}
