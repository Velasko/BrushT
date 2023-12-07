use std::collections::HashMap;

use super::traits::*;
use pixel::PixelTraits;
use color::ColorTraits;

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

impl mask::MaskTraits for PixelMask
{
	fn new(size: usize) -> Self {
		let size = if size % 2 == 0 { size + 1 } else { size };
		Self {
			mask: vec![vec![ColorMask::default(); size]; size],
			size,
		}
	}

	fn render<L>(&self, layer: &L) -> L where L: layer::LayerTraits {
		let pixels = layer.get_pixels();

		let mut colors = Vec::new();
		let mut indexes: Vec<Vec<[usize; 2]>> = Vec::new();

		for (i, line) in pixels.iter().enumerate() {
			for (j, pixel) in line.iter().enumerate() {

				let mut new_color = L::ColorImpl::default();

				for m in 0..self.size{
					for n in 0..self.size{
						let color = match pixels.get(i - (m/2 + 1)) {
							None => L::ColorImpl::default(),
							Some(line) => match line.get(j - (n/2 + 1)) {
								None => L::ColorImpl::default(),
								Some(pixel) => L::ColorImpl::color_into(pixel.get_color().upgrade()
									.expect("Could not render mask.").as_ref())
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
