use crate::prelude::*;
use crate::smartpointer::Sr;
use  std::{time::{Duration, Instant}, cmp::min};
use std::f64::consts::PI;

type CellestialBody = (Vec2Float, f64, f64);

pub const SCREEN_WIDTH: i64 = 5;
pub const SCREEN_HEIGHT: i64 = 3; 
pub const  SIZE: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
const NINETY_DEGREES: f64 = PI / 2.0; 

static mut DELTA: f64 = 1.0;

#[derive(Copy, Clone)]
enum Menus {
    MainMenu,
    SaveSelector,
    Quit,
    InGame
} 

#[derive(Clone, Copy)]
pub enum SimType {
    Dust,
    Fluid,
    Solid(Option<(usize, usize)>),
    None
}

struct TypeBendData {
    strength: f64,
    rigidity: f64,
    relasticity: f64
}

struct PixelStructureRef {
    pixel: *mut Pixel,
    connections: Vec<usize>,
    bstrength: f64,
    bendcoe: f64,
    connectedtoroot: bool,
}

struct PixelStructure {
    pixels: Vec<Option<PixelStructureRef>>,
    size: Vec2Integer,
    root: usize,
    velocity: Vec2Float,
    velocity_change: Vec2Float,
    rotation: f64,
    rotation_change: f64,
    center_of_mass: Vec2Float,
    mass: f64,
    angular_mass: f64
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
    sim_type: SimType,
    reactions: Vec<Reaction>,
    strength: Option<TypeBendData>,
    specificheat: f64,
    pressure_line: (f64, f64),
}

struct Ratio {
    pixel_type: usize,
    ratio_num: i32,
    change: PixelChange,
    type_change: TypeChanges,
    conditions: Conditions
}

#[derive(Clone)]
pub struct Conditions {
    heat: [f64; 2], 
}

pub struct Reaction {
    main_pixel: usize,
    ratio: Vec<Ratio>,
    ratio_total: i32
}

pub struct PixelChange {
    stype: SimType,
    temp_change: f64,
}

#[derive(Copy, Clone, PartialEq)]
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
    velocity: Vec2Float,
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
    pub pixel_types: Vec<PixelStruct>,
    structures: Vec<PixelStructure>,
    cellestial_bodies: Vec<CellestialBody>,
    slider: usize
}

impl PixelStructure { //ALSO UNFINISHED
    fn new(size: Vec2Integer, mut pixels: Vec<Option<PixelStructureRef>>, structures: &mut Vec<PixelStructure>) {
        let mut root: usize = 0;
        let mut tmass: f64 = 0.0;
        for pixelref in pixels.iter().enumerate() {
            match pixelref.1 {
                None => {}
                Some(pixref) => { //neeeds to check for multiple pixels or just one...
                    if root == 0 {
                        root = pixelref.0;
                    }
                    tmass += unsafe {(*pixref.pixel).total_bits};
                }
            }
        }
        if root > 0 {
            let mut structure = PixelStructure {pixels: pixels, size: size, root: root, velocity: {Vec2Float { x: 0.0, y: 0.0 }}, velocity_change: Vec2Float {x: 0.0, y: 0.0}, rotation: 0.0, rotation_change: 0.0, center_of_mass: Vec2Float { x: 0.0, y: 0.0 }, mass: tmass, angular_mass: 0.0}; //NEEDS FINISHING
            structure.checkconnection(structures);
            structure.find_com_and_am();
            structures.push(structure);
        } 
    }

    fn checkconnection(&mut self, listofstructures: &mut Vec<PixelStructure>) { //Finished
        let mut unfinished = true;
        let mut pixelstocheck = vec![self.root];
        let mut pixelstostage: Vec<usize> = vec![];
        while unfinished {
            for pixel in pixelstocheck.drain(..) {
                let mut pixelref = match &mut self.pixels[pixel] {
                    Some(pixel) => pixel,
                    None => panic!("invalid structure (specifically false connection)"),
                };
                pixelref.connectedtoroot = true;
                for connection in pixelref.connections.iter() {
                    pixelstostage.push(*connection);
                }
            }
            if pixelstostage.is_empty() {
                unfinished = false;
            } else {
                pixelstocheck = pixelstostage;
                pixelstostage = vec![];
            }
        }
        let mut discconect: bool = false;
        let mut newstruct: Vec<Option<PixelStructureRef>> = vec![];
        let mut count: usize = 1;
        for maybepixelref in self.pixels.iter_mut() {
            match maybepixelref {
                Some(pixelref) => {
                    if !pixelref.connectedtoroot {
                        self.mass += -unsafe {(*pixelref.pixel).total_bits};
                        newstruct.push(Some(PixelStructureRef { pixel: pixelref.pixel, connections: {let mut connections: Vec<usize> = vec![]; for connection in pixelref.connections.drain(..) {connections.push(connection);} connections}, bstrength: pixelref.bstrength, bendcoe: pixelref.bendcoe,connectedtoroot: false}));
                        *maybepixelref = None;
                        discconect = true;
                    }
                },
                None => {newstruct.push(None);}
            }
            count += 1;
        }
        if discconect {
            PixelStructure::new(self.size, newstruct, listofstructures);
        }
    }


    fn find_com_and_am(&mut self) {
        let mass = self.mass;
        let mut com = Vec2Float {x: 0.0, y: 0.0};
        let mut angular_mass = 0.0;
        for maybepix in self.pixels.iter().enumerate() {
            if let Some(pixelref) = maybepix.1 {
                let pos = Vec2Integer::number_to_coordinate(maybepix.0);
                let x_y_distance = Vec2Float {x: pos.x as f64, y: pos.y as f64}.subtract(com);
                com = com.add(x_y_distance.multiply_by_scalar(unsafe {(*pixelref.pixel).total_bits}).divide_by_scalar(mass));
            }
        }
        for maybepix in self.pixels.iter().enumerate() {
            if let Some(pixelref) = maybepix.1 {
                let coordinate = Vec2Integer::number_to_coordinate(maybepix.0);
                let fpos = Vec2Float {x: coordinate.x as f64, y: coordinate.y as f64};
                let distance = fpos.subtract(com).distance();
                angular_mass += unsafe {(*pixelref.pixel).total_bits} * distance;
            }
        }
        self.angular_mass = angular_mass;
        self.center_of_mass = com;
    }
}

impl Vec2Integer {
    fn to_one(&self) -> i64 {
        self.x * SCREEN_WIDTH + self.y
    }
    
    fn number_to_coordinate(num: usize) -> Vec2Integer {
        let y = num as i64 % SCREEN_WIDTH;
        let x = (num as i64 - y) / SCREEN_HEIGHT;
        Vec2Integer {x: x, y: y}
    }

    fn equals(&self, second: Vec2Integer) -> bool {
        self.x == second.x && self.y == second.y
    }
}

impl Ratio {
    fn new(pixel: usize, ratio: i32, overchange: PixelChange, tchange: TypeChanges, temp: [f64; 2]) -> Ratio {
        Ratio { 
            pixel_type: pixel, 
            ratio_num: ratio,
            change: overchange,
            type_change: tchange,
            conditions: Conditions { heat: temp }
        }
    }
}

impl Conditions {
    fn new(heat_values: [f64; 2]) -> Conditions {
        Conditions {
            heat: heat_values,
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
            pixel_types: pixels,
            structures: vec![],
            cellestial_bodies: vec![(Vec2Float {x: 10.0,y: -100.0}, 50.0, 5.0)],
            slider: 0
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
            velocity: Vec2Float { x: 0.0, y: 0.0 },
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
            velocity: Vec2Float { x: 0.0, y: 0.0 },
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

impl Vec2Float {
    fn floor_to_int(&self) -> Vec2Integer {
        Vec2Integer {x: self.x as i64, y: self.y as i64}
    }

    fn distance(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    fn perpendicular(&self) -> Vec2Float {
        Vec2Float { x: self.y, y: -self.x }
    }

    fn add(&self, second: Vec2Float) -> Vec2Float {
        Vec2Float { x: self.x + second.x, y: self.y + second.y }
    }

    fn subtract(&self, second: Vec2Float) -> Vec2Float {
        Vec2Float { x: self.x - second.x, y: self.y - second.y }
    }

    fn slope(&self) -> f64 {
        self.y / self.x
    }

    fn divide_by_scalar(&self, second: f64) -> Vec2Float {
        Vec2Float { x: self.x / second, y: self.y / second }
    }

    fn multiply_by_scalar(&self, second: f64) -> Vec2Float {
        Vec2Float { x: self.x * second, y: self.y * second }
    }

    fn dot_product(&self, second: &Vec2Float) -> f64 {
        let divisor = (self.x * second.x) + (self.y * second.y);
        (divisor / (self.distance() * second.distance())).acos()
    }
}

fn minf64(f1: f64, f2: f64) -> f64 {
    if f1 > f2 {
        f2
    } else {
        f1
    }
}

fn absf64(i: f64) -> f64 {
    if i < 0.0 {
        -i
    } else {
        i
    }
}

fn get_sign_as_onef64(i: f64) -> f64 {
    i / absf64(i)
}

pub fn cycle() {
    //reactions
    let pixels: Vec<PixelStruct> = vec![PixelStruct {pixel: String::from("NONE"), reactions: vec![], sim_type: SimType::None, strength: None, specificheat: 0.0, pressure_line: (0.0, 0.0)}];
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
                calculate_collisions(mutable);
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
    for map in areas {
        //calculate and store priority
        match map.priority {
            PriorityType::Full(x) => {
                let mut priority_total: i32 = 0;
        for smrt_ref in &map.area {
            //measure total priority
            priority_total += smrt_ref.get_immut().priority;
        }
        //set simulation on or off
        for smrt_ref in &mut map.area {
            
            let priority = priority_total / (SCREEN_HEIGHT * SCREEN_WIDTH) as i32;
            let pixel = smrt_ref.get_mut();
            if pixel.priority >= priority {
                pixel.simulate = true;
            }
            else {
                pixel.simulate = false;
            }
        }
            },
            PriorityType::Random => {
                for mut pixel_num in 0..map.area.len() {
                    let pixel = map.area[pixel_num].get_mut();
                    if pixel.priority > 0 {
                        pixel.simulate = true;
                    }
                }
            }
        }
    }
}

fn calculate_pixel_changes(state: &mut State) {
    for map in state.simulated_area.iter_mut() { //
        for enumeration in 0..SIZE {
            let mut pixel = map.area[enumeration].clone(); //get pixel
            if pixel.get_immut().simulate { //look for sim
                { //HEAT SYSTEM
                    let mut scanned_pixels = find_nearby_pixels(enumeration, map); //find nearby pixels (4 pixels)
                    let sim_heat = pixel.get_immut().tempature; //get simulated pixels heat
                    for mut scanned_pixel in scanned_pixels.drain(..) { //drain it
                        let heat_diff = (scanned_pixel.get_immut().tempature - sim_heat)/16.0; //find hear difference and apply changes
                        pixel.get_mut().changes.temp_change += heat_diff; //change iterated pixel
                        scanned_pixel.get_mut().changes.temp_change += heat_diff; //change scanned pixel
                    }
                }
                let mut adjacent_pixels = find_adjacent_pixels(map, enumeration); //find all adjacent pixels for reactions
                for ptype in 0..pixel.get_immut().types.len() { //GET TYPES
                    for reaction in state.pixel_types[ptype].reactions.iter_mut()  { //GET REACTIONS
                        let mut requirements: Vec<(f64, Vec<(*mut Pixel_types, f64)>)> = vec![(0.0, vec![]); reaction.ratio.len()]; //REQUIREMENTS FOR REACTION
                        for ratio in reaction.ratio.iter_mut().enumerate() { //GET RATIO/REQUIREMENT
                            for mut nearby_pixel in adjacent_pixels.drain(..) { //SEARCH FOR PIXEL REQUIRED
                                for scanned_ptype in 0..nearby_pixel.get_immut().types.len()  { //GET PTYPE FOR REQUIREMENT
                                    if scanned_ptype == ratio.1.pixel_type && ratio.1.conditions.heat[0] < nearby_pixel.get_immut().tempature && ratio.1.conditions.heat[1] > nearby_pixel.get_immut().tempature { //NEEDS PIXEL HEAT THINGS!!!
                                        let bits = nearby_pixel.get_immut().types[scanned_ptype].bits / ratio.1.ratio_num as f64;
                                        let reaction_amount = {
                                            if ratio.1.change.temp_change > 0.0 {
                                                minf64(bits, (ratio.1.conditions.heat[1] - nearby_pixel.get_immut().tempature) / ratio.1.change.temp_change) 
                                            } else {
                                                minf64(bits, (ratio.1.conditions.heat[0] - nearby_pixel.get_immut().tempature) / ratio.1.change.temp_change) 
                                            }
                                        };
                                        requirements[ratio.0].0 += reaction_amount;
                                        requirements[ratio.0].1.push((&mut nearby_pixel.get_mut().types[scanned_ptype] as *mut Pixel_types, reaction_amount));
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
                for cellestial_body in state.cellestial_bodies.iter() {
                    let position: Vec2Float = {
                        match map.stype {
                            MapType::Planet(pos) => {
                                Vec2Float {x: pos.x as f64, y: pos.y as f64}
                            },
                            
                            MapType::Space { velocity, position} => {
                                position
                            }
                        }
                    };
                    //GRAVITY TIME
                    let x_ydistance = Vec2Float {x: position.x - cellestial_body.0.x, y: position.y - cellestial_body.0.y};
                    let distance = x_ydistance.distance();
                    let pixel = pixel.get_mut();
                    if distance < cellestial_body.2 {
                        pixel.velocity = Vec2Float {x: pixel.velocity.x + (x_ydistance.x / distance) * (cellestial_body.1 / (distance * distance)), y: pixel.velocity.y + (x_ydistance.y / distance) * (cellestial_body.1 / (distance * distance))};
                    } else {
                        pixel.velocity = Vec2Float {x: pixel.velocity.x + ((x_ydistance.x / distance) * (cellestial_body.1 / (distance * distance)) * (distance / cellestial_body.2)), y: pixel.velocity.y}
                    }
                }
                //STRUCTURAL TIME
                let pix = pixel.get_mut();
                match pix.sim_type {
                    SimType::Solid(structure) => {
                        match structure {
                            Some(coordinates) => {
                                let structure = &mut state.structures[coordinates.0];
                                let vel = structure.velocity;
                                let mass = structure.mass;
                                structure.velocity_change = structure.velocity_change.add(Vec2Float {x: (pix.velocity.x - vel.x) * pix.total_bits / mass, y: ((pix.velocity.y - vel.y) * pix.total_bits) / mass});
                                let pixel = Vec2Integer::number_to_coordinate(coordinates.1);
                                let x_y_dis = structure.center_of_mass.subtract(Vec2Float { x: pixel.x as f64, y: pixel.y as f64 });
                                let (distance, angle) = (x_y_dis.distance(), (x_y_dis.slope()).atan());
                                let perp = x_y_dis.perpendicular();
                                let spin_vel = Vec2Float {x: structure.rotation * perp.x, y: structure.rotation * perp.y};
                                let adjusted_vel = pix.velocity.subtract(vel).subtract(spin_vel);
                                structure.rotation_change += {
                                    let multiplier = {
                                        let angle_diff = perp.dot_product(&adjusted_vel);
                                        if angle_diff > NINETY_DEGREES {
                                            -1.0
                                        } else {
                                            1.0
                                        }
                                    };
                                    ((((adjusted_vel.distance() * absf64(perp.slope().atan() - angle.cos()) * distance) / structure.angular_mass)) * multiplier)
                                };
                            },
                            None => {}
                        }
                    }
                    _ => {}
                }
            }                      
        }
    }
}

fn calculate_collisions(state: &mut State) {
    for structure in state.structures.iter_mut() {
        structure.velocity = structure.velocity.add(structure.velocity_change);
        structure.rotation += structure.rotation_change;
    }
    let simulated_pixels = [false; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize];
    for map in state.simulated_area.iter_mut() {
        for pixelpos in 0..map.area.len() {
            let mut pixelref = map.area[pixelpos].clone();
            let mut pixel_stack: Vec<Sr<Pixel>> = vec![pixelref.clone()];
            let mut pix_count = 1;
            let mut unfinished = true;
            {
                let pixel = pixelref.get_mut();
                match pixel.sim_type { 
                    SimType::Solid(opstructure) => {if let Some(structure) = opstructure {
                            let structureref = &mut state.structures[structure.0];
                            let pixpoint = Vec2Integer::number_to_coordinate(structure.1);
                                pixel.velocity = {
                                let x_y_dis = Vec2Float {x: structureref.center_of_mass.x - pixpoint.x as f64, y: structureref.center_of_mass.x - pixpoint.x as f64};
                                let perp = x_y_dis.perpendicular();
                                structureref.velocity.add(Vec2Float { x: perp.x * structureref.rotation, y: perp.y * structureref.rotation })
                            }; 
                        }
                    },
                _ => {}
                }
            }
            while unfinished {
                let pixel = pixel_stack[pix_count - 1].get_immut();
                let (new_pos, real) = {
                    let x_bigger = absf64(pixel.velocity.x) > absf64(pixel.velocity.y);
                    let mut loops: usize = 1;
                    let distance = (pixel.velocity.distance() / unsafe {DELTA}).floor() as usize;
                    let mut real_colision: bool = false;
                    let mut continue_phy = true;
                    let mut  new_pos: i64 = 0;
                    while continue_phy {
                        new_pos = if x_bigger { //could be more descript
                            pixel.perfect_position.add(pixel.velocity.divide_by_scalar((pixel.velocity.x / loops as f64))).floor_to_int().to_one()
                        } else {
                            pixel.perfect_position.add(pixel.velocity.divide_by_scalar((pixel.velocity.y / loops as f64))).floor_to_int().to_one()
                        };
                        loops += 1;
                        if loops <= distance && in_bounds(new_pos) {
                            match map.area[new_pos as usize].get_immut().sim_type {
                                SimType::None => {
                                    continue_phy = false;
                                },
                                _ => {
                                    continue_phy = false;
                                    real_colision = true;
                                }
                            }
                        } else {
                            continue_phy =false;
                        }
                    }
                    (new_pos, real_colision)
                };
                if real {
                    pixel_stack.push(map.area[new_pos as usize].clone());
                } else {
                    unfinished = false;
                }
            }
            unfinished = true;
            let mut count_down: i64 = (pixel_stack.len() -2) as i64;
            while unfinished {
                for collision in pixel_stack.len() - 1..pix_count {

                }
                count_down -1;
                unfinished = count_down >= 0;
            }
        }
    }
}

fn finalize_changes(state: &mut State) {
    let everything_else = state as *mut State;
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
                    _ => {
                        match pixel.changes.stype {
                            SimType::Solid(pos) => {
                                if let Some(x) = pos {
                                    state.structures[x.0].pixels[x.1] = None;
                                    state.structures[x.0].checkconnection(unsafe {&mut (*everything_else).structures});
                                }
                            }
                            _ => {}
                        }
                        pixel.sim_type = pixel.changes.stype;
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