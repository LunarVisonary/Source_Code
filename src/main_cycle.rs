use crate::prelude::*;

pub const SCREEN_WIDTH: i64 = 5;
pub const SCREEN_HEIGHT: i64 = 3; 

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
    change: PixelChange<'a>,
    conditions: Conditions<'a>,
    ratio: Vec<Ratio<'a>>,
    ratio_total: i32
}

#[derive(Copy, Clone)]
pub struct PixelChange<'a> {
    ptype: &'a PixelStruct<'a>,
    bits: f64,
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

#[derive(Copy, Clone)]
pub struct Pixel<'a> {
    pub pixel_type: &'a PixelStruct<'a>,
    bits: f64,
    pub sim_type: SimType,
    pub perfect_position: Vec2Float,
    pub priority: i32,
    pub simulate: bool,
    pub tempature: f64,
    pub changes: PixelChange<'a>
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
    fn new<'a>(m_pixel: &'a PixelStruct, products: Vec<Pixel>, changes: PixelChange<'a>, r_conditions: Conditions<'a>, ratios: Vec<Ratio<'a>>) -> Reaction<'a> {
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
    fn new<'a>(ptypes: Vec<&'a PixelStruct>, pos: Vec2Float, stypes: Vec<SimType>) -> Pixel<'a> {
        Pixel { 
            pixel_type: ptypes[0], 
            bits: 0.0,
            sim_type: SimType::Dust, 
            perfect_position: pos,
            priority: 0,
            simulate: false,
            tempature: 0.0,
            changes: PixelChange::new(ptypes, stypes)
        }
    }
}

impl PixelChange<'_> {
    fn new<'a>(pixels: Vec<&'a PixelStruct>, sim_type: Vec<SimType>) -> PixelChange<'a> {
        PixelChange {
            ptype: pixels[1],
            bits: 0.0,
            stype: sim_type[0],
            temp_change: 0.0,
        }
    }
}

pub fn coerce<'a, T>(_desired_life: &'a i8, coerced_life: &'a T) -> &'a T {
    coerced_life
}

pub fn cycle() {
     //reactions
    let Pixels: Vec<PixelStruct>;
    let mut state = State::new(vec![]);
    let mut sim_areas = vec![];
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
                pioritize_pixels(&mut sim_areas);
                calculate_pixel_changes(&mut state);
                calculate_colisions();
                finalize_changes(&mut state);
                break;
            },
            Menus::InGame => {
                in_game = true;
            },
            _ => {}
        }
    }
}


fn pioritize_pixels(areas: &mut Vec<Map>,) {
    //make changes for all maps
    for maps in areas {
        //calculate and store priority
        let mut priority_total: i32 = 0;
        for pixel in &maps.area {
            //measure total priority
            priority_total += pixel.priority;
        }
        //set simulation on or off
        for pixel in &mut maps.area {
            
            let priority = priority_total / (SCREEN_HEIGHT * SCREEN_WIDTH) as i32;

            if pixel.priority >= priority {
                pixel.simulate = true;
            }
            else {
                pixel.simulate = false
            }
        }
    }
}



fn calculate_pixel_changes(state: &mut State) {
    let mut maps = &mut state.simulated_area;
    for map_num in 0..maps.len() {
        let mut map = &mut maps[map_num]; //Current map
        for p_num in 0..map.area.len() - 1{
            
            //heat changes
            let mut pixel = map.area[p_num]; //the pixel
            if pixel.simulate {
                
                    //indent for ownership then find pixels and heat change
                let scoped_pixels = find_nearby_pixels(p_num); //Future changes need to made for more "diverse" maps
                
                for nearby_pixel in &scoped_pixels {
                    
                    let near_pixel = &mut map.area[*nearby_pixel]; //set to scanned pix
                    let heat_difference = (near_pixel.tempature - pixel.tempature)/2.0;
                    
                    
                    near_pixel.changes.temp_change += -heat_difference; //scanned pix
                    

                    pixel.changes.temp_change += heat_difference; //original pixel

                }


                for reaction in &pixel.pixel_type.reactions { //get reactions

                    let mut ratio_requirements: Vec<(f64, Vec<&Pixel>)> = vec![]; //ajusted ration measurements

                    for compared_pixel_num in &scoped_pixels {
                        let compared_pix = &map.area[*compared_pixel_num];
                        for ratio in &reaction.ratio  {
                            if compared_pix.pixel_type.id == ratio.pixel_type.id {
                                ratio_requirements.push((compared_pix.bits / ratio.ratio_num as f64, vec![compared_pix]));
                            }
                        }
                    }
                }
            }
            map.area[p_num] = pixel;
        }
    }
}

fn calculate_colisions() {

}

fn finalize_changes<'a>(state: &'a mut State<'a>) {
    let mut maps = &mut state.simulated_area; //define the maps
    let pixels = &mut state.pixel_types;
    for map in maps {
        for pixel in &mut map.area {
            //change type
            match pixel.changes.ptype {
                _ => {
                    pixel.pixel_type = &pixel.changes.ptype;
                    pixel.changes.ptype = &pixels[0];
                }
            }
            //change simulation type
            match pixel.changes.stype {
                SimType::None => {}
                _ => {
                    pixel.sim_type = pixel.changes.stype;
                    pixel.changes.stype = SimType::None;
                }
            }
            //Do tempature changes
            pixel.tempature += pixel.changes.temp_change;
            pixel.changes.temp_change = 0.0;

            //Do position changes
        }
    }
}