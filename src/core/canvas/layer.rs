use std::rc;
use std::collections::HashMap;

use super::traits::*;

#[derive(Clone)]
pub struct Layer<P>
where
	P: pixel::PixelTraits,
{
	colors_users: Vec<Vec<[usize; 2]>>,
	colors: Vec<rc::Rc<P::ColorImpl>>,
	pixels: Vec<Vec<P>>,
	dimensions: [usize; 2],
}

impl<P> layer::LayerTraits for Layer<P>
where
	P: pixel::PixelTraits,
{
	type PixelImpl = P;
	type ColorImpl = P::ColorImpl;

	fn new(dimensions: [usize; 2]) -> Self {
		let mut this = Self {
			colors_users: Vec::new(),
			colors: Vec::new(),
			pixels: Vec::new(),
			dimensions
		};
		let (_, default_color) = this.add_color(Self::ColorImpl::default());
		this.pixels = vec![
			vec![Self::PixelImpl::new(default_color.clone()); dimensions[1]];
			dimensions[0]
		];
		this
	}

	fn add_color(&mut self, new_color: Self::ColorImpl) -> (usize, rc::Weak<Self::ColorImpl>) {
		let box_color = rc::Rc::new(new_color);
		let position = match self.colors.binary_search(&box_color) {
			Ok(pos) => pos,
			Err(pos) => {
				self.colors.insert(pos, box_color);
				pos
			}
		};
		(position, rc::Rc::downgrade(&self.colors[position]))
	}

	fn set_pixel_color(&mut self, map: HashMap<Self::ColorImpl, Vec<[usize; 2]>>) {
		for (mapped_color, pixel_indexes) in map.into_iter(){
			let (_, color) = self.add_color(mapped_color);
			for pixel_index in pixel_indexes {
				self.pixels[pixel_index[0]][pixel_index[1]].set_color(color.clone());
			}
		}

		self.drop_unused_colors();
	}

	fn change_color_value(&mut self, old_color: Self::ColorImpl, new_color: Self::ColorImpl) {
		let box_old_color = rc::Rc::new(old_color);
		if let Ok(pos) = self.colors.binary_search(&box_old_color){
			let safe_color_ptr = rc::Rc::as_ptr(&self.colors[pos]);
			let unsafe_color_ptr = safe_color_ptr.cast_mut();
			unsafe { unsafe_color_ptr.write(new_color) };
			// color.set_color_value(*new_color.get_values())
		}
	}

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

	fn get_dimensions(&self) -> &[usize; 2] {
		&self.dimensions
	}

	fn resize(&self, height: usize, width: usize) -> Self {
		unimplemented!("yet too make resize");
	}

	fn get_pixels(&self) -> &Vec<Vec<Self::PixelImpl>> {
		&self.pixels
	}

	fn get_colors(&self) -> &Vec<rc::Rc<Self::ColorImpl>> {
		&self.colors
	}

	fn drop_unused_colors(&mut self) {
		self.colors.retain(|color| rc::Rc::weak_count(color) > 1);
	}
}
