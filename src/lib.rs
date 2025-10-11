pub use minifb::*;

// Define a vector2 structure
#[derive(Copy, Clone)]
pub struct Vec2{
    pub x: i32,
    pub y: i32 
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
    pub r: u8,
    pub g: u8,
    pub b: u8
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
    Triangle {position: Vec2, vertex_1: Vec2, vertex_2: Vec2, vertex_3: Vec2, color:Color}, // Take a position and 3 vertex relative to the position.
    Circle {position: Vec2,radius: u32, color:Color},
    Rectangle {position: Vec2, width: u32, height: u32, color:Color}

}

impl Shape{

    pub fn set_position(&mut self, _position: Vec2){
        match self {
            Self::Triangle { position, ..} 
            | Self::Circle { position, ..}
            | Self::Rectangle { position, ..}  => *position = _position
        }
    }

    pub fn set_color(&mut self, _color: Color){
        match self {
            Self::Triangle { color, ..} 
            | Self::Circle { color, ..} 
            | Self::Rectangle { color, ..}  => *color = _color
        }
    }

    pub fn set_vertex(&mut self, vertex_id:u8, vertex: Vec2){
        match self {
            Self::Triangle {vertex_1, vertex_2, vertex_3,..} => {
                match vertex_id {
                    1 => {*vertex_1 = vertex}
                    2 => {*vertex_2 = vertex}
                    3 => {*vertex_3 = vertex}
                    _ => {panic!(
                        "ERROR: Invalide vertex id \nfile : {}\nline : {}",
                        file!(),
                        line!()
                    )}
                    
                }
            },
            Self::Circle {..} => panic!(
                "ERROR: You cannot change the vertices of a Circle\nFile : {}\nLine : {}",
                file!(),
                line!(),
            ),
            Self::Rectangle { ..}  => panic!(
                "ERROR: You cannot change the vertices of a Rectangle\nFile : {}\nLine : {}",
                file!(),
                line!(),
            )
        }
    }

    pub fn set_radius(&mut self, _radius: u32){
        match self {
            Self::Circle {radius,..} => *radius = _radius,
            Self::Triangle {..} => panic!(
                "ERROR: You cannot change the radius of a Triangle\nFile : {}\nLine : {}",
                file!(),
                line!(),
            ),
            Self::Rectangle {..}  => panic!(
                "ERROR: You cannot change the radius of a Rectangle\nFile : {}\nLine : {}",
                file!(),
                line!(),
            )
        }
    }

    pub fn set_dimensions(&mut self, _width: u32, _height: u32){
        match self {
                        Self::Rectangle { width, height, .. } => {
                *width = _width;
                *height = _height;
            },
            Self::Triangle {..} => panic!(
                "ERROR: You cannot change the dimensions of a Triangle\nFile : {}\nLine : {}",
                file!(),
                line!(),
            ),
            Self::Circle {..} => panic!(
                "ERROR: You cannot change the dimensions of a Circle \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )
        }
    }

    pub fn get_position(&self) -> Vec2 {
        let mut _position: Vec2;
        match self {
            Self::Triangle { position, .. }   
            | Self::Circle { position, .. } 
            | Self::Rectangle { position, .. } => _position = *position
        }
        _position
    }

    pub fn get_color(&self) -> Color{
        let mut _color;
        match self {
            Self::Triangle {color, ..}
            | Self::Circle {color, ..}
            | Self::Rectangle {color, ..} => _color = *color 
            
        }
        _color
    }

    pub fn get_vertex(&self, vertex_id: u8) -> Vec2{
        let mut _vertex: Vec2;
        match self {
            Self::Triangle {vertex_1, vertex_2, vertex_3, ..} => {
                match vertex_id {
                    1 => {_vertex = *vertex_1}
                    2 => {_vertex = *vertex_2}
                    3 => {_vertex = *vertex_3}
                    _ => {panic!(
                    "ERROR: Invalide vertex id \nFile : {}\nLine : {}",
                    file!(),
                    line!(),
                    )}
                }
            },
            Self::Circle {..} => {panic!(
                "ERROR: Circle don't have vertices \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )},
            Self::Rectangle {..} => {panic!(
                "ERROR: Rectangle don't have vertices \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
        }
        _vertex
    }
 
    pub fn get_radius(&self) -> u32 {
        let mut _radius: u32;
        match self {
            Self::Circle { radius, ..} => _radius = *radius,
            Self::Triangle {..} => {panic!(
                "ERROR: Triangle don't have radius \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
            Self::Rectangle {..} => {panic!(
                "ERROR: Rectangle don't have radius \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
        }
        _radius
    }

    pub fn get_width(&self) -> u32{
        let mut _widht: u32;
        match self {
            Self::Rectangle {width, ..} => _widht = *width,
            Self::Circle {..} => {panic!(
                "ERROR: Circle don't have width \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
            Self::Triangle {..} => {panic!(
                "ERROR: Triangle don't have width \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
        }
        _widht
    }

    pub fn get_height(&self) -> u32{
        let mut _height: u32;
        match self {
            Self::Rectangle {height, ..} => _height = *height,
            Self::Circle {..} => {panic!(
                "ERROR: Circle don't have height \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
            Self::Triangle {..} => {panic!(
                "ERROR: Triangle don't have height \nFile : {}\nLine : {}",
                file!(),
                line!(),
            )}
        }
        _height
    }


}

// Define a Canvas structure 
#[derive(Clone)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
    pub shapes_list: Vec<Shape>,
    pub frame_buffer: Vec<u32>

}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self{
        let frame_buffer: Vec<u32> = vec![background_color.to_u32(); (width*height) as usize];
        Canvas { width, height, background_color, shapes_list: Vec::<Shape>::new(), frame_buffer}

    }

    // Get the 2D cross product by giving the 3 point of the 2 vector
    pub fn _2d_cross_product(pixel_coordinates: Vec2, vertex_1: Vec2, vertex_2: Vec2) -> bool {
        (((vertex_1.x - pixel_coordinates.x)*(vertex_2.y - pixel_coordinates.y)) - ((vertex_1.y - pixel_coordinates.y)*(vertex_2.x - pixel_coordinates.x))) >= 0 
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

    pub fn draw_circle(&mut self, position: Vec2, radius: u32, color: Color){
        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = position.x + radius as i32;
        let rtzr_area_min_x:i32 = position.x - radius as i32;
        let rtzr_area_max_y:i32 = position.y + radius as i32;
        let rtzr_area_min_y:i32 = position.y - radius as i32;

        //Check for every pixel if it is in the circle. If yes, color it in the color specified
        for y in rtzr_area_min_y..rtzr_area_max_y{
            for x in rtzr_area_min_x..rtzr_area_max_x{
                let cursor: Vec2 = Vec2::new(x, y);
                if Vec2::sub(cursor, position).length() <= radius as f32{
                    self.frame_buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = color.to_u32();
                }
            }
        }
    }

    pub fn draw_rectangle(&mut self, position: Vec2, _width: u32, _height: u32, color: Color){
        // Find the area of rasterization 
        let rtzr_area_max_x:i32 = position.x + (_width/2) as i32;
        let rtzr_area_min_x:i32 = position.x - (_width/2) as i32;
        let rtzr_area_max_y:i32 = position.y + (_height/2) as i32;
        let rtzr_area_min_y:i32 = position.y - (_height/2) as i32;

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
        self.frame_buffer = vec![self.background_color.to_u32(); (self.width*self.height) as usize];
    }
    //Gives the frame buffer of the canvas
    pub fn as_buffer(&mut self) -> &Vec<u32> {
        &self.frame_buffer
    }

    pub fn draw_shapes(&mut self){
        for shape in & self.shapes_list {
            match shape {
                Shape::Circle { position, radius, color } => {
                    // Find the area of rasterization 
                    let rtzr_area_max_x:i32 = position.x + *radius as i32;
                    let rtzr_area_min_x:i32 = position.x - *radius as i32;
                    let rtzr_area_max_y:i32 = position.y + *radius as i32;
                    let rtzr_area_min_y:i32 = position.y - *radius as i32;

                    //Check for every pixel if it is in the circle. If yes, color it in the color specified
                    for y in rtzr_area_min_y..rtzr_area_max_y{
                        for x in rtzr_area_min_x..rtzr_area_max_x{
                            let cursor: Vec2 = Vec2::new(x, y);
                            if Vec2::sub(cursor, *position).length() <= *radius as f32{
                                self.frame_buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = color.to_u32();
                            }
                        }
                    }
                },
                Shape::Rectangle { position, width, height, color } => {
                    // Find the area of rasterization 
                    let rtzr_area_max_x:i32 = position.x + (width/2) as i32;
                    let rtzr_area_min_x:i32 = position.x - (width/2) as i32;
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
                },
                Shape::Triangle { position, vertex_1, vertex_2, vertex_3, color } => {
                    let _vertex_1 = Vec2::add(*position, *vertex_1);
                    let _vertex_2 = Vec2::add(*position, *vertex_2);
                    let _vertex_3 = Vec2::add(*position, *vertex_3);
                    
                    // Find the area of rasterization 
                    let rtzr_area_max_x:i32 = *(vec![_vertex_1.x, _vertex_2.x, _vertex_3.x]).iter().max().unwrap();
                    let rtzr_area_min_x:i32 = *(vec![_vertex_1.x, _vertex_2.x, _vertex_3.x]).iter().min().unwrap();
                    let rtzr_area_max_y:i32 = *(vec![_vertex_1.y, _vertex_2.y, _vertex_3.y]).iter().max().unwrap();
                    let rtzr_area_min_y:i32 = *(vec![_vertex_1.y, _vertex_2.y, _vertex_3.y]).iter().min().unwrap();
                    println!("{}", rtzr_area_max_x);
                    println!("{}", rtzr_area_min_x);
                    println!("{}", rtzr_area_max_y);
                    println!("{}", rtzr_area_min_y);
                    println!("");
                    let mut cursor: Vec2;
                    //Check for every pixel if it is in the triangle. If yes, color it in the color specified
                    for y in rtzr_area_min_y..rtzr_area_max_y{
                        for x in rtzr_area_min_x..rtzr_area_max_x{
                            cursor = Vec2::new(x, y);
                            //print!("{}, {}; {}, {}; {}, {}; {}, {} ", _vertex_1.x, _vertex_1.y, _vertex_2.x, _vertex_2.y, _vertex_3.x, _vertex_3.y, cursor.x, cursor.y);
                            if Self::is_in_triangle(
                                _vertex_1, 
                                _vertex_2, 
                                _vertex_3, 
                                cursor) {
                                self.frame_buffer[(cursor.y as usize*(self.width as usize))+cursor.x as usize] = color.to_u32();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn push_shape(&mut self,shape: Shape){
        self.shapes_list.push(shape);
    }

    pub fn clone_shape(&mut self, shape: &Shape){
        self.shapes_list.push(shape.clone());
    }
}
