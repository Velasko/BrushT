use std::rc;
use std::collections::HashMap;

use super::traits::*;

#[derive(Clone)]
pub struct Layer<P, C>
{
	colors_users: Vec<Vec<[usize; 2]>>,
	colors: Vec<rc::Rc<C>>,
	pixels: Vec<Vec<P>>,
	dimensions: [usize; 2],
}

impl<P, C> layer::LayerTraits<P, C> for Layer<P, C>
where
	P: pixel::PixelTraits,
	C: color::ColorTraits,
{
	type PixelImpl = P;
	type ColorImpl = C;

	fn new(dimensions: [usize; 2]) -> Self {
		let mut this = Self {
			colors_users: Vec::new(),
			colors: Vec::new(),
			pixels: Vec::new(),
			dimensions
		};
		let (_, default_color) = this.add_color(C::default());
		this.pixels = vec![
			vec![P::new(default_color.clone()); dimensions[1]];
			dimensions[0]
		];
		this
	}

	fn add_color(&mut self, new_color: C) -> (usize, rc::Weak<C>) {
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

	fn set_pixel_color(&mut self, map: HashMap<C, Vec<[usize; 2]>>) {
		for (mapped_color, pixel_indexes) in map.into_iter(){
			let (_, color) = self.add_color(mapped_color);
			for pixel_index in pixel_indexes {
				self.pixels[pixel_index[0]][pixel_index[1]].set_color(color.clone());
			}
		}

		self.drop_unused_colors();
	}

	fn change_color_value(&mut self, old_color: C, new_color: C) {
		let box_old_color = rc::Rc::new(old_color);
		if let Ok(pos) = self.colors.binary_search(&box_old_color){
			let safe_color_ptr = rc::Rc::as_ptr(&self.colors[pos]);
			let unsafe_color_ptr = safe_color_ptr.cast_mut();
			unsafe { unsafe_color_ptr.write(new_color) };
			// color.set_color_value(*new_color.get_values())
		}
	}

	fn get_dimensions(&self) -> &[usize; 2] {
		&self.dimensions
	}

	fn resize(&self, height: usize, width: usize) -> Self {
		unimplemented!("yet too make resize");
	}

	fn get_pixels(&self) -> &Vec<Vec<P>> {
		&self.pixels
	}

	fn get_colors(&self) -> &Vec<rc::Rc<C>> {
		&self.colors
	}

	fn drop_unused_colors(&mut self) {
		self.colors.retain(|color| rc::Rc::weak_count(color) > 1);
	}
}
