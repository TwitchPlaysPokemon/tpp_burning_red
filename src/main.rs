#![feature(plugin)]
#![plugin(phf_macros)]
#![allow(clippy::cast_lossless)]

extern crate phf;
extern crate reqwest;
extern crate strum;
#[macro_use] extern crate strum_macros;
extern crate bit_field;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod bizhawk;
mod warp_connections;
mod constants;
mod pokemon;
mod gamestate;

use crate::gamestate::*;
use crate::constants::*;
use byteorder::{ByteOrder, LittleEndian};

fn main() {

    // Game / Warp map
    let bizhawk = bizhawk::Bizhawk::new(5337);

    let mut game_state = GameState::from_file(&bizhawk).unwrap();

    game_state.get_current_game(&bizhawk);
    game_state.collect_mapstate(&bizhawk);

    // Frame timing
    let mut current_frame = bizhawk.framecount().unwrap();

    // keep the system enabled for now
    game_state.enabled = true;

    loop { 
        if game_state.enabled {
            if let Ok(frame) = bizhawk.framecount() {
                if current_frame != frame {
                    current_frame = frame;
                    //println!("{:?}", frame);
                    game_state.check_for_transition(&bizhawk, current_frame);
                }
            }
        } else if game_state.game == gamestate::Game::FIRERED {
            // Keep the system disabled until we get oaks parcel
            if LittleEndian::read_u16(&bizhawk.read_slice_custom("*03005008+1000/AA".to_string(), 0x02).unwrap()) == 0x05 {
                println!("Oaks parcel obtained, enabling system");
                game_state.collect_mapstate(&bizhawk);
                game_state.read_trainer_data(&bizhawk);
                game_state.enabled = true;
            }
        }
        std::thread::sleep(POLL_DELAY);
    }
}

/*
Fire red
EWRAM 
0x031DBC = map bank
0x031DBD = map number
0x031DBE = which entrance to use // if ff, its a map transition 

0x024284 6 100 byte structures = party

Red
WRAM
0x1365 = last map
0x135E = current map
0x13B1 = which entrance to use
0x13B2 = which map to go to (0xFF is last map)
0x142F = current warp desination ID <-- next warp ID
0x1163 = current party count
0x116B = First Mon Start 44 / 0x2c size
0x12B5 = Nickname for first mon, 0xB len
*/

/*
println!("Address 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
for i in 0..0x10 {
    print!("{:07X}", i);
    for j in 0..0x10 {
        print!(" {:02X}", flags[(i*0x10)+j]);
    }
    println!("");
}*/

/*
bit 0-1 = text speed (0 = SLOW, 1 = MED, 2 = FAST)

bit 3-6 = window frame (0-9)

bit 8 = sound (0 = MONO, 1 = STEREO)
bit 9 = battle style (0 = SHIFT, 1 = SET)
bit 10 = battle animations (0 = ON, 1 = OFF)
*/