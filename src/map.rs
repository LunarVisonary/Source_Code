use crate::prelude::*;
use crate::SmartPointer::{Sr};
use std::mem::{self, MaybeUninit};

#[derive(Copy, Clone, PartialEq)]
pub enum MapType {
    Full,
    Player,
    Space {x: usize, y: usize},
    Random
}

 pub struct Map<'a> {
    pub stype: MapType,
    pub area: [Sr<Pixel<'a>>; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize]
} 

impl Map<'_> {
     pub fn create_map<'a>(map_type: MapType) -> Map<'a> {
        Map { 
            stype: map_type, 
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


pub fn index_pixel<'a>(point: Vec2Integer, map: &'a mut Map<'a>) -> &'a mut Pixel<'a> {
    map.area[(point.x * point.y) as usize].get_mut()
} 

pub fn in_bounds(point: Vec2Integer) -> bool {
    let idx: usize = point.x * (SCREEN_HEIGHT as usize) + point.y;
    
    idx >= 0
    && idx <= (SCREEN_HEIGHT * SCREEN_WIDTH) as usize
}

pub fn find_nearby_pixels<'a>(pos: usize, map: &mut Map<'a>) -> Vec<Sr<Pixel<'a>>> { //Needs map size inputs
    let mut pixels: Vec<Sr<Pixel>> = vec![];
    for scan_x in 0..1 {
        for scan_y in -1..1 {
            if scan_x != 0 || scan_y != 0{
                if pos as i64 + (scan_x * SCREEN_HEIGHT) + scan_y > (SCREEN_HEIGHT * SCREEN_WIDTH) {
                    pixels.push(map.area[pos + (scan_y + (scan_x * SCREEN_HEIGHT)) as usize].clone());
                }
            }
        }
    }
    pixels
}

pub fn find_adjacent_pixels(pos: usize) -> Vec<usize> {
    let mut adjacent_pixels: Vec<usize> = vec![];
    for scan_x in -1..1 {
        for scan_y in -1..1 {
            let place = pos + ((scan_x * SCREEN_HEIGHT) + scan_y) as usize;
            if place > (SCREEN_HEIGHT * SCREEN_WIDTH) as usize {
                if scan_x != 0 || scan_y != 0 {
                    adjacent_pixels.push(place)
                }
            }
        }
    }
    adjacent_pixels
}

fn number_to_coordinate(num: usize) -> Vec2Integer {
    let y = num % SCREEN_WIDTH as usize;
    let x = (num - y) / SCREEN_HEIGHT as usize;
    Vec2Integer {x: x, y: y}
}  