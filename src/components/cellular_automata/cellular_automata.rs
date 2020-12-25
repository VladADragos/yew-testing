use super::rules::Rule;
use super::visual_buffer::VisualBuffer;
#[derive(Clone)]
pub enum CellStates{
    dead = 0,
    alive = 1
}


impl CellStates{
    pub fn from_int(int:u8)->CellStates{
        match int{
            0=> CellStates::dead,
            1=>CellStates::alive,
            _=> CellStates::alive
        }
    }
}

const rule28:[u8;8] = [0,0,0,1,1,1,0,0];


struct CellularAutomata{
    line_buffer: Vec<CellStates>,
    new_line_buffer: Vec<CellStates>,
    current_line: u16,
    height: u16 ,
    width: u16,
    rule: &'static [u8],
    observer: VisualBuffer
}

impl CellularAutomata{
    pub fn new(width: u16, height: u16,rule: Rule,observer:VisualBuffer)->Self{
        Self{
            line_buffer: Vec::with_capacity(width as usize),
            new_line_buffer : Vec::with_capacity(width as usize),
            current_line: 0,
            height,
            width,
            observer,
            rule: Rule::get_rule(rule), 
        }
    }
}

impl CellularAutomata{
    pub fn next_state(&mut self){
        for x in 0..self.line_buffer.len(){
            let current = & self.line_buffer[x];
            let left = & self.line_buffer[(x+(self.width-1 ) as usize)%self.width as usize];
            let right = & self.line_buffer[(x+1)%self.width as usize];
            let new_state = Rule::get_new_state((left,right,current), self.rule);
            self.new_line_buffer[x] = if new_state == 0 {CellStates::dead} else {CellStates::alive}; 
            self.notifySet(self.current_line as usize,x,CellStates::from_int(new_state));
        }
        self.line_buffer = self.new_line_buffer.clone();
        self.current_line = (self.current_line +1 ) % (self.height);
    }

    fn notifySet(&mut self,from:usize,to:usize,new_state: CellStates){
        self.observer.onSet(from,to,new_state);

    }
    fn notifySpawn(&mut self,spawn_location:usize){
        self.observer.onSpawn(spawn_location);
    }
}