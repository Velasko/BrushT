use super::*;

pub enum RenderNode<R>
where
    R: RenderTrait,
{
    Layer(R::LayerImpl),
    Mask(Box<dyn mask::MaskTraits>, R),
}

pub trait RenderTrait: Sized {
    type LayerImpl: layer::LayerTraits;

    fn new(dimension: [usize; 2]) -> Self;
    fn render(&mut self) -> &Self::LayerImpl;
    fn clear_cache(&mut self);

    fn insert_layer(&mut self, index: usize, layer: Self::LayerImpl);
    fn insert_mask(&mut self, index: usize, mask: impl mask::MaskTraits);
    fn pop(&mut self, layer: &Self::LayerImpl);
    fn get(&self, index: usize) -> &Self::LayerImpl;
    fn get_id(&mut self, layer: &Self::LayerImpl);
    //fn resize(&mut self, dimension: [usize; 2])
    //fn move(&mut self, pos1, pos2)?
}
