use std::rc;

use super::traits::color::{ColorTraits, ColorValue};
use super::traits::pixel::PixelTraits;

#[derive(Debug)]
#[derive(Clone)]
pub struct Pixel<C>
{
	color: rc::Weak<C>
}

impl<C> PixelTraits for Pixel<C>
where
	C: ColorTraits,
{
	type ColorImpl = C;
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
