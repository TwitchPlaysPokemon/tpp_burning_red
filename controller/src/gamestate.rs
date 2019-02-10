use crate::pokemon::*;
use crate::constants::*;
use crate::bizhawk::*;
use bincode;
use std::collections::HashMap;
use std::fs::File;
use rand;
use byteorder::{ByteOrder, LittleEndian, BigEndian};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub trainer: TrainerInfo,
    red_items: [Pocket;5],
    firered_items: [Pocket;4],
    options: Options,
    pub game: Game,
    money: u32,
    badges: [u8;2],
    seen: [u8;19], // 152 bits
    owned: [u8;19], // 152 bits
    party_uids: [u16;6], // For HashMap
    pokemon_list: HashMap<u16, Pokemon>,
    map_state: MapState,
    pub enabled: bool,
    pub first_warp: bool
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            trainer: TrainerInfo::new(),
            red_items: [Pocket::new(&[0x86, 0xA4, 0xAD, 0xA4, 0xB1, 0xA0, 0xAB, 0x50], P_GENR), // "General"
                        Pocket::new(&[0x8A, 0xA4, 0xB8, 0x7F, 0x88, 0xB3, 0xA4, 0xAC, 0xB2, 0x50], P_KEYI), // "Key Items"
                        Pocket::new(&[0x8F, 0xAE, 0xAA, 0xBA, 0x7F, 0x81, 0xA0, 0xAB, 0xAB, 0xB2, 0x50], P_BALL), // "Poké Balls"
                        Pocket::new(&[0x93, 0x8C, 0xB2, 0x7F, 0xF3, 0x7F, 0x87, 0x8C, 0xB2, 0x50], P_TMHM),// "TMs / HMs"
                        Pocket::new(&[0x8F, 0x82, 0x7F, 0x88, 0xB3, 0xA4, 0xAC, 0xB2, 0x50, 0x50, 0x50, 0x50, 0x50], P_PC)], // "PC Items"
            firered_items: [Pocket::new(&[0xFF], P_GENR), // We dont care about pocket name in FR
                            Pocket::new(&[0xFF], P_KEYI),
                            Pocket::new(&[0xFF], P_BALL),
                            Pocket::new(&[0xFF], P_TMHM)],
            options: Options::new(),
            game: Game::FIRERED,
            money: 0,
            badges: [0,0],
            seen: [0;19], // 152 bits
            owned: [0;19], // 152 bits
            party_uids: [0,0,0,0,0,0],
            pokemon_list: HashMap::new(),
            map_state: MapState::new(),
            enabled: false,
            first_warp: true
        }
    }

    pub fn from_file() -> std::io::Result<GameState> {
        if let Ok(f) = File::open("GameState") {
            println!("GameState found, loading from disk.\n");
            Ok(bincode::deserialize_from(f).unwrap())
        } else {
            println!("No existing GameState found, using default.\n");
            let mut default = GameState::new();
            default.get_current_game();
            default.read_trainer_data();
            default.collect_mapstate();
            Ok(default)
        }
    }

    pub fn delay_frames(&mut self, frames: u32, rewind: bool, paused: bool) {
        let start_delay = BIZHAWK.framecount().unwrap();

        if rewind { BIZHAWK.toggle_rewind(true).unwrap(); }
        if paused {
            BIZHAWK.play().unwrap();
        }
        if rewind {
            while start_delay - BIZHAWK.framecount().unwrap() < frames {
                std::thread::sleep(POLL_DELAY);
            }
            BIZHAWK.toggle_rewind(false).unwrap();
        } else {
            while BIZHAWK.framecount().unwrap() - start_delay < frames {
                std::thread::sleep(POLL_DELAY);
            }
        }
        if paused {
            BIZHAWK.pause().unwrap();
        }
    }

    pub fn save_state(&self) -> std::io::Result<()> {
        //println!("Saving GameState to disk.\n");
        let f = File::create("GameState")?;
        bincode::serialize_into(f, self).unwrap();
        Ok(())
    }

    pub fn get_current_game(&mut self) {
        self.game = if let Ok(name) = BIZHAWK.get_rom_name() {
            match name.as_str() {
                "red" => Game::RED,
                "firered" => Game::FIRERED,
                _ => panic!("Neither red nor firered loaded!")
            }
        } else {
            panic!("Failed to get rom name!");
        };
    }

    pub fn collect_mapstate(&mut self) {
        match self.game {
            Game::RED => {
                self.map_state = MapState {
                    previous_map: BIZHAWK.read_u8_sym(&SYM["wCurMap"]).unwrap() as u16,
                    previous_warp: BIZHAWK.read_u8_sym(&SYM["wDestinationWarpID"]).unwrap(),
                    previous_lastmap: 0,
                    map_checked: true,
                    last_change: 0
                }
            },
            Game::FIRERED => {
                self.map_state = MapState {
                    previous_map: BIZHAWK.read_u16(&MemRegion::EWRAM, 0x0003_1DBC).unwrap(),
                    previous_warp: BIZHAWK.read_u8(&MemRegion::EWRAM, 0x0003_1DBE).unwrap(),
                    previous_lastmap: BIZHAWK.read_u32(&MemRegion::EWRAM, 0x0003_1DB4).unwrap(),
                    map_checked: true,
                    last_change: 0
                }
            }
        };
    }

    // send the player back through the warp they just came from
    pub fn warp_rebound(&mut self, map: u16, warp: u8, last_map: u8) {
        BIZHAWK.pause().unwrap();
        match &self.game {
            Game::FIRERED => { 

                BIZHAWK.toggle_rewind(true).unwrap();
                BIZHAWK.play().unwrap();
                //BIZHAWK.unthrottle(0).unwrap();

                let mut entered_menu = false;
                let start_rewind = BIZHAWK.framecount().unwrap();

                loop {
                    let map_data = BIZHAWK.read_slice(&MemRegion::EWRAM, 0x0003_1DB0, 0x10).unwrap();
                    let xy = BIZHAWK.read_slice(&MemRegion::IWRAM, 0x0000_0014, 0x4).unwrap();
                    if xy[0] == 0x00 && xy[2] == 0x00 {entered_menu = true;}
                    if (LittleEndian::read_u16(&map_data[0x0C..0x0E]) != map || map_data[0x0E] != warp) && 
                        xy[0] % 0x10 == 0x00 && 
                        xy[2] % 0x10 == 0x08 && 
                        start_rewind - BIZHAWK.framecount().unwrap() > 10 {

                        if entered_menu {
                            self.delay_frames(30, true, false);
                        }
                        break;

                    }
                    std::thread::sleep(POLL_DELAY);
                }
                BIZHAWK.pause().unwrap();

                BIZHAWK.toggle_rewind(false).unwrap();
                //BIZHAWK.throttle().unwrap();

                let map_data = BIZHAWK.read_slice(&MemRegion::EWRAM, 0x0003_1DB0, 0x10).unwrap();

                self.map_state.previous_lastmap = LittleEndian::read_u32(&map_data[0x04..0x08]);
                self.map_state.previous_map = LittleEndian::read_u16(&map_data[0x0C..0x0E]);
                self.map_state.previous_warp = map_data[0x0E];

                BIZHAWK.play().unwrap();
            },
            Game::RED => {
                BIZHAWK.write_u8(&MemRegion::WRAM, 0x1365, last_map).unwrap(); // lastmap
                BIZHAWK.write_u8(&MemRegion::WRAM, 0x142F, warp).unwrap(); // destination warp
                BIZHAWK.write_u8(&MemRegion::HRAM, 0x000B, map as u8).unwrap(); // map
                BIZHAWK.write_u8(&MemRegion::WRAM, 0x135E, map as u8).unwrap(); // map

                BIZHAWK.play().unwrap();

                self.map_state.previous_map = BIZHAWK.read_u8(&MemRegion::WRAM, 0x135E).unwrap() as u16;
                self.map_state.previous_warp = BIZHAWK.read_u8(&MemRegion::WRAM, 0x142F).unwrap();

                self.map_state.last_change = BIZHAWK.framecount().unwrap();

                //println!("--DONE\n");
            }
        }
    }

    pub fn handle_map_change(&mut self, map: u16, warp: u8, last_map: u8, warp_enable: Arc<Mutex<bool>>) {

        BIZHAWK.pause().unwrap();

        //println!("--Reading party");

        self.read_party();

        self.read_misc();

        let warp_enable = warp_enable.lock().unwrap();

        if *warp_enable {
            match &self.game {
                Game::RED => { 
                    println!("Supported map, starting transition to FIRE RED map: {:04x}, warp: {:02x}", map, warp);

                    BIZHAWK.load_rom("firered.gba").unwrap();
                    BIZHAWK.load_state("firered_warp").unwrap();

                    self.game = Game::FIRERED;

                    self.write_trainer_data();

                    self.write_party();

                    self.write_misc();

                    BIZHAWK.write_u16(&MemRegion::EWRAM, 0x0003_1DBC, map).unwrap();
                    BIZHAWK.write_u8(&MemRegion::EWRAM, 0x0003_1DBE, warp).unwrap();

                    BIZHAWK.frameadvance().unwrap();

                    BIZHAWK.write_u32(&MemRegion::EWRAM, 0x0003_1DB4, 0xFFFF_FFFF).unwrap();

                    self.map_state.previous_lastmap = 0xFFFF_FFFF;

                    BIZHAWK.play().unwrap();
                },
                Game::FIRERED => {
                    BIZHAWK.mute().unwrap();

                    println!("Supported map, starting transition to RED map: {:02x}, warp: {:02x}, Lastmap {:02x}", map, warp, last_map);

                    BIZHAWK.load_rom("red.gb").unwrap();
                    if self.first_warp {
                        BIZHAWK.load_state("red_default").unwrap();
                        self.first_warp = false;
                        println!("first warp");
                    } else {
                        BIZHAWK.load_state("red_warp").unwrap();
                    }

                    BIZHAWK.on_memory_write("item_api", SYM["wItemAPICommand"].bus_addr as u32, 0x04, "http://localhost:5340/item_api").ok();

                    self.game = Game::RED;

                    self.write_trainer_data();

                    self.write_party();

                    self.write_misc();

                    BIZHAWK.write_u8_sym(&SYM["wDestinationWarpID"], warp).unwrap();
                    BIZHAWK.write_u8_sym(&SYM["wCurMap"], map as u8).unwrap();
                    BIZHAWK.write_u8_sym(&SYM["wCurMapTileset"], 0x16).unwrap(); // Writing the 0x16 tileset fixes a bug with overworld maps
                    BIZHAWK.write_u8(&MemRegion::HRAM, 0x000B, map as u8).unwrap();

                    BIZHAWK.stop_drawing().unwrap();
                    BIZHAWK.clear_screen(0x00000000).unwrap();
                    BIZHAWK.play().unwrap();
                    BIZHAWK.unthrottle(0).unwrap();

                    let start_warp = BIZHAWK.framecount().unwrap();
                    
                    while BIZHAWK.framecount().unwrap() - start_warp < 240 {
                        BIZHAWK.write_u8_sym(&SYM["wMuteAudioAndPauseMusic"], 0x01).unwrap();
                        std::thread::sleep(POLL_DELAY);
                    }; // wait 240 frames

                    BIZHAWK.throttle().unwrap();
                    BIZHAWK.start_drawing().unwrap();

                    let current_music = BIZHAWK.read_u8_sym(&SYM["wLastMusicSoundID"]).unwrap();
                    BIZHAWK.write_slice_sym(&SYM["wAudioFadeOutControl"], &[current_music, 0x00, 0x00]).unwrap();
                    BIZHAWK.write_u8_sym(&SYM["wLastMap"], last_map).unwrap();

                    BIZHAWK.unmute().unwrap();
                }
            }
            self.map_state.previous_map = map;
            self.map_state.previous_warp = warp;

            self.map_state.last_change = BIZHAWK.framecount().unwrap();
        } else {
            BIZHAWK.play().unwrap();
        }
    }

    pub fn check_for_transition(&mut self, mut current_frame: u32, warp_enable: Arc<Mutex<bool>>) {
        match self.game {
            Game::RED => {
                let map_data = BIZHAWK.read_slice_sym(&SYM["wCurMap"], 0xFF).unwrap();
                let last_map = map_data[0x07];
                let current_map = map_data[0x00];
                let current_warp = map_data[0xD1];

                if current_map as u16 != self.map_state.previous_map || current_warp != self.map_state.previous_warp {
                    BIZHAWK.pause().unwrap();
                    BIZHAWK.save_state("red_warp").unwrap();
                    BIZHAWK.play().unwrap();

                    self.map_state.last_change = BIZHAWK.framecount().unwrap();
                    current_frame = self.map_state.last_change; //  Needed in case the emulator is unthrottled
                    self.map_state.previous_map = current_map as u16;
                    self.map_state.previous_warp = current_warp;
                    self.map_state.map_checked = false;
                }

                if !self.map_state.map_checked && current_frame - self.map_state.last_change > 18 {
                    if current_warp != 0xFF {println!("Map change detected, Map: {:02x}, Warp: {:02x}, Lastmap {:02x}", current_map, current_warp, last_map)};
                    if let Some(destination) = RED_FIRERED_WARP_MAP.get(&(current_map, current_warp, last_map)) {
                        self.save_state().unwrap();
                        self.handle_map_change(destination.0, destination.1, 0, warp_enable);
                    } else if current_warp != 0xFF {
                        println!("Map unknown or not supported.");
                    }
                    self.map_state.map_checked = true;
                    return;
                }

                if current_warp != 0xFF && current_frame - self.map_state.last_change > 120 {
                    BIZHAWK.write_u8_sym(&SYM["wDestinationWarpID"], 0xFF).unwrap();
                }
            },
            Game::FIRERED => {
                let map_data = BIZHAWK.read_slice(&MemRegion::EWRAM, 0x0003_1DB0, 0x10).unwrap();
                let current_map = LittleEndian::read_u16(&map_data[0x0C..0x0E]);
                let current_lastmap = LittleEndian::read_u32(&map_data[0x04..0x08]);
                let current_warp = map_data[0x0E];
                
                if self.map_state.previous_lastmap != current_lastmap {
                    self.map_state.previous_lastmap = current_lastmap;
                    println!("Map change detected, Map: {:04x}, Warp: {:02x}", current_map, current_warp);
                    if let Some(destination) = FIRERED_RED_WARP_MAP.get(&(current_map, current_warp)) {
                        BIZHAWK.pause().unwrap();
                        BIZHAWK.framerewind().unwrap();
                        BIZHAWK.save_state("firered_warp").unwrap();
                        self.save_state().unwrap();
                        //self.warp_rebound(&BIZHAWK, current_map, current_warp, 0x00);
                        self.handle_map_change(destination.0 as u16, destination.1, destination.2, warp_enable);
                    } else {
                        BIZHAWK.play().unwrap();
                        println!("Map unknown or not supported.");
                        //self.warp_rebound(&BIZHAWK, current_map, current_warp, 0x00);
                    }
                    self.map_state.map_checked = true;
                } else {
                    self.map_state.previous_lastmap = current_lastmap;
                }
            }
        }
    }

    pub fn read_party(&mut self) {
        match &self.game {
            Game::RED => {
                let party_count = BIZHAWK.read_u8_sym(&SYM["wPartyCount"]).unwrap() as u32;
                for i in 0..6 {
                    if i < party_count {
                        let pk1 = &BIZHAWK.read_slice(&MemRegion::WRAM, SYM["wPartyMons"].addr as u32 + (44*i), 44).unwrap();
                        let nickname = &BIZHAWK.read_slice(&MemRegion::WRAM, SYM["wPartyMonNicks"].addr as u32 + (11*i), 11).unwrap();

                        let mut uid = get_uid(RED_FIRERED_SPECIES[pk1[0x00] as usize], pk1[0x07]);

                        if uid == 0x0000 {
                            let mut j = 1;
                            loop {
                                uid = get_uid(RED_FIRERED_SPECIES[pk1[0x00] as usize], j as u8);
                                if uid != 0x0000 && !self.pokemon_list.contains_key(&uid) {
                                    self.pokemon_list.insert(uid, Pokemon::from_pk1(pk1, nickname, &self.trainer, uid));
                                    break;
                                }
                                j += 1;
                            }
                        } else {
                            if let Some(pokemon) = self.pokemon_list.get_mut(&uid) {
                                pokemon.update_from_pk1(pk1, nickname).unwrap();
                            } else {
                                self.pokemon_list.insert(uid, Pokemon::from_pk1(pk1, nickname, &self.trainer, uid));
                            }
                        }

                        self.party_uids[i as usize] = uid;
                    } else {
                        self.party_uids[i as usize] = 0x0000;
                    }
                    
                }
            },
            Game::FIRERED => {
                for i in 0..6 {
                    if BIZHAWK.read_u16(&MemRegion::EWRAM, 0x0002_4284 + 100*i).unwrap() == 0 && BIZHAWK.read_u16(&MemRegion::EWRAM, 0x0002_4284 + 100*i + 4).unwrap() == 0 {
                        self.party_uids[i as usize] = 0x0000;
                    } else {
                        let pk3 = &BIZHAWK.read_slice(&MemRegion::EWRAM, 0x0002_4284 + 100*i, 100).unwrap();
                        let mut decrypted_pk3 = pk3.clone();
                        decrypt_unshuffle_pk3(&mut decrypted_pk3);

                        let species = LittleEndian::read_u16(&decrypted_pk3[0x20..0x22]);

                        if species <= 152 {
                            let mut uid = get_uid(decrypted_pk3[0x20], decrypted_pk3[0x3E]);
                            if uid == 0x0000 {
                                let mut j = 1;
                                loop {
                                    uid = get_uid(decrypted_pk3[0x20], j as u8);
                                    if uid != 0x0000 && !self.pokemon_list.contains_key(&uid) {
                                        self.pokemon_list.insert(uid, Pokemon::from_pk3(pk3, &self.trainer, uid));
                                        break;
                                    }
                                    j += 1;
                                }
                            } else {
                                if let Some(pokemon) = self.pokemon_list.get_mut(&uid) {
                                    pokemon.update_from_pk3(pk3).unwrap();
                                } else {
                                    self.pokemon_list.insert(uid, Pokemon::from_pk3(pk3, &self.trainer, uid));
                                }
                            }

                            self.party_uids[i as usize] = uid;
                        } else {
                            self.party_uids[i as usize] = 0x0000;
                        }
                    }
                }
            }
        }
    }

    pub fn write_party(&mut self) {
        match &self.game {
            Game::RED => {
                let mut slot = 0;
                let mut count = 0;
                let mut data = vec![0;0x194];
                for i in &self.party_uids {
                    println!("{:?}", i);
                    if *i == 0x0000 {
                        data[0x01 + slot] = 0xFF;
                        slot += 1;
                    } else {
                        let pokemon = &self.pokemon_list.get(i).unwrap();
                        data[0x01 + slot] = FIRERED_RED_SPECIES[pokemon.species as usize];
                        if let Some(ref gen1) = pokemon.gen1 {
                            data[(0x08 + slot*0x2C)..(0x08 + (slot+1)*0x2C)].copy_from_slice(&gen1.bytes);
                            data[(0x110 + slot*0x0B)..(0x118 + (slot)*0x0B)].copy_from_slice(&self.trainer.gen_1_name);
                            data[(0x152 + slot*0x0B)..(0x152 + (slot+1)*0x0B)].copy_from_slice(&gen1.name);
                        } else {
                            panic!("Trying to write Gen 1 data but Gen 1 data is not generated yet");
                        }
                        count += 1;
                        slot += 1;
                    }
                }
                data[0x00] = count as u8;
                BIZHAWK.write_slice_sym(&SYM["wPartyDataStart"], &data).unwrap();
            },
            Game::FIRERED => {
                let mut slot = 0;
                for i in &self.party_uids {
                    println!("{:?}", i);
                    if *i == 0x0000 {
                        BIZHAWK.write_slice(&MemRegion::EWRAM, 0x0002_4284 + (slot*100), &[0;100]).unwrap();
                        slot += 1;
                        //break;
                    } else {
                        let pokemon = &self.pokemon_list.get(i).unwrap();
                        BIZHAWK.write_slice(&MemRegion::EWRAM, 0x0002_4284 + (slot*100), &pokemon.get_encrypted_pk3().unwrap()).unwrap();
                        slot += 1;
                    }
                }
            }
        }
    }

    pub fn read_trainer_data(&mut self) {
        match &self.game {
            Game::RED => {
                self.trainer = TrainerInfo::from_gen1(BIZHAWK.read_u16_be_sym(&SYM["wPlayerID"]).unwrap(),
                                                      &BIZHAWK.read_slice_sym(&SYM["wPlayerName"], 0x08).unwrap(),
                                                      &BIZHAWK.read_slice_sym(&SYM["wRivalName"], 0x08).unwrap());
            },
            Game::FIRERED => {
                let data = BIZHAWK.read_slice_custom("*0300500C/F".to_string(), 0x0F).unwrap();
                let rival_name = BIZHAWK.read_slice_custom("*03005008+3A4C/8".to_string(), 0x08).unwrap();
                self.trainer = TrainerInfo::from_gen3(LittleEndian::read_u16(&data[0x0A..0x0C]),
                                                      LittleEndian::read_u16(&data[0x0C..0x0E]),
                                                      &data[0x00..0x08],
                                                      &rival_name);
            }
        }
    }

    pub fn write_trainer_data(&mut self) {
        match &self.game {
            Game::RED => {
                BIZHAWK.write_u16_be_sym(&SYM["wPlayerID"], self.trainer.tid).unwrap();
                BIZHAWK.write_slice_sym(&SYM["wPlayerName"], &self.trainer.gen_1_name).unwrap();
                BIZHAWK.write_slice_sym(&SYM["wRivalName"], &self.trainer.gen_1_rival_name).unwrap();
            },
            Game::FIRERED => {
                let pointer1 = BIZHAWK.read_u32(&MemRegion::IWRAM, 0x500C).unwrap() & 0x00FFFFFF;
                let pointer2 = BIZHAWK.read_u32(&MemRegion::IWRAM, 0x5008).unwrap() & 0x00FFFFFF;

                BIZHAWK.write_u16(&MemRegion::EWRAM, pointer1 + 0x0A, self.trainer.tid).unwrap();
                BIZHAWK.write_u16(&MemRegion::EWRAM, pointer1 + 0x0C, self.trainer.sid).unwrap();
                BIZHAWK.write_slice(&MemRegion::EWRAM, pointer1 + 0x00, &self.trainer.gen_3_name).unwrap();
                BIZHAWK.write_slice(&MemRegion::EWRAM, pointer2 + 0x3A4C, &self.trainer.gen_3_rival_name).unwrap();
            }
        }
    }

    pub fn read_misc(&mut self) {
        match &self.game {
            Game::RED => {
                let options_byte = BIZHAWK.read_u8_sym(&SYM["wOptions"]).unwrap();

                self.options.text_speed = options_byte & 0x0F;
                self.options.animations = options_byte & 0x80 == 0x00;
                self.options.switching = options_byte & 0x40 == 0x00;
                
                self.money = u32::from_str_radix(&format!("{:06x}", BIZHAWK.read_u32_be_sym(&SYM["wPlayerMoney"]).unwrap() >> 8), 10).unwrap_or(0); // BCD

                self.owned.copy_from_slice(&BIZHAWK.read_slice_sym(&SYM["wPokedexOwned"], 19).unwrap());
                self.seen.copy_from_slice(&BIZHAWK.read_slice_sym(&SYM["wPokedexSeen"], 19).unwrap());
            },
            Game::FIRERED => {
                let options_word = LittleEndian::read_u16(&BIZHAWK.read_slice_custom("*0300500C+14/2".to_string(), 0x02).unwrap());

                self.options.text_speed = ((0x03 - (options_word & 0x0003) as u8) * 2) - 1; // convert to red text speed
                self.options.animations = options_word & 0x0400 == 0x0000;
                self.options.switching = options_word & 0x0200 == 0x0000;

                let key = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap());
                self.money = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*03005008+290/4".to_string(), 0x04).unwrap()) ^ key;

                self.owned.copy_from_slice(&BIZHAWK.read_slice_custom("*0300500C+28/13".to_string(), 19).unwrap());
                self.seen.copy_from_slice(&BIZHAWK.read_slice_custom("*0300500C+5C/13".to_string(), 19).unwrap());
            }
        }
    }

    pub fn write_misc(&mut self) {
        match &self.game {
            Game::RED => {
                let options_byte = (self.options.text_speed & 0x0F) | 
                                   (if self.options.animations { 0x00 } else { 0x80 }) |
                                   (if self.options.switching { 0x00 } else { 0x40 });

                BIZHAWK.write_u8_sym(&SYM["wOptions"], options_byte).unwrap();

                let mut money_vec = vec![0;3];

                BigEndian::write_u24(&mut money_vec, u32::from_str_radix(&format!("{:06}", self.money), 16).unwrap());

                BIZHAWK.write_slice_sym(&SYM["wPlayerMoney"], &money_vec).unwrap(); // BCD

                BIZHAWK.write_slice_sym(&SYM["wPokedexOwned"], &self.owned).unwrap();
                BIZHAWK.write_slice_sym(&SYM["wPokedexSeen"], &self.seen).unwrap();
            },
            Game::FIRERED => {
                let pointer1 = BIZHAWK.read_u32(&MemRegion::IWRAM, 0x500C).unwrap() & 0x00FFFFFF;
                let pointer2 = BIZHAWK.read_u32(&MemRegion::IWRAM, 0x5008).unwrap() & 0x00FFFFFF;

                // get the byte with the options we change from the game masked off
                let mut options_word = LittleEndian::read_u16(&BIZHAWK.read_slice_custom("*0300500C+14/2".to_string(), 0x02).unwrap()) & 0xF9FC;

                // apply our options
                options_word |= ((0x03 - ((self.options.text_speed as u16 + 1) / 2)) & 0x0003) | 
                                (if self.options.animations { 0x0000 } else { 0x0400 }) |
                                (if self.options.switching { 0x0000 } else { 0x0200 });

                // write it back
                BIZHAWK.write_u16(&MemRegion::EWRAM, pointer1 + 0x14, options_word).unwrap();

                let key = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap());

                BIZHAWK.write_u32(&MemRegion::EWRAM, pointer2 + 0x290, self.money ^ key).unwrap();

                BIZHAWK.write_slice(&MemRegion::EWRAM, pointer1 + 0x28, &self.owned).unwrap();
                BIZHAWK.write_slice(&MemRegion::EWRAM, pointer1 + 0x5C, &self.seen).unwrap();
            }
        }
    }

    pub fn read_items(&mut self) {
        match &self.game {
            Game::RED => {
                // Dont use this function
            },
            Game::FIRERED => {
                let key = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap()) as u16;

                let mut item_memory = BIZHAWK.read_slice_custom("*03005008+310/2E8".to_string(), 0x2E8).unwrap();

                // Decrypt
                for i in 0..(item_memory.len() / 0x04) {
                    let current_section = LittleEndian::read_u16(&item_memory[(i*0x04 + 0x02)..(i*0x04) + 0x04]);
                    LittleEndian::write_u16(&mut item_memory[(i*0x04 + 0x02)..(i*0x04) + 0x04], current_section ^ key);
                }

                for i in 0..4 {
                    let pocket = match i {
                        0 => &item_memory[0x0000..0x00A8],
                        1 => &item_memory[0x00A8..0x0120],
                        2 => &item_memory[0x0120..0x0154],
                        _ => &item_memory[0x0154..0x023C]
                    };

                    for j in 0..pocket.len() / 0x04 {
                        let id = LittleEndian::read_u16(&pocket[j*0x04..0x02 + j*0x04]);
                        let count = LittleEndian::read_u16(&pocket[0x02 + j*0x04..0x04 + j*0x04]);

                        self.firered_items[i].content.clear();

                        if id == 0 || count == 0 {
                            break;
                        } 

                        self.firered_items[i].content.push([id, count]);
                    }
                }
            }
        }
    }

    pub fn write_items(&mut self) {
        match &self.game {
            Game::RED => {
                // Dont use this function
            },
            Game::FIRERED => {
                
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MapState {
    previous_map: u16,
    previous_warp: u8,
    previous_lastmap: u32,
    map_checked: bool,
    last_change: u32
}

impl MapState {
    pub fn new() -> MapState {
        MapState {
            previous_map: 0x00,
            previous_warp: 0x00,
            previous_lastmap: 0x00,
            map_checked: false,
            last_change: 0x00
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrainerInfo {
    pub gen_1_name: Vec<u8>,
    pub gen_3_name: Vec<u8>,
    pub gen_1_rival_name: Vec<u8>,
    pub gen_3_rival_name: Vec<u8>,
    pub tid: u16,
    pub sid: u16,
}

impl TrainerInfo {
    pub fn new() -> TrainerInfo {
        TrainerInfo {
            gen_1_name: Vec::new(),
            gen_3_name: Vec::new(),
            gen_1_rival_name: Vec::new(),
            gen_3_rival_name: Vec::new(),
            tid: 0,
            sid: 0
        }
    }
    pub fn from_gen1(tid: u16, nickname: &[u8], rival: &[u8]) -> TrainerInfo {

        let mut gen_1_name = nickname.to_vec();
        let mut gen_1_rival_name = rival.to_vec();
        let mut gen_3_name = nickname.to_vec();
        let mut gen_3_rival_name = rival.to_vec();

        for character in &mut gen_3_name {
            *character = RED_FIRERED_TEXT[*character as usize];
        }

        for character in &mut gen_3_rival_name {
            *character = RED_FIRERED_TEXT[*character as usize];
        }

        // Force terminators
        gen_1_name[0x07] = 0x50;
        gen_1_rival_name[0x07] = 0x50;
        gen_3_name[0x07] = 0xFF;
        gen_3_rival_name[0x07] = 0xFF;

        TrainerInfo {
            gen_1_name,
            gen_3_name,
            gen_1_rival_name,
            gen_3_rival_name,
            tid,
            sid: rand::random::<u16>()
        }
    }
    pub fn from_gen3(tid: u16, sid:u16, nickname: &[u8], rival: &[u8]) -> TrainerInfo {

        let mut gen_1_name = nickname.to_vec();
        let mut gen_1_rival_name = rival.to_vec();
        let mut gen_3_name = nickname.to_vec();
        let mut gen_3_rival_name = rival.to_vec();

        for character in &mut gen_1_name {
            *character = FIRERED_RED_TEXT[*character as usize];
        }

        for character in &mut gen_1_rival_name {
            *character = FIRERED_RED_TEXT[*character as usize];
        }

        // Force terminators
        gen_1_name[0x07] = 0x50;
        gen_1_rival_name[0x07] = 0x50;
        gen_3_name[0x07] = 0xFF;
        gen_3_rival_name[0x07] = 0xFF;

        TrainerInfo {
            gen_1_name,
            gen_3_name,
            gen_1_rival_name,
            gen_3_rival_name,
            tid,
            sid
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Options {
    pub text_speed: u8,
    pub animations: bool,
    pub switching: bool
}

impl Options {
    pub fn new() -> Options {
        Options {
            text_speed: 3,
            animations: true,
            switching: true
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Game {
    RED,
    FIRERED
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Pocket {
    pub content: Vec<[u16;2]>,
    pub id: u8,
    pub name: Vec<u8>
}

impl Pocket {
    pub fn new(name: &[u8], id: u8) -> Pocket {
        Pocket {
            content: Vec::new(),
            id,
            name: name.to_vec()
        }
    }
}