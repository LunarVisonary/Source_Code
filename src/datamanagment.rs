use std::fs;
use std::path::Path;

pub fn fetch_save(save: &str) { //UNFINISHED AND I DONT WANNA DEAL WITH IT
    let path = Path::new(save);
    let data = match fs::read_to_string(path) {
        Ok(deta) => {deta}
        Err(x) => {panic!("failed: save doesnt exist {}", x);}
    }; 
    let mut count = 0;
    let mut unfinished = true;
    while unfinished {
        
    }
}