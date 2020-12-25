use crate::data_types::colors::Color;

#[derive(Clone,Copy)]
pub struct Origin{
    pub x: u32,
    pub y: u32
}
#[derive(Clone,Copy)]
pub struct Rect{
    pub origin: Origin,
    pub height:u32,
    pub width: u32,
    pub color:Color
}
#[derive(Clone,Copy)]
pub struct Circle{
    origin: Origin,
    radius: u32,
}
impl Origin{
    pub fn default()->Self{
        Self{x:0,y:0}
    }
    pub fn new(x:u32,y:u32)->Self{
        Self{x,y}
    }
}

impl Rect{
    pub fn default()->Self{
        Self{origin:Origin{x:0,y:0},width:0,height:0,color:Color::white}
    }
    pub fn new(x:u32,y:u32,width:u32,height:u32,color:Color)->Self{
        Self{origin:Origin{x,y},width,height,color}
    }
}
