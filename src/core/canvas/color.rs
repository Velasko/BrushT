use std::{cmp, fmt};

use super::traits::color::*;

#[derive(Debug)]
#[derive(Hash)]
#[derive(Default)]
#[derive(cmp::Ord)]
#[derive(cmp::Eq)]
#[derive(cmp::PartialOrd)]
#[derive(cmp::PartialEq)]
pub struct Color<T>
where T: ColorValue<T>
{
    values: [T; 4],
}

impl<T> std::clone::Clone for Color<T>
where T: ColorValue<T>
{
    fn clone(&self) -> Self {
        Self::new(self.get_values().clone())
    }
}

impl<T> fmt::Display for Color<T>
where T: ColorValue<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result where Self : Sized{
		let [r, g, b, a] = self.get_values();
		let type_name = T::get_size() * 8;
		write!(f, "Color<u{}>: [{}, {}, {}, {}]", type_name, r, g, b, a)
	}
}

impl<T> ColorTraits<T> for Color<T>
where T: ColorValue<T>
{
    fn new(color: [T; 4]) -> Self {
        Color {
            values: color,
        }
    }

    fn get_values(&self) -> &[T; 4] {
        &self.values
    }

	fn set_color_value(&mut self, new_value: [T; 4]) {
	    self.values = new_value;
	}
}

impl<T> std::ops::Mul<&[[f64; 4]; 4]> for Color<T>
where
    T: ColorValue<T>
{
    type Output = Self;

    fn mul(self, other: &[[f64; 4]; 4]) -> Self {
        ColorTraits::<T>::mul_matrix(&self, other)
    }
}
