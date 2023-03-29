use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum MapType {
    Full,
    Player,
    Random
}

 pub struct Map<'a> {
    pub stype: MapType,
    pub area: [Pixel<'a>; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize]
} 

impl Map<'_> {
     pub fn create_map(map_type: MapType, map_area: [Pixel; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize]) -> Map {
        Map { 
            stype: map_type, 
            area: map_area 
        }
     }
}


pub fn index_pixel<'a>(point: Vec2Integer, map: &'a Map) -> &'a Pixel<'a> {
    &map.area[(point.x * point.y) as usize]
} 

pub fn in_bounds(point: Vec2Integer) -> bool {
    let idx: i64 = point.x * (SCREEN_HEIGHT) as i64 + point.y;
    
    idx >= 0
    && idx <= (SCREEN_HEIGHT * SCREEN_WIDTH).into() 
}

pub fn find_nearby_pixels(pos: usize) -> Vec<usize> { //Needs map size inputs
    let mut pixels: Vec<usize> = vec![];
    for scan_x in 0..1 {
        for scan_y in -1..1 {
            if scan_x != 0 || scan_y != 0{
                if pos as i64 + (scan_x * SCREEN_HEIGHT) + scan_y > (SCREEN_HEIGHT * SCREEN_WIDTH) {
                    pixels.push(pos + (scan_y + (scan_x * SCREEN_HEIGHT)) as usize);
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