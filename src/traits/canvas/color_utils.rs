use super::color::ColorValue;

macro_rules! iterate_array {
    ($func:expr, $variable:expr) => {
        [
            $func(&$variable[0]),
            $func(&$variable[1]),
            $func(&$variable[2]),
            $func(&$variable[3])
        ]
    }
}

fn f64_to_u128(item: &f64) -> u128 {
    ( item * f64::exp2(64.0) ) as u128
}

fn line_f64_to_u128(line: &[f64; 4]) -> [u128; 4] {
    iterate_array!(f64_to_u128, line)
}

pub fn matrix_f64_to_u128(matrix: &[[f64; 4]; 4]) -> [[u128; 4]; 4] {
    iterate_array!(line_f64_to_u128, matrix)
}

fn dot_product<T>(vector_a: &[T; 4], vector_b: &[u128; 4]) -> T
where
    T: ColorValue<T>
{
    let value: u128 = vector_a.iter().zip(vector_b.iter()).map(
        |(x, y)| (<T as Into<u128>>::into(*x)).saturating_mul(*y) >> 64 ).sum();
    T::from_u128_saturated(value)
}

pub fn identity_matrix(value: f64) -> [[f64; 4]; 4] {
    [
        [value, 0., 0., 0.],
        [0., value, 0., 0.],
        [0., 0., value, 0.],
        [0., 0., 0., value]
    ]
}

pub fn matrix_product<T>(matrix: &[[u128; 4]; 4], vector: &[T; 4]) -> [T; 4]
where
    T: ColorValue<T>
{
    let color_vec = matrix.iter().map(
        | line | dot_product(&vector, line)
    ).collect::<Vec<T>>();
    match color_vec.try_into() {
        Ok(color) => color,
        Err(_) => unreachable!()
    }
}
