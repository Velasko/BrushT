use std::{rc, marker};

use super::traits::*;

pub enum RenderTreeTypes<M, L, P, C, T>
{
	Layer(L),
	Mask(M, rc::Weak<RenderNode<M, L, P, C, T>>),
	Tree(rc::Weak<RenderNode<M, L, P, C, T>>, rc::Weak<RenderNode<M, L, P, C, T>>),
}

pub struct RenderNode<M, L, P, C, T>
{
	parent: Option<rc::Weak<Self>>,
	child: RenderTreeTypes<M, L, P, C, T>,
	cache: Option<L>,
	_P: marker::PhantomData<P>,
	_C: marker::PhantomData<C>,
	_T: marker::PhantomData<T>
}

impl<M, L, P, C, T> RenderNode<M, L, P, C, T>
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
			cache: None,
			_P: marker::PhantomData,
			_C: marker::PhantomData,
			_T: marker::PhantomData,
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

pub struct RenderTree<M, L, P, C, T> {
	nodes: Vec<rc::Rc<RenderNode<M, L, P, C, T>>>,
	dimension: [usize; 2],
}

impl<M, L, P, C, T> render::RenderTrait<L, P, C, T> for RenderTree<M, L, P, C, T>
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
