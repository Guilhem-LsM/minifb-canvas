pub use minifb::*;

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
    pub fn length(&self) -> f32{
        ((self.x as f32).powf(2.0) + (self.y as f32).powf(2.0)).sqrt()
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

    pub fn to_u32(&self) -> u32{
    let r = self.r as u32;
    let g = self.g as u32;
    let b = self.b as u32;

    (r << 16) | (g << 8) | b
}


}

// Define a Shape Enum
#[derive(Clone)]
pub enum Shape{
    TriangleByPoints {point_1: Vec2, point_2: Vec2, point_3: Vec2, color:Color},
    TriangleByPosition {position: Vec2, point_1: Vec2, point_2: Vec2, point_3: Vec2, color:Color},
    Circle {position: Vec2, color:Color},
    Rectangle {position: Vec2, width: u32, height: u32, color:Color}

}

// Define a Canvas structure 
pub struct Canvas {
    width: u32,
    height: u32,
    background_color: Color,
    shapes_list: Vec<Shape>,
    frame_buffer: Vec<u32>

}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self{
        let frame_buffer: Vec<u32> = vec![background_color.to_u32(); (width*height) as usize];
        Canvas { width, height, background_color, shapes_list: Vec::<Shape>::new(), frame_buffer}

    }

    // Get the 2D cross product by giving the 3 point of the 2 vector
    pub fn _2d_cross_product(pixel_coordinates: Vec2, vertex_1: Vec2, vertex_2: Vec2) -> bool {
        (((vertex_1.x - pixel_coordinates.x)*(vertex_2.y - pixel_coordinates.y)) - ((vertex_1.y - pixel_coordinates.y)*(vertex_2.x - pixel_coordinates.x))) <= 0 
    }
    // Take the 3 vertex of a triangle and a point and calculate if the point is in the triangle
    pub fn is_in_triangle(vertex_1: Vec2, vertex_2: Vec2, vertex_3: Vec2, pixel_coordinates:Vec2) -> bool {
        let a:bool = Self::_2d_cross_product(pixel_coordinates, vertex_1, vertex_2);
        let b:bool = Self::_2d_cross_product(pixel_coordinates, vertex_2, vertex_3);
        let c:bool = Self::_2d_cross_product(pixel_coordinates, vertex_3, vertex_1);
        (a, b, c) == (true, true, true)
    }   
    // Draw a triangle in the canvas's frame buffer with the 3 vertex coordinates and the color
    pub fn draw_triangle(&mut self, vertex_1: Vec2, vertex_2: Vec2, vertex_3: Vec2, color:Color ){

        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().max().unwrap();
        let rtzr_area_min_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().min().unwrap();
        let rtzr_area_max_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().max().unwrap();
        let rtzr_area_min_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().min().unwrap();

        //Check for every pixel if it is in the triangle. If yes, color it in the color specified
        for y in rtzr_area_min_y..rtzr_area_max_y{
            for x in rtzr_area_min_x..rtzr_area_max_x{
                let cursor: Vec2 = Vec2::new(x, y);
                if Self::is_in_triangle(vertex_1, vertex_2, vertex_3, cursor) {
                self.frame_buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = color.to_u32();
                 }
            }
        }
    }

    pub fn draw_circle(&mut self, position: Vec2, rayon: u32, color: Color){
        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = position.x + rayon as i32;
        let rtzr_area_min_x:i32 = position.x - rayon as i32;
        let rtzr_area_max_y:i32 = position.y + rayon as i32;
        let rtzr_area_min_y:i32 = position.y - rayon as i32;

        //Check for every pixel if it is in the circle. If yes, color it in the color specified
        for y in rtzr_area_min_y..rtzr_area_max_y{
            for x in rtzr_area_min_x..rtzr_area_max_x{
                let cursor: Vec2 = Vec2::new(x, y);
                if Vec2::sub(cursor, position).length() <= rayon as f32{
                    self.frame_buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = color.to_u32();
                }
            }
        }
    }

    pub fn draw_rectangle(&mut self, position: Vec2, _width: u32, height: u32, color: Color){
        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = position.x + (_width/2) as i32;
        let rtzr_area_min_x:i32 = position.x - (_width/2) as i32;
        let rtzr_area_max_y:i32 = position.y + (height/2) as i32;
        let rtzr_area_min_y:i32 = position.y - (height/2) as i32;

        let mut cursor: Vec2 = Vec2::new(0, 0);
        //Color all the area
        for y in rtzr_area_min_y..rtzr_area_max_y{
            for x in rtzr_area_min_x..rtzr_area_max_x{
                cursor.x = x; cursor.y = y;
                self.frame_buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = color.to_u32();
            }
        }
    }

    //Clear the frame buffer of the canvas
    pub fn clear_frame_buffer(&mut self){
        self.frame_buffer = vec![self.background_color.to_u32(), self.width*self.height]
    }
    //Gives the frame buffer of the canvas
    pub fn as_buffer(&mut self) -> &Vec<u32> {
        &self.frame_buffer
    }

}
