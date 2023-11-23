use std::rc;

use crate::traits::colors::{ColorTraits, ColorValue};

pub trait PixelTraits<C, T> :
where
	C: ColorTraits<T>,
	T: ColorValue<T>
{
	fn new(color: rc::Weak<C>) -> Self;

	fn get_color(&self) -> &rc::Weak<C>;
	fn set_color(&mut self, box_color: rc::Weak<C>);

	fn add(&self, other: &Self) -> C
	{
		let this_color = self.get_color().upgrade().expect("");
		let other_color = other.get_color().upgrade().expect("");
		let new_color = this_color.add(&other_color);
		new_color
	}
}
