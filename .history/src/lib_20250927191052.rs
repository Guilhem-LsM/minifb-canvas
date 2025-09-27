pub use minifb::*;

// Define a vector2 structure
pub struct Vec2{
    x: u32,
    y: u32
}

impl Vec2 {
    
    pub fn new(x: u32, y: u32) -> Self{
        Vec2 { x, y }
    }

}

// Define a color structure
pub struct Color{
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Color {r, g, b}
    }

}

// Define a Shape Enum
pub enum Shape{
    Triangle {point1: Vec2, point2: Vec2, point3: Vec2, color:Color, layer_pos:u32},
    Cricle {position: Vec2, color:Color, layer_pos:u32},
    Rectangle {position: u32, width: u32, height: u32, color:Color, layer_pos:u32}

}

pub struct Canvas {
    width: u32,
    height: u32,
    background_color: Color,
    shapes_list: Vec<Shape>,

}