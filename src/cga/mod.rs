#![allow(dead_code)]
/// IBM CGA specification: http://minuszerodegrees.net/oa/OA%20-%20IBM%20Color%20Graphics%20Monitor%20Adapter%20(CGA).pdf
mod buffer;
mod characters;
mod color;
mod writer;

pub use color::{Color, ColorCode};
pub use writer::*;
