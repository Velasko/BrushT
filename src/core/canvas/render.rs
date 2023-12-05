use std::rc;

use super::traits::*;

pub enum RenderTreeTypes<M, L>
{
	Layer(L),
	Mask(M, rc::Weak<RenderNode<M, L>>),
	Tree(rc::Weak<RenderNode<M, L>>, rc::Weak<RenderNode<M, L>>),
}

pub struct RenderNode<M, L>
{
	parent: Option<rc::Weak<Self>>,
	child: RenderTreeTypes<M, L>,
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
	fn new(dimension: [usize; 2]) -> Self {
		Self {
			parent: None,
			child: RenderTreeTypes::Layer(L::new(dimension)),
			cache: None
		}
	}

	fn render(&mut self) -> &L {
		let layer = match self.cache.take() {
			Some(render) => render,
			None => {
				let rendered_layer = match &mut self.child {
					RenderTreeTypes::Mask(mask, tree) =>
						mask.render(tree.upgrade().unwrap().render()),
					RenderTreeTypes::Layer(layer) => layer.clone(),
					RenderTreeTypes::Tree(ref mut left, ref mut right) => {
						left.upgrade().unwrap().render().add(
							right.upgrade().unwrap().render()
						)
					}
				};
				rendered_layer
			}
		};
		self.cache = Some(layer);
		self.cache.as_ref().expect("Render cache was just defined, yet became None !")
	}

	fn clear_cache(&mut self) {
		self.cache = None;
		match self.parent {
			None => (),
			Some(parent) => {
				parent.upgrade().unwrap().render();
			}
		}
	}
}

pub struct RenderTree<M, L> {
	nodes: Vec<rc::Rc<RenderNode<M, L>>>,
	dimension: [usize; 2],
	root: rc::Weak<RenderNode<M, L>>
}

impl<M, L, P, C, T> render::RenderTrait<L, P, C, T> for RenderTree<M, L>
where
	M: mask::MaskTraits<L, P, C, T>,
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>,
{
	// How to edit the tree ?

	fn new(dimension: [usize; 2]) -> Self {
		let root = rc::Rc::new(RenderNode::new(dimension));
		let root_ref = rc::Rc::downgrade(&root);

		Self {
			nodes: vec![root],
			dimension,
			root: root_ref
		}
	}

	fn render(&mut self) -> &L {
		self.root.upgrade().unwrap().render()
	}

	fn clear_cache(&mut self) {
		self.root.upgrade().unwrap().clear_cache()
	}

	fn insert(&mut self, index: usize, layer: L) {
	}

	fn pop(&mut self, layer: L) {
	}

}
