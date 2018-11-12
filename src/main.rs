extern crate reqwest;
extern crate strum;
#[macro_use] extern crate strum_macros;
extern crate bit_field;
extern crate bidir_map;

mod bizhawk;
mod warp_connections;
mod constants;

#[derive(PartialEq,Eq)]
enum Game {
    RED,
    FIRERED
}

use crate::Game::*;

fn main() {

    // Game / Warp map
    let mut game = Game::RED;
    let bizhawk = bizhawk::Bizhawk::new(5337);
    let warp_mapping = warp_connections::get_connections();

    // Map change detection
    let mut previous_map: u16 = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x135E).unwrap() as u16;
    let mut previous_warp: u8 = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x142F).unwrap() as u8;
    let mut map_checked = false;

    // Frame timing
    let mut current_frame = bizhawk.framecount().unwrap();
    let mut last_change = current_frame;
    
    loop { 
        if current_frame != bizhawk.framecount().unwrap() {

            current_frame = bizhawk.framecount().unwrap();

            match game {
                RED => {

                    let last_map = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x1365).unwrap();
                    let current_map = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x135E).unwrap();
                    let current_warp = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x142F).unwrap();

                    //println!("map: {:02x}, {:02x}, warp:{:02x}, {:02x}", current_map, previous_map, current_warp, previous_warp);
                    if current_map as u16 != previous_map || (current_warp != previous_warp) {
                        last_change = bizhawk.framecount().unwrap();
                        previous_map = current_map as u16;
                        previous_warp = current_warp;
                        map_checked = false;
                    }

                    //println!("{:02x}, {:02x}", current_frame, last_change);
                    if !map_checked && current_frame - last_change > 18 { // wait 18 frames
                        if let Some(destination) = warp_mapping.get_by_first(&(current_map, current_warp, last_map)) {
                            println!("{:02x}, {:02x}, {:02x} -> {:04x}, {:02x}", current_map, current_warp, last_map, destination.0, destination.1);
                            bizhawk.pause().unwrap(); 

                            bizhawk.load_rom("firered.gba").unwrap();
                            bizhawk.load_state("firered_warp").unwrap();

                            bizhawk.write_u16(bizhawk::MemRegion::EWRAM, 0x031DBC, destination.0).unwrap();
                            bizhawk.write_u8(bizhawk::MemRegion::EWRAM, 0x031DBE, destination.1).unwrap();

                            bizhawk.play().unwrap();

                            previous_map = destination.0;
                            previous_warp = destination.1; 

                            last_change = bizhawk.framecount().unwrap();

                            game = Game::FIRERED;
                        } else {
                            println!("not detected, {:02x}, {:02x}, {:02x}", current_map, current_warp, last_map);
                        }
                        map_checked = true;
                        continue;
                    }

                    if  current_warp != 0xFF && current_frame - last_change > 120 {
                        bizhawk.write_u8(bizhawk::MemRegion::WRAM, 0x142F, 0xFF).unwrap();
                    }
                },
                FIRERED => {

                    let current_map = bizhawk.read_u16(bizhawk::MemRegion::EWRAM, 0x031DBC).unwrap();
                    let current_warp = bizhawk.read_u8(bizhawk::MemRegion::EWRAM, 0x031DBE).unwrap();

                    //println!("map: {:02x}, {:02x}, warp:{:02x}, {:02x}", current_map, previous_map, current_warp, previous_warp);
                    if current_map != previous_map || (current_warp != previous_warp) {
                        last_change = bizhawk.framecount().unwrap();
                        previous_map = current_map as u16;
                        previous_warp = current_warp;
                        map_checked = false;
                    }

                    //println!("{:02x}, {:02x}", current_frame, last_change);
                    if !map_checked && current_frame - last_change > 36 {
                        if let Some(destination) = warp_mapping.get_by_second(&(current_map, current_warp)) {
                            println!("{:04x}, {:02x} -> {:02x}, {:02x}, {:02x}", current_map, current_warp, destination.0, destination.1, destination.2);
                            bizhawk.pause().unwrap();
                            //bizhawk.mute();
                            bizhawk.load_rom("red.gbc").unwrap();
                            bizhawk.load_state("red_warp").unwrap();

                            last_change = bizhawk.framecount().unwrap();

                            bizhawk.write_u8(bizhawk::MemRegion::WRAM, 0x1365, destination.2).unwrap(); // lastmap
                            bizhawk.write_u8(bizhawk::MemRegion::WRAM, 0x13B1, destination.1).unwrap(); // warp
                            bizhawk.write_u8(bizhawk::MemRegion::WRAM, 0x13B2, destination.0).unwrap(); // map

                            bizhawk.play().unwrap();
                            bizhawk.unthrottle().unwrap();
                            //bizhawk.stop_drawing().unwrap();
                            //bizhawk.clear_screen(0x00000000).unwrap();

                            while bizhawk.framecount().unwrap() - last_change < 36 + 45 {}; // wait 45 frames

                            //bizhawk.start_drawing().unwrap();
                            bizhawk.throttle().unwrap();

                            previous_map = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x135E).unwrap() as u16;
                            previous_warp = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x142F).unwrap();

                            last_change = bizhawk.framecount().unwrap();

                            game = Game::RED;
                        } else {
                            println!("not detected, {:02x}, {:02x}", current_map, current_warp);
                        }
                        map_checked = true;
                    }
                }
            }
        }
    }
}

/*
Fire red
EWRAM 
0x031DBC = map bank
0x031DBD = map number
0x031DBE = which entrance to use // if ff, its a map transition 

Red
WRAM
0x1365 = last map
0x135E = current map
0x13B1 = which entrance to use
0x13B2 = which map to go to (0xFF is last map)
0x142F = current warp desination ID
*/