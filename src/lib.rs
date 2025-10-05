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
    buffer: Vec<u32>

}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color, shapes_list: Vec<Shape>) -> Self{
        let buffer: Vec<u32> = vec![background_color.to_u32(); (width*height) as usize];
        Canvas { width, height, background_color, shapes_list, buffer}

    }

    // Get the 2D cross product by giving the 3 point of the 2 vector
    pub fn _2d_cross_product(point_1: Vec2, point_2: Vec2, point_3: Vec2) -> bool {
        (((point_2.x - point_1.x)*(point_3.y - point_1.y)) - ((point_2.y - point_1.y)*(point_3.x - point_1.x))) <= 0 
    }

    pub fn is_in_triangle(point_1: Vec2, point_2: Vec2, point_3: Vec2, point_4:Vec2) -> bool {
        let a:bool = Self::_2d_cross_product(point_4, point_1, point_2);
        let b:bool = Self::_2d_cross_product(point_4, point_2, point_3);
        let c:bool = Self::_2d_cross_product(point_4, point_3, point_1);
        (a, b, c) == (true, true, true)
    }   

    pub fn draw_triangle(&mut self, triangle: &Shape) -> Vec<u32>{
        let mut vertex_1: Vec2 = Vec2::new(0, 0);
        let mut vertex_2: Vec2 = Vec2::new(0, 0);
        let mut vertex_3: Vec2 = Vec2::new(0, 0);

        let mut triangle_color: Color = Color{r: 255, g:0, b:220}; 

        if let Shape::TriangleByPoints { point_1, point_2, point_3, color } = triangle {
            vertex_1 = *point_1;
            vertex_2 = *point_2;
            vertex_3 = *point_3;
            triangle_color = *color;
        }
        else if let Shape::TriangleByPosition { position, point_1, point_2, point_3, color } = triangle  {
            vertex_1 = Vec2::add(*position, *point_1);
            vertex_2 = Vec2::add(*position, *point_2);
            vertex_3 = Vec2::add(*position, *point_3);
            triangle_color = *color;
            
        }

        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().max().unwrap();
        let rtzr_area_min_x:i32 = *(vec![vertex_1.x, vertex_2.x, vertex_3.x]).iter().min().unwrap();
        let rtzr_area_max_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().max().unwrap();
        let rtzr_area_min_y:i32 = *(vec![vertex_1.y, vertex_2.y, vertex_3.y]).iter().min().unwrap();
        // The 2 points of the rectangle around the triangle
        let rtzr_point_top_left: Vec2 = Vec2::new(rtzr_area_min_x, rtzr_area_max_y);   
        let rtzr_point_bottom_right: Vec2 = Vec2::new(rtzr_area_max_x, rtzr_area_min_y);   
        //Check every pixel in the rectangle
        let mut cursor: Vec2 = rtzr_point_top_left;
        for i in 0..((rtzr_area_max_x-rtzr_area_min_x)*(rtzr_area_max_y-rtzr_area_min_y)) as usize{
            
            if Self::is_in_triangle(vertex_1, vertex_2, vertex_3, cursor) {
                self.buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = triangle_color.to_u32();
            }

            cursor.x += 1;
            if cursor.x == rtzr_area_max_x{
                cursor.x = rtzr_point_top_left.x;
                cursor.y -= 1;
            }

        }

        let a: Vec<u32> = vec![0,5];
        a
    }

    pub fn as_buffer(&mut self) -> &Vec<u32> {

        for shape in &self.shapes_list.clone() {
            match shape {
                Shape::TriangleByPoints { point_1, point_2, point_3, color } => {
                    Self::draw_triangle(self, &shape);
                },
                Shape::TriangleByPosition { position, point_1, point_2, point_3, color } => {
                    Self::draw_triangle(self, &shape);
                },
                Shape::Circle { position, color } => {
                },
                Shape::Rectangle { position, width, height, color } => {
                    
                }

            }

        }

        &self.buffer
    }

}