#![allow(clippy::cast_lossless)]
#![feature(proc_macro_hygiene, decl_macro)]

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
mod gfx_system;
mod item_api;

use crate::gamestate::*;
use crate::constants::*;
use crate::gfx_system::*;
use crate::item_api::*;
use std::thread;
use byteorder::{ByteOrder, LittleEndian};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

fn main() {
    let warp_enable = Arc::new(Mutex::new(true));
    let enable_bypass = Arc::new(Mutex::new(false));

    let mut cycle = 0u64;

    let mut old_time = Instant::now();

    println!("Parsing Symfile");

    println!("wItemAPICommand: 0x{:X}", SYM["wItemAPICommand"].bus_addr); // use the sym table to make sure it gets built now and not later (builds on first use)

    println!("");

    let mut gfx = GfxSystem::new(Arc::clone(&warp_enable), Arc::clone(&enable_bypass));

    thread::Builder::new()
        .name("Gui".to_string())
        .spawn(move || {
            gfx.run();
        })
        .expect("error: failed to start gui");

    thread::Builder::new()
        .name("ItemApi".to_string())
        .spawn(move || {
            start_item_api();
        })
        .expect("error: failed to start ItemApi");

    
    let mut game_state = GameState::from_file().unwrap();

    game_state.get_current_game();
    game_state.collect_mapstate();

    if game_state.game == gamestate::Game::RED {
        BIZHAWK.remove_callback("item_api").ok();
        BIZHAWK.on_memory_write("item_api", SYM["wItemAPICommand"].bus_addr as u32, 0x04, "http://localhost:5340/item_api").ok();
    }

    game_state.send_hud_data();
    game_state.hud_enable(true);

    // Frame timing
    let mut current_frame = BIZHAWK.framecount().unwrap();

    loop { 
        if game_state.enabled || *enable_bypass.lock().unwrap() {
            if let Ok(frame) = BIZHAWK.framecount() {
                if current_frame != frame {
                    current_frame = frame;
                    //println!("{:?}", frame);
                    game_state.check_for_transition(current_frame, Arc::clone(&warp_enable));
                }
            }
        } else if game_state.game == gamestate::Game::FIRERED {
            game_state.collect_mapstate();
            // Keep the system disabled until we get oaks parcel 0x015D
            if LittleEndian::read_u16(&BIZHAWK.read_slice_custom("*03005008+3b8/2".to_string(), 0x02).unwrap()) == 0x015D { // if first slot is oaks parcel
                println!("Enabling system");
                game_state.read_trainer_data();
                game_state.enabled = true;
                *OAKS_PARCEL_OBTAINED.lock().unwrap() = true;
            }
        } else {
            game_state.collect_mapstate();
        }

        if cycle % 200 == 0 {
            game_state.recurring_functions();
        }

        if cycle % 60000 == 59999 { // 5 minutes
            make_backup(false);
        }

        cycle += 1;
        let new_time = Instant::now();

        if old_time + POLL_DELAY > new_time {
            //println!("{:?}", (POLL_DELAY - (new_time - old_time)));
            thread::sleep(POLL_DELAY - (new_time - old_time));
        }
        old_time = new_time;
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