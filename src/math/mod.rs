mod complex;
mod vec2;

pub use complex::ComplexExt;
use num_complex::Complex;
use raylib::prelude::*;

#[macro_export]
macro_rules! cmpx {
    ($x:expr) => {
        num_complex::Complex::new($x, $x)
    };
    ($re:expr, $im:expr) => {
        num_complex::Complex::new($re, $im)
    };
    (($re:expr, $im:expr)) => {
        num_complex::Complex::new($re, $im)
    };
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr) => {
        raylib::math::Vector2::new($x, $x)
    };
    ($x:expr, $y:expr) => {
        raylib::math::Vector2::new($x, $y)
    };
}

#[macro_export]
macro_rules! rect {
    ($xy:expr, $size:expr) => {
        raylib::math::Rectangle::new($xy, $xy, $size, $size)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr) => {
        raylib::math::Rectangle::new($x, $y, $width, $height)
    };
    (($re:expr, $im:expr)) => {
        num_complex::Complex::new($re, $im)
    };
}

pub trait CartesianExt {
    type Output;

    fn x(&self) -> &Self::Output;
    fn y(&self) -> &Self::Output;
}

pub trait ToVec2 {
    fn to_vec2(&self) -> Vector2;
}

pub trait ToComplex {
    fn to_cmpx(&self) -> Complex<f32>;
}
