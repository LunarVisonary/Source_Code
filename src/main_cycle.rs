use crate::prelude::*;
use crate::smartpointer::Sr;
use  std::{time::{Duration, Instant}, cmp::min};

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

pub enum SimType {
    Dust,
    Fluid,
    Solid,
    Special(*const dyn FnMut(&mut Map, &mut Vec2Integer, &mut Vec<PixelStruct>)),
    None
}

#[derive(Clone)]
struct TypeChanges {
    new_type: Vec<(usize, f64)>,
    bits: f64,
}

#[derive(Clone)]
struct Pixel_types {
    pixel_type: usize,
    bits: f64,
    changes: TypeChanges,
    reactions: Vec<(*const Ratio, f64)>
}

pub struct PixelStruct {
    pixel: String,
    reactions: Vec<Reaction>,
    specificheat: f64
}

struct Ratio {
    pixel_type: usize,
    ratio_num: i32,
    change: PixelChange,
    type_change: TypeChanges
}

#[derive(Clone)]
pub struct Conditions {
    heat: [f64; 2],
    pixels: Vec<usize>, 
}

pub struct Reaction {
    main_pixel: usize,
    conditions: Conditions,
    ratio: Vec<Ratio>,
    ratio_total: i32
}

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

pub struct Pixel {
    types: Vec<Pixel_types>,
    sim_type: SimType,
    perfect_position: Vec2Float,
    priority: i32,
    simulate: bool,
    tempature: f64,
    specific_heat: f64,
    total_bits: f64,
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

impl Ratio {
    fn new(pixel: usize, ratio: i32, overchange: PixelChange, tchange: TypeChanges) -> Ratio {
        Ratio { 
            pixel_type: pixel, 
            ratio_num: ratio,
            change: overchange,
            type_change: tchange
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
    fn new(m_pixel: usize, products: Vec<Pixel>, r_conditions: Conditions, ratios: Vec<Ratio>) -> Reaction {
        let mut total_of_ratios = 0;
        for ratio_num in 0..ratios.len() {
            total_of_ratios += ratios[ratio_num].ratio_num;
        }

        Reaction {
            main_pixel: m_pixel,
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
    fn new(ptypes: Vec<Pixel_types>, pos: Vec2Float, stypes: SimType, pixeltypes: Vec<PixelStruct>) -> Pixel {
        let mut ash = 0.0;
        let mut totalbits = 0.0;
        for ptype in ptypes.iter() {
            totalbits += ptype.bits;
        }
        for ptype in ptypes.iter() {
            ash += pixeltypes[ptype.pixel_type].specificheat * (ptype.bits/totalbits);
        }
        
        Pixel { 
            types: ptypes,
            sim_type: SimType::Dust, 
            perfect_position: pos,
            priority: 0,
            simulate: false,
            tempature: 0.0,
            specific_heat:ash,
            total_bits: totalbits,
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
            specific_heat: 0.0,
            total_bits: 0.0,
            changes: PixelChange { stype: SimType::None, temp_change: 0.0 }
        }
    }

    pub fn update_pixel(&mut self, types: &mut Vec<PixelStruct>) {
        
        let mut total_bits = 0.0;
        let mut ash = 0.0; 

        for ptype in self.types.iter() {
            total_bits += ptype.bits;
        }

        for ptype in self.types.iter().enumerate() {
            ash += types[ptype.0].specificheat * (ptype.1.bits/total_bits);
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
        TypeChanges { new_type: vec![], bits: 0.0}
    }
}

impl SimType {
    fn find_velocity(&self, ptypes: &mut Vec<PixelStruct>, pixel_point: Vec2Integer, map: &mut Map) { //not done
        match *self {
            Self::Dust => {

            }
            
            Self::Fluid => {

            }

            Self::Solid => {

            }

            Self::Special(_) => {

            }

            Self::None => {}
        }
    }
}

impl Vec2Float {
    fn floor_to_int(&self) -> Vec2Integer {
        Vec2Integer {x: self.x as i64, y: self.y as i64}
    }
}

fn minf64(f1: f64, f2: f64) -> f64 {
    if f1 > f2 {
        f2
    } else {
        f1
    }
}

pub fn cycle() {
    //reactions
    let pixels: Vec<PixelStruct> = vec![PixelStruct {pixel: String::from("NONE"), reactions: vec![], specificheat: 0.0}];
    let mut state = State::new(pixels);
    let mut sim_areas: Vec<Map> = vec![];
    let mut in_game = false;
    let mut loop_num = 0;
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
                loop_num += 1;
                let mutable = &mut state;
                pioritize_pixels(&mut sim_areas);
                calculate_pixel_changes(mutable);
                calculate_colisions();
                finalize_changes(mutable);
                if loop_num >= 8 {break;}
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
                    let sim_heat = pixel.get_immut().tempature; //get simulated pixels heat
                    for mut scanned_pixel in scanned_pixels.drain(..) { //drain it
                        let heat_diff = (scanned_pixel.get_immut().tempature - sim_heat)/16.0; //find hear difference and apply changes
                        pixel.get_mut().changes.temp_change += heat_diff; //change iterated pixel
                        scanned_pixel.get_mut().changes.temp_change += heat_diff; //change scanned pixel
                    }
                }
                let mut adjacent_pixels = find_adjacent_pixels(map, enumeration); //find all adjacent pixels for reactions
                for ptype in 0..pixel.get_immut().types.len() {
                    for reaction in state.pixel_types[ptype].reactions.iter_mut()  {
                        let mut requirements: Vec<(f64, Vec<(*mut Pixel_types, f64)>)> = vec![(0.0, vec![]); reaction.ratio.len()];
                        for ratio in reaction.ratio.iter_mut().enumerate() {
                            for mut nearby_pixel in adjacent_pixels.drain(..) {
                                for scanned_ptype in 0..nearby_pixel.get_immut().types.len()  {
                                    if scanned_ptype == ratio.1.pixel_type {
                                        requirements[ratio.0].0 += nearby_pixel.get_immut().types[scanned_ptype].bits / ratio.1.ratio_num as f64;
                                        requirements[ratio.0].1.push((&mut nearby_pixel.get_mut().types[scanned_ptype] as *mut Pixel_types, nearby_pixel.get_immut().types[scanned_ptype].bits));
                                    }
                                }
                            }
                        }
                        let mut minimum: f64 = 0.0;
                        for react_lvl in requirements.iter().enumerate() {
                            if minimum > react_lvl.1.0 || react_lvl.0 == 0 {
                                minimum = react_lvl.1.0;
                            }
                        }

                        for reactionary_pix in requirements.drain(..).enumerate() {
                            let reactionary_ratio = minimum / reactionary_pix.1.0;
                            for pixel in reactionary_pix.1.1 {
                                unsafe {
                                    let smallest = minf64(minimum, pixel.1) * reactionary_ratio;
                                    (*pixel.0).reactions.push((&reaction.ratio[reactionary_pix.0] as *const Ratio, smallest));
                                }
                            }
                        }
                    }           
                }
                //physics simulation!
                pixel.get_immut().sim_type.find_velocity(&mut state.pixel_types, pixel.get_immut().perfect_position.floor_to_int(), map);
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
            if pixel.simulate {
                pixel.tempature += pixel.changes.temp_change;
                pixel.changes.temp_change = 0.0;

                for ptype in pixel.types.iter_mut() {
                    let reactions = ptype.reactions.len();
                    for reaction in ptype.reactions.drain(..) {
                        unsafe {
                            ptype.changes.bits += ((*reaction.0).type_change.bits * reaction.1) / reactions as f64; //change bits in future
                            for new_types in (*reaction.0).type_change.new_type.iter() {
                                let mut new_types = *new_types;
                                new_types.1 = new_types.1 * reaction.1 / reactions as f64;
                                ptype.changes.new_type.push(new_types); //add new types (ps news reaction adjustment and to be a vector)
                            }
                            pixel.tempature += ((*reaction.0).change.temp_change * reaction.1)/pixel.specific_heat * pixel.total_bits * reactions as f64; //
                        }
                    }
                }

                match pixel.changes.stype { //change simtype
                    SimType::None => {},
                    SimType::Dust => {
                        pixel.sim_type = SimType::Dust
                    },
                    SimType::Fluid => {
                        pixel.sim_type = SimType::Fluid
                    },
                    SimType::Solid => {
                        pixel.sim_type = SimType::Solid
                    },
                    SimType::Special(x) => {
                        pixel.sim_type = SimType::Special(x)
                    }
                }

                pixel.changes.stype = SimType::None;

                {
                    for pix_type_num in 0..pixel.types.len() { //change pixel types
                        for newtype in 0..pixel.types[pix_type_num].changes.new_type.len() {
                            let types = &mut pixel.types;
                            let id = types[pix_type_num].changes.new_type[newtype].0; //get change id
                            if id != 0 { //scan for valid change type
                                types[pix_type_num].bits += types[pix_type_num].changes.bits; //change bits
                                let mut already_a_type = (false, 0); //look for pre-existing equal type
                                for ptypes in types.iter().enumerate() { 
                                    if ptypes.1.pixel_type == types[pix_type_num].changes.new_type[newtype].0 { //if existing
                                        already_a_type = (true, ptypes.0); //set true and get number
                                    }
                                }
                                if already_a_type.0 { //if pre-existing
                                    types[already_a_type.1].bits += types[pix_type_num].changes.bits;
                                } else {
                                    types.push(Pixel_types { pixel_type: types[pix_type_num].changes.new_type[newtype].0, bits: types[pix_type_num].changes.new_type[newtype].1, changes: TypeChanges::default(), reactions: vec![]});
                                }
                            }
                        }
                    }
                }  
                pixel.update_pixel(&mut state.pixel_types);
            }
        }
    }
}