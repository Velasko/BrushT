use std::rc;

extern crate matrix;
use matrix::prelude::*;

use crate::traits::colors::color::{ColorTraits, ColorValue};
use crate::traits::canvas::pixel::PixelTraits;
use crate::traits::canvas::layer::LayerTraits;

pub struct Layer<P, C>
{
	colors_users: Vec<Vec<[usize; 2]>>,
	colors: Vec<rc::Rc<C>>,
	pixels: Vec<Vec<P>>,
	width: usize,
	height: usize,
}

impl<P, C, T> LayerTraits<P, C, T> for Layer<P, C>
where
	P: PixelTraits<C, T>,
	C: ColorTraits<T>,
	T: ColorValue<T>,
{
	fn new(height: u16, width: u16) -> Self {
		let width: usize = width.into();
		let height: usize = height.into();

		let mut this = Self {
			colors_users: Vec::new(),
			colors: Vec::new(),
			pixels: Vec::with_capacity(height),
			width,
			height,
		};
		let (_, default_color) = this.add_color(C::default());

		for _ in 0..height {
			let mut line = Vec::with_capacity(width);
			for _ in 0..width {
				line.push(P::new(default_color.clone()));
			}
			this.pixels.push(line);
		}

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

	fn set_pixel_color(&mut self, map: Vec<(C, Vec<(C, Vec<[usize; 2]>)>)>){
		// let (_, color) = self.add_color(new_color);
		// for [i, j] in pixel_coords {
		// 	self.pixels[i][j].set_color(color.clone());
		// }
		// self.drop_unused_colors();
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
