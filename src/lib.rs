//! Rust binding of libharu PDF library.
#![warn(missing_docs)]

use std::fmt::Debug;

use libharu_sys::{HPDF_Point, HPDF_Rect};
mod context;
mod destination;
mod document;
mod encoder;
mod error;
mod image;
mod outline;
mod page;

/// prelude
pub mod prelude;

/// Floating-point type used in libharu.
pub type Real = libharu_sys::HPDF_REAL;

/// A lower-level representation of the rectangle type used in libharu
/// Translates to `/Rect [ xLL yLL xUR yUR ]`
#[derive(Default)]
pub struct Rectangle {
    pub lower_left: Point,
    pub upper_right: Point,
}

enum Corner {
    LowerLeft,
    LowerRight,
    UpperLeft,
    UpperRight,
}

pub struct RectangleBuilder {
    point: Point,
    starting: Corner,
    height: f32,
    width: f32,
}

impl RectangleBuilder {
    /// Define a starting point for the Rectangle
    fn new(point: Point) -> Self {
        Self {
            point: Point::default(),
            starting: Corner::LowerLeft,
            height: 0.0,
            width: 0.0,
        }
    }
    /// Designate the starting corner
    fn starting_corner(&mut self, corner: Corner) -> &mut Self {
        self.starting = corner;
        self
    }

    /// Set the width of the rectangle
    fn set_width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    /// Set the height of the rectangle
    fn set_height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }

    /// Build and return the rectangle
    fn build(self) -> Rectangle {
        match self.starting {
            Corner::LowerLeft => Rectangle {
                lower_left: self.point,
                upper_right: Point {
                    x: self.point.x + self.width,
                    y: self.point.y + self.height,
                },
            },
            Corner::LowerRight => Rectangle {
                lower_left: Point {
                    x: self.point.x - self.width,
                    y: self.point.y,
                },
                upper_right: Point {
                    x: self.point.x,
                    y: self.point.y + self.height,
                },
            },
            Corner::UpperLeft => Rectangle {
                lower_left: Point {
                    x: self.point.x,
                    y: self.point.y - self.height,
                },
                upper_right: Point {
                    x: self.point.x + self.width,
                    y: self.point.y,
                },
            },
            Corner::UpperRight => Rectangle {
                lower_left: Point {
                    x: self.point.x - self.width,
                    y: self.point.y - self.height,
                },
                upper_right: self.point,
            },
        }
    }
}

impl From<Rectangle> for HPDF_Rect {
    fn from(value: Rectangle) -> Self {
        Self {
            left: value.lower_left.x,
            bottom: value.lower_left.y,
            right: value.upper_right.x,
            top: value.upper_right.y,
        }
    }
}

/// RGB color type.
#[derive(Debug, Clone)]
pub struct Color {
    /// red (0.0 ~ 1.0)
    pub red: Real,

    /// green (0.0 ~ 1.0)
    pub green: Real,

    /// blue (0.0 ~ 1.0)
    pub blue: Real,
}

impl Copy for Color {}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0
        }
    }
}

impl From<(Real, Real, Real)> for Color {
    fn from(v: (Real, Real, Real)) -> Self {
        Self {
            red: v.0,
            green: v.1,
            blue: v.2,
        }
    }
}

/// CMYK color type
#[derive(Debug, Clone)]
pub struct CmykColor {
    /// cyan (0.0 ~ 1.0)
    pub cyan: Real,

    /// magenta (0.0 ~ 1.0)
    pub magenta: Real,

    /// yellow (0.0 ~ 1.0)
    pub yellow: Real,

    /// keyplate (0.0 ~ 1.0)
    pub keyplate: Real,
}

impl Copy for CmykColor {}

impl From<(Real, Real, Real, Real)> for CmykColor {
    fn from(v: (Real, Real, Real, Real)) -> Self {
        Self {
            cyan: v.0,
            magenta: v.1,
            yellow: v.2,
            keyplate: v.3,
        }
    }
}
/// Point
#[derive(Debug, Clone, Default)]
pub struct Point {
    /// x
    pub x: Real,

    /// y
    pub y: Real,
}

impl From<HPDF_Point> for Point {
    fn from(v: HPDF_Point) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl Copy for Point {}

impl From<(Real, Real)> for Point {
    fn from(v: (Real, Real)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl Point {
    pub fn set<P>(&mut self, point: P) where P: Into<Point> {
        let point = point.into();
        self.x = point.x;
        self.y = point.y;
    }
}

/// Rect
#[derive(Debug, Clone)]
pub struct Rect {
    /// Left position
    pub left: Real,

    /// Top position
    pub top: Real,

    /// Right position
    pub right: Real,

    /// Bottom position
    pub bottom: Real,
}

impl From<HPDF_Rect> for Rect {
    fn from(v: HPDF_Rect) -> Self {
        Self {
            left: v.left,
            top: v.top,
            right: v.right,
            bottom: v.bottom,
        }
    }
}

impl Copy for Rect {}

impl From<(Real, Real, Real, Real)> for Rect {
    fn from(v: (Real, Real, Real, Real)) -> Self {
        Self {
            left: v.0,
            top: v.1,
            right: v.2,
            bottom: v.3,
        }
    }
}

impl From<(Real, Real)> for Rect {
    fn from(v: (Real, Real)) -> Self {
        Self {
            left: v.0,
            top: v.1,
            right: v.0,
            bottom: v.1,
        }
    }
}

/// Font handle type.
pub struct Font<'a> {
    font: libharu_sys::HPDF_Font,
    _doc: &'a prelude::Document,
}

impl<'a> Font<'a> {
    pub(crate) fn new(_doc: &'a prelude::Document, font: libharu_sys::HPDF_Font) -> Self {
        Self { font, _doc }
    }

    #[inline]
    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Font {
        self.font
    }

    /// Get the name of the font.
    pub fn name(&self) -> anyhow::Result<&str> {
        unsafe {
            let name = libharu_sys::HPDF_Font_GetFontName(self.handle());

            let s = std::ffi::CStr::from_ptr(name).to_str()?;

            Ok(s)
        }
    }
}

impl<'a> Debug for Font<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font").field("font", &self.font).finish()
    }
}
