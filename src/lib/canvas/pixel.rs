use std::rc;

use crate::traits::colors::color::{ColorTraits, ColorValue};
use crate::traits::canvas::pixel::PixelTraits;

#[derive(Debug)]
pub struct Pixel<C>
{
	color: rc::Weak<C>
}

impl<C, T> PixelTraits<C, T> for Pixel<C>
where
	C: ColorTraits<T>,
	T: ColorValue<T>
{
	fn new(box_color: rc::Weak<C>) -> Self {
		Self {
			color: box_color
		}
	}

	fn get_color(&self) -> &rc::Weak<C> {
		&self.color
	}

	fn set_color(&mut self, box_color: rc::Weak<C>) {
		self.color = box_color;
	}
}
