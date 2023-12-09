#![allow(dead_code, unused)]

mod core;
mod traits;

use crate::traits::canvas::color::{ColorTraits, ColorValue};
use crate::traits::canvas::layer::LayerTraits;
use crate::traits::canvas::pixel::PixelTraits;

use crate::core::canvas::*;

fn main() {
    let mut layer_instance: layer::Layer<pixel::Pixel<color::Color<u8>>> =
        layer::Layer::new([10, 10]);

    let col1 = color::Color::<u8>::new([255, 0, 0, 1]);

    // let inserted = layer_instance.add_color(col1);

    let mat: [[f64; 4]; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [3., 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.],
    ];

    //     let col2 = (&inserted.upgrade().expect("A")).mul(&mat);
    //
    //     println!("{}", col2);
    println!("Cores da camada: {:?}", layer_instance.get_colors());

    let default = color::Color::<u8>::new([0, 0, 0, 0]);
    layer_instance.drop_unused_colors();
    layer_instance.change_color_value(default.clone(), col1.clone());
    println!("Cores da camada: {:?}", layer_instance.get_colors());

    use std::collections::HashMap;
    let map = HashMap::from([(default, vec![[0, 0], [0, 1]])]);
    layer_instance.set_pixel_color(map);
    println!("Cores da camada: {:?}", layer_instance.get_colors());

    let new_layer = layer_instance.clone();

    println!("Hello, world!");
}
