mod traits;
mod lib;

use std::rc::Rc;

use std::mem;

use crate::traits::colors::color::{ColorTraits, ColorValue};
use crate::traits::canvas::pixel::PixelTraits;
use crate::traits::canvas::layer::LayerTraits;

use crate::lib::{colors,canvas};

fn main() {

    let mut layer:
        canvas::Layer<canvas::Pixel<colors::Color<u8>>,
        colors::Color<u8>> = canvas::Layer::new(10, 10);

    let col1 = colors::Color::<u8>::new([255, 0, 0, 1]);

    println!("{}", std::mem::size_of::<
            canvas::Pixel<
                colors::Color<u8>
            >
    >());
    // let inserted = layer.add_color(col1);

    let mat: [[f64; 4]; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [3. , 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1. ],
    ];

//     let col2 = (&inserted.upgrade().expect("A")).mul(&mat);
//
//     println!("{}", col2);
    println!("Cores da camada: {:?}", layer.get_colors());


    let default = colors::Color::<u8>::new([0, 0, 0, 0]);
    layer.drop_unused_colors();
    layer.change_color_value(default, col1);
    println!("Cores da camada: {:?}", layer.get_colors());


    println!("Hello, world!");
}
