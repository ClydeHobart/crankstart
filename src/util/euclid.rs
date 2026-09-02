use ::euclid::{Point2D, Rect, Size2D, Vector2D};

pub struct Pixel;

pub type IPxPoint2D = Point2D<i32, Pixel>;
pub type IPxVector2D = Vector2D<i32, Pixel>;
pub type IPxRect = Rect<i32, Pixel>;
pub type IPxSize2D = Size2D<i32, Pixel>;

pub type FPxPoint2D = Point2D<f32, Pixel>;
pub type FPxVector2D = Vector2D<f32, Pixel>;
pub type FPxRect = Rect<f32, Pixel>;
pub type FPxSize2D = Size2D<f32, Pixel>;
