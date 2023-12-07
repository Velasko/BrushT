use std::rc;

use super::color::*;

pub trait PixelTraits:
	std::clone::Clone
{
	type ColorImpl: ColorTraits;

	fn new(color: rc::Weak<Self::ColorImpl>) -> Self;

	fn get_color(&self) -> &rc::Weak<Self::ColorImpl>;
	fn set_color(&mut self, box_color: rc::Weak<Self::ColorImpl>);

	fn blend(&self, other: &Self) -> Self::ColorImpl {
		let this_color = self.get_color().upgrade().expect("");
		let other_color = other.get_color().upgrade().expect("");
		let new_color = this_color.add(&other_color);
		new_color
	}
}
