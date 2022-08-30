/// See [IBM CGA specification](http://minuszerodegrees.net/oa/OA%20-%20IBM%20Color%20Graphics%20Monitor%20Adapter%20(CGA).pdf)
pub mod buffer;
mod characters;
mod color;
pub mod cursor;
pub mod writer;

pub use color::{Color, ColorCode};
