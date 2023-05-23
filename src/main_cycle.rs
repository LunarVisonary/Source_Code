use crate::prelude::*;
use crate::SmartPointer::Sr;

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
struct TypeChanges<'a> {
    new_type: &'a PixelStruct<'a>,
    bits: f64
}

#[derive(Copy, Clone)]
struct Pixel_types<'a> {
    pixel_type: &'a PixelStruct<'a>,
    bits: f64,
    changes: TypeChanges<'a>
}

#[derive(Clone)]
pub struct PixelStruct<'a> {
    pixel: &'a str,
    reactions: Vec<Reaction<'a>>,
    id: usize,
}

#[derive(Copy, Clone)]
struct Ratio<'a> {
    pixel_type: &'a PixelStruct<'a>,
    ratio_num: i32,
}

#[derive(Clone)]
pub struct Conditions<'a> {
    heat: [f64; 2],
    pixels: Vec<&'a PixelStruct<'a>>, 
}

#[derive(Clone)]
pub struct Reaction<'a> {
    main_pixel: &'a PixelStruct<'a>,
    change: PixelChange,
    conditions: Conditions<'a>,
    ratio: Vec<Ratio<'a>>,
    ratio_total: i32
}

#[derive(Copy, Clone)]
pub struct PixelChange {
    stype: SimType,
    temp_change: f64,
}

#[derive(Copy, Clone)]
pub struct Vec2Integer {
    pub x: usize,
    pub y: usize
}

#[derive(Copy, Clone, PartialEq)]
pub struct  Vec2Float {
    pub x: f64,
    pub y: f64
}

#[derive(Clone)]
pub struct Pixel<'a> {
    types: Vec<Pixel_types<'a>>,
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

pub struct State<'a> {
 gamestate: Menus,
 pub simulated_area: Vec<Map<'a>>, //Needs finishing
 pub pixel_types: Vec<PixelStruct<'a>>
}

impl PixelStruct<'_> {
    fn config(vector: &mut Vec<PixelStruct>, reactions: Vec<Reaction>) {
        
    }
}

impl Ratio<'_> {
    fn new<'a>(pixel: &'a PixelStruct, ratio: i32) -> Ratio<'a> {
        Ratio { 
            pixel_type: pixel, 
            ratio_num: ratio 
        }
    }
}

impl Conditions<'_> {
    fn new<'a>(types: Vec<&'a PixelStruct>, heat_values: [f64; 2]) -> Conditions<'a> {
        Conditions {
            heat: heat_values,
            pixels: types
        }
    }
}

impl Reaction<'_> {
    fn new<'a>(m_pixel: &'a PixelStruct, products: Vec<Pixel>, changes: PixelChange, r_conditions: Conditions<'a>, ratios: Vec<Ratio<'a>>) -> Reaction<'a> {
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

impl State<'_> {
    fn new(pixels: Vec<PixelStruct>) -> State {
        State { 
            gamestate: Menus::MainMenu, 
            simulated_area: vec![],
            pixel_types: pixels
        }
    }
}

impl Pixel<'_> {
    fn new<'a>(ptypes: Vec<Pixel_types<'a>>, pos: Vec2Float, stypes: SimType) -> Pixel<'a> {
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

    pub fn default<'a>() -> Pixel<'a> {
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

impl TypeChanges<'_> { //may cause problems do to coersions
    fn default<'a>(ptype: &'a PixelStruct<'a>) -> TypeChanges<'a> {
        TypeChanges { new_type: ptype, bits: 0.0 }
    }
}

pub fn cycle() {
     //reactions
    let pixels: Vec<PixelStruct> = vec![PixelStruct {pixel: "NONE", reactions: vec![], id: 0}];
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
            /*
            match pixel.changes.stype {
                SimType::None => {},
                _ => {pixel.sim_type = pixel.changes.stype}
            }

            for pix_type in pixel.types.iter_mut() {
                let id = pix_type.changes.new_type.id;
                if id != 0 {
                    pix_type.bits += -pix_type.changes.bits;
                    let mut already_a_type = (false, 0);
                    for types in pixel.types.iter_mut().enumerate() {
                        if types.1.pixel_type.id == pix_type.changes.new_type.id {
                            already_a_type = (true, types.0);
                        }
                    }
                    if already_a_type.0 {
                        pixel.types[already_a_type.1].bits += pix_type.changes.bits;
                    } else {
                        pixel.types.push(Pixel_types { pixel_type: pix_type.changes.new_type, bits: pix_type.changes.bits, changes: TypeChanges::default(&state.pixel_types[0])});
                    }
                }
            } */
        }
    }
}