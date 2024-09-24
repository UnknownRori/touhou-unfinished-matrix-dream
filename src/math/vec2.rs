use raylib::prelude::*;

use super::ToComplex;

impl ToComplex for Vector2 {
    fn to_cmpx(&self) -> num_complex::Complex<f32> {
        num_complex::Complex {
            re: self.x,
            im: self.y,
        }
    }
}
