use super::traits::*;

pub struct ColorMask
{
	mask: [[f64; 4]; 4],
}

pub struct PixelMask
{
	mask: Vec<Vec<ColorMask>>
}

// impl<L, P, C, T> mask::MaskTraits<L, P, C, T> for PixelMask
// where
// 	L: layer::LayerTraits<P, C, T>,
// 	P: pixel::PixelTraits<C, T>,
// 	C: color::ColorTraits<T>,
// 	T: color::ColorValue<T>,
// {
// 	fn new(matrix: [[f64; 4]; 4]) -> Self {
// 		Self {
// 			mask: matrix,
// 		}
// 	}
//
// 	fn render(&self, layer: &L) -> L {
// 		let mut new_layer = layer.clone();
// 		new_layer
//
// 		for line in layer.get_pixels().iter() {
// 			for pixel in line.iter()
//
// 		}
// 	}
// }
