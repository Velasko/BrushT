use std::rc;

use super::traits::*;

pub enum RenderTree<M, L>
{
	Layer(L),
	Mask(M, Box<RenderNode<M, L>>),
	Tree(Box<RenderNode<M, L>>, Box<RenderNode<M, L>>),
}

pub struct RenderNode<M, L>
{
	child: RenderTree<M, L>,
	cache: Option<L>
}

impl<M, L, P, C, T> render::RenderTrait<L, P, C, T> for RenderNode<M, L>
where
	M: mask::MaskTraits<L, P, C, T>,
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>,
{
	fn render(&mut self) -> &L {
		let layer = match self.cache.take() {
			Some(render) => render,
			None => {
				let rendered_layer = match &mut self.child {
					RenderTree::Mask(mask, tree) => mask.render(tree.render()),
					RenderTree::Layer(layer) => layer.clone(),
					RenderTree::Tree(ref mut left, ref mut right) => {
						left.render().add(right.render())
					}
				};
				rendered_layer
			}
		};
		self.cache = Some(layer);
		self.cache.as_ref().expect("Render cache was just defined, yet became None !")
	}
}
