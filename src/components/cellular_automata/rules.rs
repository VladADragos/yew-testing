use super::cellular_automata::CellStates;
const  rule28 :[u8;8]=   [0,0,0,1,1,1,0,0];
const  rule30 :[u8;8]=   [0,0,0,1,1,1,1,0];
const  rule50 :[u8;8]=   [0,0,1,1,0,0,1,0];
const  rule54 :[u8;8]=   [0,0,1,1,0,1,1,0];
const  rule60 :[u8;8]=   [0,0,1,1,1,1,0,0];
const  rule90 :[u8;8]=   [0,1,0,1,1,0,1,0];
const  rule94 :[u8;8]=   [0,1,0,1,1,1,1,0];
const  rule102 :[u8;8]= [0,1,1,0,0,1,1,0];
const  rule110 :[u8;8]= [0,1,1,0,1,1,1,0];
const  rule126 :[u8;8]= [0,1,1,1,1,1,1,0];
const  rule150 :[u8;8]= [1,0,0,1,0,1,1,0];
const  rule158 :[u8;8]= [1,0,0,1,1,1,1,0];
const  rule188 :[u8;8]= [1,0,1,1,1,1,0,0];
const  rule190 :[u8;8]= [1,0,1,1,1,1,1,0];
const  rule220 :[u8;8]= [1,1,0,1,1,1,0,0];
const  rule222 :[u8;8]= [1,1,0,1,1,1,1,0];


pub enum Rule{
  rule28  = 28,    
  rule30  = 30,  
  rule50  = 50,  
  rule54  = 54,   
  rule60  = 60,   
  rule90  = 90,   
  rule94  = 94,   
  rule102 = 102, 
  rule110 = 110, 
  rule126 = 126, 
  rule150 = 150, 
  rule158 = 158, 
  rule188 = 188, 
  rule190 = 190, 
  rule220 = 220, 
  rule222 = 222 
}

impl Rule{
    pub fn get_rule(rule: Rule)->&'static [u8]{
        match rule {
            Rule::rule28  => &rule28,    
            Rule::rule30  => &rule30,  
            Rule::rule50  => &rule50,  
            Rule::rule54  => &rule54,   
            Rule::rule60  => &rule60,   
            Rule::rule90  => &rule90,   
            Rule::rule94  => &rule94,   
            Rule::rule102 => &rule102, 
            Rule::rule110 => &rule110, 
            Rule::rule126 => &rule126, 
            Rule::rule150 => &rule150, 
            Rule::rule158 => &rule158, 
            Rule::rule188 => &rule188, 
            Rule::rule190 => &rule190, 
            Rule::rule220 => &rule220, 
            Rule::rule222 => &rule222 
        }
    }
    pub fn get_new_state(neighborhood: (&CellStates,&CellStates,&CellStates),rule_set: &[u8])->u8{
        match neighborhood{
            (CellStates::alive,CellStates::alive,CellStates::alive) => rule_set[0],
            (CellStates::alive,CellStates::alive,CellStates::dead)  => rule_set[1],
            (CellStates::alive,CellStates::dead,CellStates::alive)  => rule_set[2],
            (CellStates::alive,CellStates::dead,CellStates::dead)   => rule_set[3],
            (CellStates::dead,CellStates::alive,CellStates::alive)  => rule_set[4],
            (CellStates::dead,CellStates::alive,CellStates::dead)   => rule_set[5],
            (CellStates::dead,CellStates::dead,CellStates::alive)   => rule_set[6],
            (CellStates::dead,CellStates::dead,CellStates::dead)    => rule_set[7],
            (_,_,_) => rule_set[8]
        }
    }
}
