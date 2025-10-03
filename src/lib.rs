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
#[derive(Copy, Clone)]
pub struct Color{
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Color {r, g, b}
    }

    pub fn encode_color(color:Color) -> u32{
    let r = color.r as u32;
    let g = color.g as u32;
    let b = color.b as u32;

    (r << 16) | (g << 8) | b
}


}

// Define a Shape Enum
pub enum Shape{
    TriangleByPoints {point_1: Vec2, point_2: Vec2, point_3: Vec2, color:Color},
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

        let mut triangle_color: Color = Color{r: 255, g:0, b:220}; 

        if let Shape::TriangleByPoints { point_1, point_2, point_3, color } = triangle {
            vertex_1 = point_1;
            vertex_2 = point_2;
            vertex_3 = point_3;
            triangle_color = color;
        }
        else if let Shape::TriangleByPosition { position, point_1, point_2, point_3, color } = triangle  {
            vertex_1 = Vec2::add(position, point_1);
            vertex_2 = Vec2::add(position, point_2);
            vertex_3 = Vec2::add(position, point_3);
            triangle_color = color;
        }

        // Find the area of rasterization 
        //let rtzr_area_max_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().max().unwrap();
        let rtzr_area_min_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().min().unwrap();
        let rtzr_area_max_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().max().unwrap();
        //let rtzr_area_min_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().max().unwrap();
        // The 2 points of the rectangle around the triangle
        let rtzr_point_top_left: Vec2 = Vec2::new(rtzr_area_min_x, rtzr_area_max_y);   
        //let rtzr_point_bottom_right: Vec2 = Vec2::new(rtzr_area_max_x, rtzr_area_min_y);   
        //Check every pixel in the rectangle
        for i in 0..(self.width*self.height) as usize{
            let mut cursor: Vec2 = rtzr_point_top_left;
            // The 3 vector of the triangle
            let segment_ab: f32 = Vec2::length(Vec2::sub(vertex_1,vertex_2));
            let segment_bc: f32 = Vec2::length(Vec2::sub(vertex_2,vertex_3));
            let segment_ca: f32 = Vec2::length(Vec2::sub(vertex_3,vertex_1));
            // The 3 vector between the current point and the 3 vertex
            let segment_h: f32 = Vec2::length(Vec2::sub(cursor,vertex_1));
            let segment_i: f32 = Vec2::length(Vec2::sub(cursor,vertex_2));
            let segment_g: f32 = Vec2::length(Vec2::sub(cursor,vertex_3));
            // Get the 3 angle of the triangle relative to the cursor position, if > 180, the cursor is outside the triangle
            let angle_a:f32 = ((segment_h.powf(2.0)+segment_ab.powf(2.0)-segment_i.powf(2.0))/(2.0*segment_h*segment_ab)).acos() + 
            ((segment_ca.powf(2.0)+segment_h.powf(2.0)-segment_g.powf(2.0))/(2.0*segment_ca*segment_h));

            let angle_b:f32 = ((segment_i.powf(2.0)+segment_ab.powf(2.0)-segment_h.powf(2.0))/(2.0*segment_i*segment_ab)).acos() + 
            ((segment_bc.powf(2.0)+segment_i.powf(2.0)-segment_g.powf(2.0))/(2.0*segment_bc*segment_i));

            let angle_c:f32 = ((segment_g.powf(2.0)+segment_bc.powf(2.0)-segment_i.powf(2.0))/(2.0*segment_g*segment_bc)).acos() + 
            ((segment_ca.powf(2.0)+segment_g.powf(2.0)-segment_h.powf(2.0))/(2.0*segment_ca*segment_g));

            let sum_angles: f32 = angle_a + angle_b + angle_c;

            if sum_angles.floor() <= 180.0 {
                self.buffer[i] = Color::encode_color(triangle_color);
            }

            cursor.x += 1;
            if cursor.x == self.width as i32 {
                cursor.x = rtzr_point_top_left.x;
                cursor.y += 1;
            }

        }




        let a: Vec<u32> = vec![0,5];
        a
    }

}