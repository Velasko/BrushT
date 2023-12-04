use super::traits::*;

pub enum RenderTree<M, L, P, C, T>
where
	M: mask::MaskTraits<L, P, C, T>,
	L: layer::LayerTraits<P, C, T>
{
	Layer(L),
	Mask(M),
	Tree(Self, Self),
}
