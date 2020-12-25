use super::cellular_automata::CellStates;
use crate::data_types::shapes::{Rect,Origin};
use crate::data_types::colors::Color;

pub struct VisualBuffer{
    buffer: Vec<Rect>,
    cell_size:u32,
    height:u32,
    width:u32
}

impl VisualBuffer{
    pub fn new(height:u32,width:u32,cell_size:u32)->Self{
        let mut vb = Self{ buffer:Vec::with_capacity(width as usize),width,height,cell_size };
        vb.generate_line_buffer();
        vb
    }
}

impl VisualBuffer{
    fn generate_line_buffer(&mut self){
        for i in 0 ..self.buffer.len(){
            let rect = Rect::new( 
                (i as u32) *self.cell_size ,
                0*self.cell_size, 
                self.cell_size,
                self.cell_size,
                Color::white

            );
            self.buffer[i] = rect;
        }
    }
    pub fn onSet(&mut self,y:usize,x: usize, value: CellStates) {
         let rect = &mut self.buffer[x];
    
         match value{
             CellStates::alive => {rect.color = Color::black},
             _=>{rect.color = Color::white} 
         }
         rect.origin.y = y as u32 * self.cell_size;
    
        }
    pub fn onSpawn(&mut self,spawn_location:usize){

    }
}

