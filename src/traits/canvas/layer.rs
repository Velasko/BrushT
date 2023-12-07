use std::rc;
use std::collections::HashMap;

use super::pixel::*;
use super::color::*;

pub trait LayerTraits:
	std::clone::Clone
{
	type PixelImpl: PixelTraits;
	type ColorImpl: ColorTraits;

	fn new(dimensions: [usize; 2]) -> Self;
	fn add_color(&mut self, new_color: Self::ColorImpl) -> (usize, rc::Weak<Self::ColorImpl>);
	fn set_pixel_color(&mut self, map: HashMap<Self::ColorImpl, Vec<[usize; 2]>>);
	fn change_color_value(&mut self, old_color: Self::ColorImpl, new_color: Self::ColorImpl);

	fn get_dimensions(&self) -> &[usize; 2];
	fn resize(&self, height: usize, width: usize) -> Self;

	fn get_pixels(&self) -> &Vec<Vec<Self::PixelImpl>>;
	fn get_colors(&self) -> &Vec<rc::Rc<Self::ColorImpl>>;
	fn drop_unused_colors(&mut self);

	fn add(&self, other: &Self) -> Self;
}
