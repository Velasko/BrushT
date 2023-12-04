use super::traits::*;

// enum CanvasContent {
// 	canvas(Canvas),
// 	layer(Layer)
// }

pub struct Canvas
{
	content: Vec<Canvas>
}

impl<L, P, C, T> canvas::CanvasTraits<L, P, C, T> for Canvas
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
