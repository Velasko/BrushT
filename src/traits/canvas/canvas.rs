use super::*;

pub trait CanvasTraits:
{
	type RenderImpl: render::RenderTrait;
	type LayerImpl: layer::LayerTraits;

	fn new(dimension: [usize; 2]) -> Self;
 	fn render(&mut self) -> Self::LayerImpl;
}
