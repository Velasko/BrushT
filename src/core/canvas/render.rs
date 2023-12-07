use std::{rc, marker};

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
	cache: Option<L>,
}

impl<M, L> RenderNode<M, L>
where
	M: mask::MaskTraits,
	L: layer::LayerTraits
{
	fn new(dimension: [usize; 2]) -> Self {
		Self {
			parent: None,
			child: RenderTreeTypes::Layer(L::new(dimension)),
			cache: None,
		}
	}

	fn render(&mut self) -> &L {
		let layer = match self.cache.take() {
			Some(render) => render,
			None => {
				let rendered_layer = match &mut self.child {
					RenderTreeTypes::Mask(mask, tree) => {
						let mut tree = tree.upgrade().unwrap();
						let mut tree_instance = rc::Rc::get_mut(
							&mut tree).unwrap();
						mask.render(tree_instance.render())
					},
					RenderTreeTypes::Layer(layer) => layer.clone(),
					RenderTreeTypes::Tree(left, right) => {
						let mut left_node = left.upgrade().unwrap();
						let mut left_render = rc::Rc::get_mut(
							&mut left_node).unwrap().render();

						let mut right_node = right.upgrade().unwrap();
						let mut right_render = rc::Rc::get_mut(
							&mut right_node).unwrap().render();

						left_render.add(right_render)
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
		match &self.parent {
			None => (),
			Some(parent) => {
				rc::Rc::get_mut(&mut parent.upgrade().unwrap()).unwrap().clear_cache();
			}
		}
	}
}

pub struct RenderTree<M, L> {
	nodes: Vec<rc::Rc<RenderNode<M, L>>>,
	dimension: [usize; 2],
}

impl<M, L> render::RenderTrait for RenderTree<M, L>
where
	M: mask::MaskTraits,
	L: layer::LayerTraits
{
    type LayerImpl = L;
    type MaskImpl = M;

	// How to edit the tree ?

	fn new(dimension: [usize; 2]) -> Self {
		let root = rc::Rc::new(RenderNode::new(dimension));

		Self {
			nodes: vec![root],
			dimension,
		}
	}

	fn render(&mut self) -> &L {
		rc::Rc::get_mut(&mut self.nodes[0]).unwrap().render()
	}

	fn clear_cache(&mut self) {
		rc::Rc::get_mut(&mut self.nodes[0]).unwrap().clear_cache()
	}

	fn insert(&mut self, index: usize, layer: L) {
	}

	fn pop(&mut self, layer: L) {
	}

}
