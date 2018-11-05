extern crate reqwest;
extern crate strum;
#[macro_use] extern crate strum_macros;
extern crate bit_field;
use bit_field::BitArray;

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
    let mut previous_map: u16 = 0x26;
    let mut game = Game::RED;
    let bizhawk = bizhawk::Bizhawk::new(5337);
    bizhawk.load_rom("red.gbc");
    bizhawk.load_state("red_bedroom");

    loop {
        match game {
            RED => {
                /*if bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x135E).unwrap() as u16 != previous_map {
                    let framecount_begin = bizhawk.framecount().unwrap();
                    loop {
                        if bizhawk.framecount().unwrap() - framecount_begin > 18 {
                            bizhawk.pause();
                            bizhawk.load_rom("firered.gba");
                            bizhawk.load_state("firered_warp");
                            bizhawk.write_u32(bizhawk::MemRegion::EWRAM, 0x031DBC, 0x00020004);
                            bizhawk.play();
                            game = Game::FIRERED;
                            break;
                        }
                    }
                    previous_map = bizhawk.read_u16(bizhawk::MemRegion::EWRAM, 0x031DBC).unwrap();
                }*/
            },
            FIRERED => {
                if bizhawk.read_u16(bizhawk::MemRegion::EWRAM, 0x031DBC).unwrap() as u16 != previous_map {
                    /*let mut framecount_begin = bizhawk.framecount().unwrap();
                    loop {
                        if bizhawk.framecount().unwrap() - framecount_begin > 36 {
                            bizhawk.pause();
                            bizhawk.load_rom("red.gbc");

                            game = Game::RED;

                            bizhawk.load_state("red_warp");
                            bizhawk.write_u16(bizhawk::MemRegion::WRAM, 0x13B1, 0x2600);


                            framecount_begin = bizhawk.framecount().unwrap();

                            bizhawk.play();
                            bizhawk.stop_drawing();
                            bizhawk.clear_screen(0x00000000);

                            while bizhawk.framecount().unwrap() - framecount_begin < 18 + 17 {
                                //println!("{}", bizhawk.framecount().unwrap() - framecount_begin);
                            };

                            bizhawk.start_drawing();
                            break;
                        }
                    }
                    previous_map = bizhawk.read_u8(bizhawk::MemRegion::WRAM, 0x135E).unwrap() as u16;*/
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