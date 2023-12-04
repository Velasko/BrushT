use super::traits::*;

pub enum Filter {
	// ???
}

pub struct Mask<R>
{
	mask: [[f64; 4]; 4],
	data: R,
	filter: Filter
}

impl<R, L, P, C, T> mask::MaskTraits<L, P, C, T> for Mask<R>
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>,
{
	fn new(matrix: [[f64; 4]; 4]) -> Self {
		Mask {
			data: matrix
		}
	}

	fn extract(&self) -> &[[f64; 4]; 4] {
		&self.data
	}
}
