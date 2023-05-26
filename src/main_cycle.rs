use crate::prelude::*;
use crate::smartpointer::Sr;
use  std::time::{Duration, Instant};

pub const SCREEN_WIDTH: i64 = 5;
pub const SCREEN_HEIGHT: i64 = 3; 
pub const  SIZE: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone)]
enum Menus {
    MainMenu,
    SaveSelector,
    Quit,
    InGame
} 

#[derive(Copy, Clone, PartialEq)]
pub enum SimType {
    Dust,
    Fluid,
    Solid,
    Special,
    None
}

#[derive(Copy, Clone)]
struct TypeChanges {
    new_type: usize,
    bits: f64
}

#[derive(Copy, Clone)]
struct Pixel_types {
    pixel_type: usize,
    bits: f64,
    changes: TypeChanges
}

#[derive(Clone)]
pub struct PixelStruct {
    pixel: String,
    reactions: Vec<Reaction>,
    id: usize,
}

#[derive(Copy, Clone)]
struct Ratio {
    pixel_type: usize,
    ratio_num: i32,
}

#[derive(Clone)]
pub struct Conditions {
    heat: [f64; 2],
    pixels: Vec<usize>, 
}

#[derive(Clone)]
pub struct Reaction {
    main_pixel: usize,
    change: PixelChange,
    conditions: Conditions,
    ratio: Vec<Ratio>,
    ratio_total: i32
}

#[derive(Copy, Clone)]
pub struct PixelChange {
    stype: SimType,
    temp_change: f64,
}

#[derive(Copy, Clone)]
pub struct Vec2Integer {
    pub x: i64,
    pub y: i64
}

#[derive(Copy, Clone, PartialEq)]
pub struct  Vec2Float {
    pub x: f64,
    pub y: f64
}

#[derive(Clone)]
pub struct Pixel {
    types: Vec<Pixel_types>,
    sim_type: SimType,
    perfect_position: Vec2Float,
    priority: i32,
    simulate: bool,
    tempature: f64,
    changes: PixelChange
}

struct GameStates {
    game_area: Menus,
}

pub struct State {
 gamestate: Menus,
 pub simulated_area: Vec<Map>, //Needs finishing
 pub pixel_types: Vec<PixelStruct>
}

impl PixelStruct {
    fn config(vector: &mut Vec<PixelStruct>, reactions: Vec<Reaction>) {
        
    }
}

impl Ratio {
    fn new(pixel: usize, ratio: i32) -> Ratio {
        Ratio { 
            pixel_type: pixel, 
            ratio_num: ratio 
        }
    }
}

impl Conditions {
    fn new(types: Vec<usize>, heat_values: [f64; 2]) -> Conditions {
        Conditions {
            heat: heat_values,
            pixels: types
        }
    }
}

impl Reaction {
    fn new(m_pixel: usize, products: Vec<Pixel>, changes: PixelChange, r_conditions: Conditions, ratios: Vec<Ratio>) -> Reaction {
        let mut total_of_ratios = 0;
        for ratio_num in 0..ratios.len() {
            total_of_ratios += ratios[ratio_num].ratio_num;
        }

        Reaction {
            main_pixel: m_pixel,
            change: changes,
            conditions: r_conditions,
            ratio: ratios,
            ratio_total: total_of_ratios,
        }
    }
    
    fn configure_reactions() {
        
    }
}

impl State {
    fn new(pixels: Vec<PixelStruct>) -> State {
        State { 
            gamestate: Menus::MainMenu, 
            simulated_area: vec![],
            pixel_types: pixels
        }
    }
}

impl Pixel {
    fn new(ptypes: Vec<Pixel_types>, pos: Vec2Float, stypes: SimType) -> Pixel {
        Pixel { 
            types: ptypes,
            sim_type: SimType::Dust, 
            perfect_position: pos,
            priority: 0,
            simulate: false,
            tempature: 0.0,
            changes: PixelChange::new(stypes)
        }
    }

    pub fn default() -> Pixel {
        Pixel { 
            types: vec![],
            sim_type: SimType::None,
            perfect_position: Vec2Float { x: 0.0, y: 0.0 },
            priority: 0,
            simulate: false, 
            tempature: 0.0,
            changes: PixelChange { stype: SimType::None, temp_change: 0.0 }
        }
    }
}

impl PixelChange {
    fn new(sim_type: SimType) -> PixelChange {
        PixelChange {
            stype: sim_type,
            temp_change: 0.0,
        }
    }
}

impl TypeChanges { //may cause problems do to coersions
    fn default() -> TypeChanges {
        TypeChanges { new_type: 0, bits: 0.0 }
    }
}

pub fn cycle() {
    //reactions
    let pixels: Vec<PixelStruct> = vec![PixelStruct {pixel: String::from("NONE"), reactions: vec![], id: 0}];
    let mut state = State::new(pixels);
    let mut sim_areas: Vec<Map> = vec![];
    let mut in_game = false;
    //main loop
    loop {
        //match game state
        match state.gamestate {
            Menus::Quit => {
                println!("You have quit");
                break;
            },
            Menus::MainMenu => {
                //do systems
                let mutable = &mut state;
                pioritize_pixels(&mut sim_areas);
                calculate_pixel_changes(mutable);
                calculate_colisions();
                finalize_changes(mutable);
                break;
            },
            Menus::InGame => {
                in_game = true;
            },
            _ => {}
        }
    }
}


fn pioritize_pixels(areas: &mut Vec<Map>) {
    //make changes for all maps
    for maps in areas {
        //calculate and store priority
        let mut priority_total: i32 = 0;
        for smrt_ref in &maps.area {
            //measure total priority
            priority_total += smrt_ref.get_immut().priority;
        }
        //set simulation on or off
        for smrt_ref in &mut maps.area {
            
            let priority = priority_total / (SCREEN_HEIGHT * SCREEN_WIDTH) as i32;
            let pixel = smrt_ref.get_mut();
            if pixel.priority >= priority {
                pixel.simulate = true;
            }
            else {
                pixel.simulate = false;
            }
        }
    }
}



fn calculate_pixel_changes(state: &mut State)    {
    for map in state.simulated_area.iter_mut() { //
        for enumeration in 0..SIZE {
            let mut pixel = map.area[enumeration].clone(); //get pixel
            if pixel.get_immut().simulate { //look for sim
                {
                    let mut scanned_pixels = find_nearby_pixels(enumeration, map); //find nearby pixels (4 pixels)
                    {
                        let sim_heat = pixel.get_immut().tempature; //get simulated pixels heat
                        for mut scanned_pixel in scanned_pixels.drain(..) { //drain it
                            let heat_diff = (scanned_pixel.get_immut().tempature - sim_heat)/16.0; //find hear difference and apply changes
                            pixel.get_mut().changes.temp_change += heat_diff; //change iterated pixel
                            scanned_pixel.get_mut().changes.temp_change += heat_diff; //change scanned pixel
                        }
                    }
                }
            }                      
        }
    }
}

fn calculate_colisions() {

}

fn finalize_changes(state: &mut State) {
    for map in state.simulated_area.iter_mut() {
        for enumeration in 0..SIZE {
            let mut pixel = map.area[enumeration].get_mut();
            pixel.tempature += pixel.changes.temp_change;
            pixel.changes.temp_change = 0.0;

            match pixel.changes.stype { //change simtype
                SimType::None => {},
                _ => {pixel.sim_type = pixel.changes.stype}
            }

            {
                for pix_type_num in 0..pixel.types.len() { //change pixel types
                    let types = &mut pixel.types;
                    let id = types[pix_type_num].changes.new_type; //get change id
                    if id != 0 { //scan for valid change type
                        types[pix_type_num].bits += -types[pix_type_num].changes.bits; //change bits
                        let mut already_a_type = (false, 0); //look for pre-existing equal type
                        for ptypes in types.iter().enumerate() { 
                            if ptypes.1.pixel_type == types[pix_type_num].changes.new_type { //if existing
                                already_a_type = (true, ptypes.0); //set true and get number
                            }
                        }
                        if already_a_type.0 { //if pre-existing
                            types[already_a_type.1].bits += types[pix_type_num].changes.bits;
                        } else {
                            types.push(Pixel_types { pixel_type: types[pix_type_num].changes.new_type, bits: types[pix_type_num].changes.bits, changes: TypeChanges::default()});
                        }
                    }
                }
            }  
        }
    }
}