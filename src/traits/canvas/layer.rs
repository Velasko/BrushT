use std::rc;
use std::collections::HashMap;

use super::color::{ColorTraits, ColorValue};
use super::pixel::PixelTraits;

pub trait LayerTraits<P, C, T>:
	std::clone::Clone
where
	P: PixelTraits<C, T>,
	C: ColorTraits<T>,
	T: ColorValue<T>
{
	fn new(height: usize, width: usize) -> Self;
	fn add_color(&mut self, new_color: C) -> (usize, rc::Weak<C>);
	fn set_pixel_color(&mut self, map: HashMap<C, Vec<[usize; 2]>>);
	fn change_color_value(&mut self, old_color: C, new_color: C);

	fn get_dimensions(&self) -> [usize; 2];
	fn resize(&self, height: usize, width: usize) -> Self;

	fn get_pixels(&self) -> &Vec<Vec<P>>;
	fn get_colors(&self) -> &Vec<rc::Rc<C>>;
	fn drop_unused_colors(&mut self);

	fn add(&self, other: &Self) -> Self
	{
		let (height, width) = self.get_dimensions().into();
		let other_pixels = other.get_pixels();
		let mut color_vec: Vec<C> = Vec::new();
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

		let mut color_map: HashMap<C, Vec<[usize; 2]>> = HashMap::new();
		for (color, indexes) in color_vec.into_iter().zip(indexes_vec.into_iter()){
			color_map.insert(color, indexes);
		}
		color_map.shrink_to_fit();

		let mut new_layer = Self::new(height, width);
		new_layer.set_pixel_color(color_map);
		new_layer
	}
}
