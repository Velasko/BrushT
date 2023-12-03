pub use num;
use std::cmp;

use super::color_utils::*;

pub trait ColorValue<T> :
	num::Unsigned
	+ num::Bounded
	// + num::Saturating
	+ num::traits::SaturatingMul
	+ std::ops::Mul<T>
	+ std::iter::Sum
	+ cmp::Eq
	+ std::marker::Copy
	+ std::default::Default
	+ std::fmt::Display
	+ std::fmt::Debug
	+ cmp::Ord
	+ std::hash::Hash
	+ PartialOrd<T>
	+ From<T>
	+ Into<T>
	+ TryFrom<u128>
	+ Into<u128>
	+ num::PrimInt
where
	T: ColorValue<T>
	{
		fn from_u128_saturated(value: u128) -> Self {
			value.try_into().unwrap_or(Self::max_value())
		}

		fn get_size() -> usize {
			std::mem::size_of::<T>()
		}
	}

impl ColorValue<u8> for u8 {}
impl ColorValue<u16> for u16 {}
impl ColorValue<u32> for u32 {}
impl ColorValue<u64> for u64 {}
//impl ColorValue<u128> for u128 {}

pub trait ColorTraits<T> :
	cmp::Ord
	+ for<'a> std::ops::Mul<&'a [[f64; 4]; 4]>
	+ std::hash::Hash
	+ std::clone::Clone
	+ std::default::Default
	+ std::fmt::Debug
where
	T: ColorValue<T>,
{
	fn new(color: [T; 4]) -> Self;
	fn get_values(&self) -> &[T; 4];
	fn set_color_value(&mut self, new_value: [T; 4]);

	fn mul_f64(&self, other: f64) -> Self {
		let matrix: [[f64; 4]; 4] = identity_matrix(other);
		self.mul_matrix(&matrix)
	}

    fn mul_matrix(&self, other: &[[f64; 4]; 4]) -> Self
	where Self: Sized
    {
		let matrix = matrix_f64_to_u128(other);
		let color: [T; 4] = matrix_product(&matrix, self.get_values());
		Self::new(color)
    }

    fn add(&self, other: &Self) -> Self {
		let max: f64 = Into::<u128>::into(T::max_value()) as f64;

		let lhm: f64 = Into::<u128>::into(self.get_values()[3]) as f64 / max;
		let binding = self.mul_f64(lhm);
		let lhc = binding.get_values();

		let rhm: f64 = (1. - lhm) * Into::<u128>::into(other.get_values()[3]) as f64 / max;
		let binding = other.mul_f64(rhm);
		let rhc = binding.get_values();

		let new_color: [T; 4] = lhc.iter().zip(rhc.iter()).map(
			|(l, r)| (*l).saturating_add(*r)
		).collect::<Vec<T>>().try_into().unwrap();
		Self::new(new_color)
    }

	fn eq(&self, other: &Self) -> bool {
		let other_values = other.get_values();
		for (i, &item) in self.get_values().iter().enumerate() {
			if item != other_values[i] {
				return false;
			}
		}
		true
	}

	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		let self_values = self.get_values();
		let other_values = other.get_values();

		for i in 0..4 {
			if self_values[i] != other_values[i] {
				return self_values[i].partial_cmp(&other_values[i])
			}
		}
		self_values[0].partial_cmp(&other_values[0])
	}

	fn get_type_max(&self) -> T {
        T::max_value()
    }

	#[allow(non_snake_case)]
	fn RED() -> Self where Self: Sized {
		Self::new([T::max_value(), T::min_value(), T::min_value(), T::max_value()])
	}

	#[allow(non_snake_case)]
	fn GREEN() -> Self where Self: Sized {
		Self::new([T::min_value(), T::max_value(), T::min_value(), T::max_value()])
	}

	#[allow(non_snake_case)]
	fn BLUE() -> Self where Self: Sized {
		Self::new([T::min_value(), T::min_value(), T::max_value(), T::max_value()])
	}
}
