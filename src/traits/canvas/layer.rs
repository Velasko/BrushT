use std::rc;
use std::collections::HashMap;

use super::*;

pub trait LayerTraits<P, C>:
	std::clone::Clone
{
	type PixelImpl: pixel::PixelTraits;
	type ColorImpl: color::ColorTraits;

	fn new(dimensions: [usize; 2]) -> Self;
	fn add_color(&mut self, new_color: Self::ColorImpl) -> (usize, rc::Weak<Self::ColorImpl>);
	fn set_pixel_color(&mut self, map: HashMap<Self::ColorImpl, Vec<[usize; 2]>>);
	fn change_color_value(&mut self, old_color: Self::ColorImpl, new_color: Self::ColorImpl);

	fn get_dimensions(&self) -> &[usize; 2];
	fn resize(&self, height: usize, width: usize) -> Self;

	fn get_pixels(&self) -> &Vec<Vec<Self::PixelImpl>>;
	fn get_colors(&self) -> &Vec<rc::Rc<Self::ColorImpl>>;
	fn drop_unused_colors(&mut self);

	fn add(&self, other: &Self) -> Self
	{
		let dimension = self.get_dimensions();
		let other_pixels = other.get_pixels();
		let mut color_vec: Vec<Self::ColorImpl> = Vec::new();
		let mut indexes_vec: Vec<Vec<[usize; 2]>> = Vec::new();

		for (i, self_line) in self.get_pixels().iter().enumerate() {
			for (j, self_pixel) in self_line.iter().enumerate(){
				let color = self_pixel.blend(&other_pixels[i][j]);
				match color_vec.binary_search(&color) {
					Ok(pos) => indexes_vec[pos].push([i, j]),
					Err(pos) => {
						color_vec.insert(pos, color);
						indexes_vec.insert(pos, vec![[i, j]]);
					}
				}
			}
		}

		let mut color_map: HashMap<Self::ColorImpl, Vec<[usize; 2]>> = HashMap::new();
		for (color, indexes) in color_vec.into_iter().zip(indexes_vec.into_iter()){
			color_map.insert(color, indexes);
		}
		color_map.shrink_to_fit();

		let mut new_layer = Self::new(dimension.clone());
		new_layer.set_pixel_color(color_map);
		new_layer
	}
}
