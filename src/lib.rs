pub use minifb::*;
use std::num::*;

// Define a vector2 structure
#[derive(Copy, Clone)]
pub struct Vec2{
    x: i32,
    y: i32
}
//u32
impl Vec2 {
    
    // Create a new vector2
    pub fn new(x: i32, y: i32) -> Self{
        Vec2 { x, y }
    }
    
    // Give the sum of two vector2
    pub fn add(vec1: Vec2, vec2: Vec2) -> Self{
        Vec2 { x: (vec1.x + vec2.x), y: (vec1.y + vec2.y) }
    }

    // Give the substraction of two vector2
    pub fn sub(vec1: Vec2, vec2: Vec2) -> Self{
        Vec2 { x: (vec1.x - vec2.x), y: (vec1.y - vec2.y) }
    }

    // Give the length of 
    pub fn length(vec1: Vec2) -> f32{
        ((vec1.x as f32) + (vec1.y as f32)).sqrt()
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
    TriangleByPoints {point1: Vec2, point_2: Vec2, point_3: Vec2, color:Color},
    TriangleByPosition {position: Vec2, point_1: Vec2, point_2: Vec2, point_3: Vec2, color:Color},
    Cricle {position: Vec2, color:Color},
    Rectangle {position: Vec2, width: u32, height: u32, color:Color}

}

// Define a Canvas structure 
pub struct Canvas {
    width: u32,
    height: u32,
    background_color: Color,
    shapes_list: Vec<Shape>,
    buffer: Vec<u32>

}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color, shapes_list: Vec<Shape>, buffer:Vec<u32> ) -> Self{
        
        Canvas { width, height, background_color, shapes_list, buffer }

    }

    pub fn draw_triangle(&mut self, triangle: Shape) -> Vec<u32>{

        let mut vertex_1: Vec2 = Vec2::new(0, 0);
        let mut vertex_2: Vec2 = Vec2::new(0, 0);
        let mut vertex_3: Vec2 = Vec2::new(0, 0);

        if let Shape::TriangleByPoints { point1, point_2, point_3, color } = triangle {
            vertex_1 = point1;
            vertex_2 = point1;
            vertex_3 = point1;
        }
        else if let Shape::TriangleByPosition { position, point_1, point_2, point_3, color } = triangle  {
            vertex_1 = Vec2::add(position, point_1);
            vertex_2 = Vec2::add(position, point_2);
            vertex_3 = Vec2::add(position, point_3);
            
        }

        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().max().unwrap();
        let rtzr_area_min_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().min().unwrap();
        let rtzr_area_max_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().max().unwrap();
        let rtzr_area_min_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().max().unwrap();
        // The 4 points of the rectangle around the triangle
        let rtzr_point_top_left: Vec2 = Vec2::new(rtzr_area_min_x, rtzr_area_max_y);   
        let rtzr_point_top_right: Vec2 = Vec2::new(rtzr_area_max_x, rtzr_area_max_y);   
        let rtzr_point_bottom_left: Vec2 = Vec2::new(rtzr_area_min_x, rtzr_area_min_y);   
        let rtzr_point_bottom_right: Vec2 = Vec2::new(rtzr_area_max_x, rtzr_area_min_y);   




        let a: Vec<u32> = vec![0,5];
        a
    }

}