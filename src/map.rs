use crate::prelude::*;
use crate::smartpointer::{Sr};
use std::mem::{self, MaybeUninit};

#[derive(Copy, Clone, PartialEq)]
pub enum MapType {
    Planet,
    Space {velocity: Vec2Float, position: Vec2Float},
}

pub enum PriorityType {
    Full(f64),
    Random
}

 pub struct Map {
    pub stype: MapType,
    pub priority: PriorityType,
    pub area: [Sr<Pixel>; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize]
} 

impl Map {
     pub fn create_map(map_type: MapType) -> Map {
        Map { 
            stype: map_type,
            priority: PriorityType::Full(0.0), 
            area: {
                let mut x: [MaybeUninit<Sr<Pixel>>; SIZE] = unsafe {
                    MaybeUninit::uninit().assume_init()
                };
            
                for i in 0..SIZE {
                    x[i] = MaybeUninit::new(Sr::new(Pixel::default()));
                }
            
                
                unsafe { mem::transmute::<_, [Sr<Pixel>; SIZE]>(x) }
            }
        }
     }
}


pub fn index_pixel(point: Vec2Integer, map: &mut Map) -> &mut Pixel {
    map.area[(point.x * point.y) as usize].get_mut()
} 

pub fn in_bounds_vec(point: Vec2Float) -> bool {
    let idx: f64 = point.x * (SCREEN_HEIGHT) as f64 + point.y;
    
    idx >= 0.0
    && idx <= (SCREEN_HEIGHT * SCREEN_WIDTH) as f64
}

pub fn in_bounds(point: i64) -> bool {
    point >= 0 && point <= SCREEN_HEIGHT * SCREEN_WIDTH
}

pub fn find_nearby_pixels(pos: usize, map: &mut Map) -> Vec<Sr<Pixel>> { //Needs map size inputs
    let mut pixels: Vec<Sr<Pixel>> = vec![];
    for scan_x in 0..1 {
        for scan_y in -1..1 {
            let point = pos as i64 + (scan_x * SCREEN_HEIGHT) + scan_y;
            if scan_x != 0 || scan_y != 0 && in_bounds(point) {
                pixels.push(map.area[point as usize].clone());
            }
        }
    }
    pixels
}

pub fn find_adjacent_pixels(map: &mut Map, pos: usize) -> Vec<Sr<Pixel>> {
    let mut adjacent_pixels: Vec<Sr<Pixel>> = vec![];
    for scan_x in -1..1 {
        for scan_y in -1..1 {
            let place = pos as i64 + ((scan_x * SCREEN_HEIGHT) + scan_y);
            if scan_x != 0 || scan_y != 0 && in_bounds(place) {
                adjacent_pixels.push(map.area[place as usize].clone());
            }
        }
    }
    adjacent_pixels
}

fn number_to_coordinate(num: usize) -> Vec2Integer {
    let y = num as i64 % SCREEN_WIDTH;
    let x = (num as i64 - y) / SCREEN_HEIGHT;
    Vec2Integer {x: x, y: y}
}