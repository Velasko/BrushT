use std::rc;
use std::collections::HashMap;

// extern crate matrix;
// use matrix::prelude::*;

use crate::traits::colors::color::{ColorTraits, ColorValue};
use crate::traits::canvas::pixel::PixelTraits;

pub trait LayerTraits<P, C, T>
where
	P: PixelTraits<C, T>,
	C: ColorTraits<T>,
	T: ColorValue<T>
{
	fn new(height: u16, width: u16) -> Self;
	fn add_color(&mut self, new_color: C) -> (usize, rc::Weak<C>);
	fn set_pixel_color(&mut self, map: HashMap<C, Vec<[usize; 2]>>);
	fn change_color_value(&mut self, old_color: C, new_color: C);
	// fn resize(&mut self, height: u16, width: u16);

	fn get_pixels(&self) -> &Vec<Vec<P>>;
	fn get_colors(&self) -> &Vec<rc::Rc<C>>;
	fn drop_unused_colors(&mut self);
}
