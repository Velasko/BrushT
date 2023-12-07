use super::traits::*;

pub struct Canvas<R>
where
	R: render::RenderTrait
{
	content: R
}

impl<R> canvas::CanvasTraits for Canvas<R>
where
	R: render::RenderTrait
{
	type RenderImpl = R;
	type LayerImpl = R::LayerImpl;

	fn new(dimension: [usize; 2]) -> Self {
		Self {
			content: R::new(dimension)
		}
	}

	fn render(&mut self) -> Self::LayerImpl {
		self.content.render().clone()
	}
}
