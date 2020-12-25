use yew::web_sys::{CanvasRenderingContext2d};


type NumberType = u32;
pub struct Renderer{
    rendering_context: CanvasRenderingContext2d,
    font: Option<String>,
    height: NumberType,
    width: NumberType
}
// builders
impl Renderer {
    pub fn new(rendering_context:  CanvasRenderingContext2d,height:NumberType,width:NumberType)->Self{
        Self{rendering_context,font:None,height,width}
    }
    pub fn with_font(self,font:String)->Self{
        Self{font:Some(font),..self}
    }

}

// instance methods
impl Renderer {
    pub fn draw_rects(&self,rects:&[Rect]){
        for rect in rects{
            self.draw_rect(rect);
        }
    }
    pub fn draw_rect(&self,rect:&Rect){
        self.rendering_context.fill_rect(rect.origin.x as f64,rect.origin.y as f64,rect.width as f64,rect.height as f64);
    }
    pub fn clear_all(&self){
        self.rendering_context.clear_rect(0.0,0.0,self.width as f64,self.height as f64);

    }
    pub fn clear_area(&self,area: Rect){
        self.rendering_context.clear_rect(area.origin.x as f64,area.origin.y as f64,area.width as f64,area.height as f64);
    }
}



// enum Shapes{
//     Rect,
//     Circle
// }
#[derive(Clone,Copy)]
pub struct Origin{
    pub x: NumberType,
    pub y: NumberType
}
#[derive(Clone,Copy)]
pub struct Rect{
    pub origin: Origin,
    pub height:NumberType,
    pub width: NumberType
}
#[derive(Clone,Copy)]
pub struct Circle{
    origin: Origin,
    radius: NumberType,
}

impl Rect{
    pub fn default()->Self{
        Self{origin:Origin{x:0,y:0},width:0,height:0}
    }
}