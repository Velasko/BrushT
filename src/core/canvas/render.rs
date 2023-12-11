use super::traits::*;
use crate::traits::canvas::render::RenderTrait;

pub struct RenderTree<L>
where
    L: layer::LayerTraits,
{
    nodes: Vec<render::RenderNode<Self>>,
    cache: Vec<Option<L>>,
    dimension: [usize; 2],
}

impl<L> RenderTree<L>
where
    L: layer::LayerTraits,
{
    fn insert(&mut self, index: usize, content: render::RenderNode<Self>) {
        self.nodes.insert(index, content);
        self.cache.resize(self.nodes.len() - 1, None);
    }

    fn recursive_render(&mut self, cache_index: usize) -> Option<&L> {
        match self.cache[cache_index] {
            Some(layer) => Some(&layer),
            None => {
                let layer = if cache_index >= self.cache.len() {
                    let left_index = cache_index - (1 << (self.cache.len().ilog2() + 1));
                    let right_index = left_index + 1;
                    match self.nodes.get(right_index) {
                        None => &self.nodes[self.nodes.len()],
                        Some(render_node) => {
                            let right = match render_node {
                                render::RenderNode::Layer(layer) => layer.clone(),
                                render::RenderNode::Mask(mask, subrender) => {
                                    mask.render(subrender.render())
                                }
                            };
                            let left = match self.nodes[left_index] {
                                render::RenderNode::Layer(layer) => layer.clone(),
                                render::RenderNode::Mask(mask, subrender) => {
                                    mask.render(subrender.render())
                                }
                            };
                            left.add(&right)
                        }
                    }
                } else {
                    let left_index = (cache_index << 1) + 1;
                    let right_index = left_index + 1;

                    self.recursive_render(left_index)?
                        .add(self.recursive_render(right_index)?)
                };

                self.cache[cache_index] = Some(layer);
                Some(&layer)
            }
        }
    }
}

impl<L> render::RenderTrait for RenderTree<L>
where
    L: layer::LayerTraits,
{
    type LayerImpl = L;

    fn new(dimension: [usize; 2]) -> Self {
        let root = render::RenderNode::Layer(L::new(dimension));

        Self {
            nodes: vec![root],
            cache: vec![],
            dimension,
        }
    }

    fn render(&mut self) -> &L {
        self.recursive_render(0).unwrap()
    }

    fn clear_cache(&mut self) {
        unimplemented!();
    }

    fn insert_layer(&mut self, index: usize, layer: Self::LayerImpl) {
        self.insert(index, render::RenderNode::Layer(layer))
    }

    fn insert_mask(&mut self, index: usize, mask: impl mask::MaskTraits) {
        self.insert(
            index,
            render::RenderNode::Mask(mask, Self::new(self.dimension)),
        )
    }

    fn pop(&mut self, layer: &L) {
        unimplemented!()
    }

    fn get(&self, index: usize) -> &L {
        unimplemented!();
    }

    fn get_id(&mut self, layer: &Self::LayerImpl) {
        unimplemented!();
    }
}

// pub struct RenderNode<M, L> {
//     parent: Option<rc::Weak<Self>>,
//     child: RenderTreeTypes<M, L>,
//     cache: Option<L>,
// }
//
// impl<M, L> RenderNode<M, L>
// where
//     M: mask::MaskTraits,
//     L: layer::LayerTraits,
// {
//     fn new(dimension: [usize; 2]) -> Self {
//         Self {
//             parent: None,
//             child: RenderTreeTypes::Layer(L::new(dimension)),
//             cache: None,
//         }
//     }
//
//     fn render(&mut self) -> &L {
//         let layer = match self.cache.take() {
//             Some(render) => render,
//             None => {
//                 let rendered_layer = match &mut self.child {
//                     RenderTreeTypes::Mask(mask, tree) => {
//                         let mut tree = tree.upgrade().unwrap();
//                         let mut tree_instance = rc::Rc::get_mut(&mut tree).unwrap();
//                         mask.render(tree_instance.render())
//                     }
//                     RenderTreeTypes::Layer(layer) => layer.clone(),
//                     RenderTreeTypes::Tree(left, right) => {
//                         let mut left_node = left.upgrade().unwrap();
//                         let mut left_render = rc::Rc::get_mut(&mut left_node).unwrap().render();
//
//                         let mut right_node = right.upgrade().unwrap();
//                         let mut right_render = rc::Rc::get_mut(&mut right_node).unwrap().render();
//
//                         left_render.add(right_render)
//                     }
//                 };
//                 rendered_layer
//             }
//         };
//         self.cache = Some(layer);
//         self.cache
//             .as_ref()
//             .expect("Render cache was just defined, yet became None !")
//     }
//
//     fn clear_cache(&mut self) {
//         self.cache = None;
//         match &self.parent {
//             None => (),
//             Some(parent) => {
//                 rc::Rc::get_mut(&mut parent.upgrade().unwrap())
//                     .unwrap()
//                     .clear_cache();
//             }
//         }
//     }
// }
