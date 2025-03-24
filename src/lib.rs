mod orientations;
mod scales;
mod sizes;

pub use orientations::Orientation;
pub use scales::Scale;
pub use sizes::Size;

pub fn new_thumbnail(sz: i32, scale: Scale) -> Size {
    Size::new_thumbnail(sz, scale)
}

pub fn new_landscape(w: i32, h:i32, scale: Scale) -> Size {
    Size::new_landscape(w, h, scale)
}

pub fn new_portrait(w: i32, h: i32, scale: Scale) -> Size {
    Size::new_portrait(w, h, scale)
}





