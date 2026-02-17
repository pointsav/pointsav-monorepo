#![no_std]

pub mod geometry;

use system_core::{PointSavResult, Substrate};

/// The root trait for a display surface (Rasterizer/Layout).
pub trait Surface: Substrate {
    fn dimensions(&self) -> geometry::Size;
    fn clear(&mut self) -> PointSavResult<()>;
}
