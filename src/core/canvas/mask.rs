use std::collections::HashMap;

use super::traits::*;

#[derive(Default, Clone)]
pub struct ColorMask
{
	mask: [[f64; 4]; 4],
}

pub struct PixelMask
{
	mask: Vec<Vec<ColorMask>>,
	size: usize
}

impl<L, P, C, T> mask::MaskTraits<L, P, C, T> for PixelMask
where
	L: layer::LayerTraits<P, C, T>,
	P: pixel::PixelTraits<C, T>,
	C: color::ColorTraits<T>,
	T: color::ColorValue<T>,
{
	fn new(size: usize) -> Self {
		let size = if size % 2 == 0 { size + 1 } else { size };
		Self {
			mask: vec![vec![ColorMask::default(); size]; size],
			size,
		}
	}

	fn render(&self, layer: &L) -> L {
		let pixels = layer.get_pixels();

		let mut colors = Vec::new();
		let mut indexes: Vec<Vec<[usize; 2]>> = Vec::new();

		for (i, line) in pixels.iter().enumerate() {
			for (j, pixel) in line.iter().enumerate() {

				let mut new_color = C::default();

				for m in 0..self.size{
					for n in 0..self.size{
						let color = match pixels.get(i - (m/2 + 1)) {
							None => C::default(),
							Some(line) => match line.get(j - (n/2 + 1)) {
								None => C::default(),
								Some(pixel) => pixel.get_color().upgrade()
									.expect("Could not render mask.").as_ref().clone()
							}
						};
						let masked_color = color.mul_matrix(&self.mask[m][n].mask);
						new_color = new_color.add(&masked_color);
					}
				}

				match colors.binary_search(&new_color) {
					Ok(pos) => { indexes[pos].push([i, j] as [usize; 2]) },
					Err(pos) => {
						colors.insert(pos, new_color);
						indexes.insert(pos, vec![[i, j] as [usize; 2]]);
					}
				};
			}
		}

		let mut map = HashMap::new();
		for (color, index) in colors.into_iter().zip(indexes.into_iter()) {
			map.insert(color, index);
		}

		let mut new_layer = layer.clone();
		new_layer.set_pixel_color(map);
		new_layer
	}
}
