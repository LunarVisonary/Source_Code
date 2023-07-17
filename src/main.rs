mod map;
mod main_cycle;
mod smartpointer;
mod datamanagment;
mod player;

mod prelude {
    pub use crate::map::*;
    pub use crate::main_cycle::*;
}

use crate::main_cycle::*;

fn main() {
    open_window(); //needs actual window
    cycle(); 
}

fn open_window() {
    
}
