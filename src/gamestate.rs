use crate::pokemon::*;
use crate::constants::*;
use crate::bizhawk::*;
use bincode;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use rand;
use byteorder::{ByteOrder, LittleEndian, BigEndian};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub trainer: TrainerInfo,
    red_items: Bag,
    firered_items: Bag,
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
            red_items: Bag::new(),
            firered_items: Bag::new(),
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

    pub fn from_file(bizhawk: &Bizhawk) -> std::io::Result<GameState> {
        if let Ok(f) = File::open("GameState") {
            println!("GameState found, loading from disk.\n");
            Ok(bincode::deserialize_from(f).unwrap())
        } else {
            println!("No existing GameState found, using default.\n");
            let mut default = GameState::new();
            default.get_current_game(bizhawk);
            default.read_trainer_data(bizhawk);
            default.collect_mapstate(bizhawk);
            Ok(default)
        }
    }

    pub fn save_state(&mut self) -> std::io::Result<()> {
        //println!("Saving GameState to disk.\n");
        let f = File::create("GameState")?;
        bincode::serialize_into(f, self).unwrap();
        Ok(())
    }

    pub fn get_current_game(&mut self, bizhawk: &Bizhawk) {
        self.game = if let Ok(name) = bizhawk.get_rom_name() {
            match name.as_str() {
                "Pokemon - Red Version (USA, Europe)" => Game::RED,
                "firered" => Game::FIRERED,
                _ => panic!("Neither red nor firered loaded!")
            }
        } else {
            panic!("Failed to get rom name!");
        };
    }

    pub fn collect_mapstate(&mut self, bizhawk: &Bizhawk) {
        match self.game {
            Game::RED => {
                self.map_state = MapState {
                    previous_map: bizhawk.read_u8(MemRegion::WRAM, 0x135E).unwrap() as u16,
                    previous_warp: bizhawk.read_u8(MemRegion::WRAM, 0x142F).unwrap(),
                    previous_lastmap: 0,
                    map_checked: true,
                    last_change: 0
                }
            },
            Game::FIRERED => {
                self.map_state = MapState {
                    previous_map: bizhawk.read_u16(MemRegion::EWRAM, 0x0003_1DBC).unwrap(),
                    previous_warp: bizhawk.read_u8(MemRegion::EWRAM, 0x0003_1DBE).unwrap(),
                    previous_lastmap: bizhawk.read_u32(MemRegion::EWRAM, 0x0003_1DB4).unwrap(),
                    map_checked: true,
                    last_change: 0
                }
            }
        };
    }

    pub fn handle_map_change(&mut self, bizhawk: &Bizhawk, map: u16, warp: u8, last_map: u8) {

        bizhawk.pause().unwrap();

        //println!("--Reading party");

        self.read_party(bizhawk);

        self.read_misc(bizhawk);

        if self.options.switching {
            match &self.game {
                Game::RED => { 
                    println!("Supported map, starting transition to FIRE RED map: {:04x}, warp: {:02x}", map, warp);
                    //println!("--Loading FIRE RED ROM");

                    bizhawk.load_rom("firered.gba").unwrap();
                    bizhawk.load_state("firered_warp").unwrap();

                    self.game = Game::FIRERED;

                    self.write_trainer_data(bizhawk);

                    self.write_party(bizhawk);

                    self.write_misc(bizhawk);

                    bizhawk.write_u16(MemRegion::EWRAM, 0x0003_1DBC, map).unwrap();
                    bizhawk.write_u8(MemRegion::EWRAM, 0x0003_1DBE, warp).unwrap();

                    bizhawk.frameadvance().unwrap();

                    bizhawk.write_u32(MemRegion::EWRAM, 0x0003_1DB4, 0xFFFF_FFFF).unwrap();

                    self.map_state.previous_lastmap = 0xFFFF_FFFF;

                    bizhawk.play().unwrap();

                    self.map_state.previous_map = map;
                    self.map_state.previous_warp = warp; 

                    self.map_state.last_change = bizhawk.framecount().unwrap();

                    //println!("--DONE\n");
                },
                Game::FIRERED => {
                    bizhawk.mute().unwrap();
                    println!("Supported map, starting transition to RED map: {:02x}, warp: {:02x}, Lastmap {:02x}", map, warp, last_map);

                    //println!("--Loading RED ROM");

                    bizhawk.load_rom("red.gbc").unwrap();
                    if self.first_warp {
                        bizhawk.load_state("red_default").unwrap();
                        self.first_warp = false;
                        println!("first warp");
                    } else {
                        bizhawk.load_state("red_warp").unwrap();
                    }

                    self.game = Game::RED;

                    let start_warp = bizhawk.framecount().unwrap();

                    self.write_trainer_data(bizhawk);

                    self.write_party(bizhawk);

                    self.write_misc(bizhawk);

                    bizhawk.write_u8(MemRegion::WRAM, 0x1365, last_map).unwrap(); // lastmap
                    bizhawk.write_u8(MemRegion::WRAM, 0x142F, warp).unwrap(); // destination warp
                    bizhawk.write_u8(MemRegion::HRAM, 0x000B, map as u8).unwrap(); // map
                    bizhawk.write_u8(MemRegion::WRAM, 0x135E, map as u8).unwrap(); // map

                    bizhawk.stop_drawing().unwrap();
                    bizhawk.clear_screen(0x00000000).unwrap();
                    bizhawk.play().unwrap();
                    bizhawk.unthrottle(0).unwrap();
                    
                    while bizhawk.framecount().unwrap() - start_warp < 240 {
                        bizhawk.write_u8(MemRegion::WRAM, 0x0002, 0x01).unwrap(); // Music
                        std::thread::sleep(POLL_DELAY);
                    }; // wait 240 frames

                    bizhawk.throttle().unwrap();
                    bizhawk.start_drawing().unwrap();

                    let current_music = bizhawk.read_u8(MemRegion::WRAM, 0x0FCA).unwrap(); // map
                    bizhawk.write_slice(MemRegion::WRAM, 0x0fC7, &[current_music, 0x00, 0x00]).unwrap();

                    bizhawk.unmute().unwrap();

                    self.map_state.previous_map = bizhawk.read_u8(MemRegion::WRAM, 0x135E).unwrap() as u16;
                    self.map_state.previous_warp = bizhawk.read_u8(MemRegion::WRAM, 0x142F).unwrap();

                    self.map_state.last_change = bizhawk.framecount().unwrap();

                    //println!("--DONE\n");
                }
            }
        } else {
            bizhawk.play().unwrap();
        }
    }

    pub fn check_for_transition(&mut self, bizhawk: &Bizhawk, mut current_frame: u32) {
        match self.game {
            Game::RED => {
                let map_data = bizhawk.read_slice(MemRegion::WRAM, 0x1350, 0xFF).unwrap();
                let last_map = map_data[0x15];
                let current_map = map_data[0x0E];
                let current_warp = map_data[0xDF];
                
                if current_map as u16 != self.map_state.previous_map || (current_warp != self.map_state.previous_warp) {
                    self.map_state.last_change = bizhawk.framecount().unwrap();
                    current_frame = self.map_state.last_change; //  Needed in case the emulator is unthrottled
                    self.map_state.previous_map = current_map as u16;
                    self.map_state.previous_warp = current_warp;
                    self.map_state.map_checked = false;
                    // If we might be warping, save the state
                    if RED_FIRERED_WARP_MAP.get(&(current_map, current_warp, last_map)).is_some() {
                        bizhawk.pause().unwrap(); 
                        bizhawk.save_state("red_warp").unwrap();
                        self.save_state().unwrap();
                    }
                    bizhawk.play().unwrap();
                }

                if !self.map_state.map_checked && current_frame - self.map_state.last_change > 18 {
                    if current_warp != 0xFF {println!("Map change detected, Map: {:02x}, Warp: {:02x}, Lastmap {:02x}", current_map, current_warp, last_map)};
                    if let Some(destination) = RED_FIRERED_WARP_MAP.get(&(current_map, current_warp, last_map)) {
                        self.handle_map_change(&bizhawk, destination.0, destination.1, 0);
                    } else if current_warp != 0xFF {
                        println!("Map unknown or not supported.");
                    }
                    self.map_state.map_checked = true;
                    return;
                }

                if current_warp != 0xFF && current_frame - self.map_state.last_change > 120 {
                    bizhawk.write_u8(MemRegion::WRAM, 0x142F, 0xFF).unwrap();
                }
            },
            Game::FIRERED => {
                let map_data = bizhawk.read_slice(MemRegion::EWRAM, 0x0003_1DB0, 0x10).unwrap();
                let current_map = LittleEndian::read_u16(&map_data[0x0C..0x0E]);
                let current_lastmap = LittleEndian::read_u32(&map_data[0x04..0x08]);
                let current_warp = map_data[0x0E];
                
                if self.map_state.previous_lastmap != current_lastmap {
                    self.map_state.previous_lastmap = current_lastmap;
                    println!("Map change detected, Map: {:04x}, Warp: {:02x}", current_map, current_warp);
                    if let Some(destination) = FIRERED_RED_WARP_MAP.get(&(current_map, current_warp)) {
                        bizhawk.pause().unwrap();
                        bizhawk.framerewind().unwrap();
                        bizhawk.save_state("firered_warp").unwrap();
                        self.save_state().unwrap();
                        self.handle_map_change(&bizhawk, destination.0 as u16, destination.1, destination.2);
                    } else {
                        bizhawk.play().unwrap();
                        println!("Map unknown or not supported.");
                    }
                    self.map_state.map_checked = true;
                } else {
                    self.map_state.previous_lastmap = current_lastmap;
                }
            }
        }
    }

    pub fn read_party(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                let party_count = bizhawk.read_u8(MemRegion::WRAM, 0x1163).unwrap() as u32;
                for i in 0..6 {
                    if i < party_count {
                        let pk1 = &bizhawk.read_slice(MemRegion::WRAM, 0x116B + (44*i), 44).unwrap();
                        let nickname = &bizhawk.read_slice(MemRegion::WRAM, 0x12B5 + (11*i), 11).unwrap();

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
                    if bizhawk.read_u16(MemRegion::EWRAM, 0x0002_4284 + 100*i).unwrap() == 0 && bizhawk.read_u16(MemRegion::EWRAM, 0x0002_4284 + 100*i + 4).unwrap() == 0 {
                        self.party_uids[i as usize] = 0x0000;
                    } else {
                        let pk3 = &bizhawk.read_slice(MemRegion::EWRAM, 0x0002_4284 + 100*i, 100).unwrap();
                        let mut decrypted_pk3 = pk3.clone();
                        decrypt_unshuffle_pk3(&mut decrypted_pk3);

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
                    }
                }
            }
        }
    }

    pub fn write_party(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                let mut slot = 0;
                let mut count = 0;
                let mut data = bizhawk.read_slice(MemRegion::WRAM, 0x1160, 0x1A0).unwrap();
                for i in &self.party_uids {
                    if *i == 0x0000 {
                        data[0x04 + slot] = 0xFF;
                        slot += 1;
                    } else {
                        let pokemon = &self.pokemon_list.get(i).unwrap();
                        data[0x04 + slot] = FIRERED_RED_SPECIES[pokemon.species as usize];
                        if let Some(ref gen1) = pokemon.gen1 {
                            data[(0x0B + slot*0x2C)..(0x0B + (slot+1)*0x2C)].copy_from_slice(&gen1.bytes);
                            data[(0x113 + slot*0x07)..(0x113 + (slot+1)*0x07 + 0x01)].copy_from_slice(&self.trainer.gen_1_name);
                            data[(0x155 + slot*0x0B)..(0x155 + (slot+1)*0x0B)].copy_from_slice(&gen1.name);
                        } else {
                            panic!("Trying to write Gen 1 data but Gen 1 data is not generated yet");
                        }
                        count += 1;
                        slot += 1;
                    }
                }
                data[0x03] = count as u8;
                bizhawk.write_slice(MemRegion::WRAM, 0x1160, &data).unwrap();
            },
            Game::FIRERED => {
                let mut slot = 0;
                for i in &self.party_uids {
                    if *i == 0x0000 {
                        bizhawk.write_slice(MemRegion::EWRAM, 0x0002_4284 + (slot*100), &[0;100]).unwrap();
                        break;
                    } else {
                        let pokemon = &self.pokemon_list.get(i).unwrap();
                        bizhawk.write_slice(MemRegion::EWRAM, 0x0002_4284 + (slot*100), &pokemon.get_encrypted_pk3().unwrap()).unwrap();
                        slot += 1;
                    }
                }
            }
        }
    }

    pub fn read_trainer_data(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                self.trainer = TrainerInfo::from_gen1(bizhawk.read_u16_be(MemRegion::WRAM, 0x1359).unwrap(),
                                                      &bizhawk.read_slice(MemRegion::WRAM, 0x1158, 0x07).unwrap(),
                                                      &bizhawk.read_slice(MemRegion::WRAM, 0x134A, 0x07).unwrap());
            },
            Game::FIRERED => {
                /*let pointer = bizhawk.read_u32(MemRegion::IWRAM, 0x0300500C).unwrap();
                let data = bizhawk.read_slice(MemRegion::EWRAM, pointer & 0x00FFFFFF, 0x0F).unwrap();*/
                let data = bizhawk.read_slice_custom("*0300500C/F".to_string(), 0x0F).unwrap();
                let rival_name = bizhawk.read_slice_custom("*03005008+3A4C/8".to_string(), 0x08).unwrap();
                self.trainer = TrainerInfo::from_gen3(LittleEndian::read_u16(&data[0x0A..0x0C]),
                                                      LittleEndian::read_u16(&data[0x0C..0x0E]),
                                                      &data[0x00..0x08],
                                                      &rival_name);
            }
        }
    }

    pub fn write_trainer_data(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                bizhawk.write_u16_be(MemRegion::WRAM, 0x1359, self.trainer.tid).unwrap();
                bizhawk.write_slice(MemRegion::WRAM, 0x1158, &self.trainer.gen_1_name).unwrap();
                bizhawk.write_slice(MemRegion::WRAM, 0x134A, &self.trainer.gen_1_rival_name).unwrap();
            },
            Game::FIRERED => {
                let pointer1 = bizhawk.read_u32(MemRegion::IWRAM, 0x500C).unwrap() & 0x00FFFFFF;
                let pointer2 = bizhawk.read_u32(MemRegion::IWRAM, 0x5008).unwrap() & 0x00FFFFFF;

                bizhawk.write_u16(MemRegion::EWRAM, pointer1 + 0x0A, self.trainer.tid).unwrap();
                bizhawk.write_u16(MemRegion::EWRAM, pointer1 + 0x0C, self.trainer.sid).unwrap();
                bizhawk.write_slice(MemRegion::EWRAM, pointer1 + 0x00, &self.trainer.gen_3_name).unwrap();
                bizhawk.write_slice(MemRegion::EWRAM, pointer2 + 0x3A4C, &self.trainer.gen_3_rival_name).unwrap();
            }
        }
    }

    pub fn read_misc(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                let old_switching = self.options.switching;

                let options_byte = bizhawk.read_u8(MemRegion::WRAM, 0x1355).unwrap();

                self.options.text_speed = options_byte & 0x0F;
                self.options.animations = options_byte & 0x80 == 0x00;
                self.options.switching = options_byte & 0x40 == 0x00;

                if old_switching != self.options.switching {
                    println!("Warps {}", if self.options.switching {"Enabled"} else {"Disabled"});
                }

                self.money = u32::from_str_radix(&format!("{:06x}", bizhawk.read_u32_be(MemRegion::WRAM, 0x1347).unwrap() >> 8), 10).unwrap_or(0); // BCD

                self.owned.copy_from_slice(&bizhawk.read_slice(MemRegion::WRAM, 0x12f7, 19).unwrap());
                self.seen.copy_from_slice(&bizhawk.read_slice(MemRegion::WRAM, 0x130a, 19).unwrap());
            },
            Game::FIRERED => {
                let old_switching = self.options.switching;

                let options_word = LittleEndian::read_u16(&bizhawk.read_slice_custom("*0300500C+14/2".to_string(), 0x02).unwrap());

                self.options.text_speed = ((0x03 - (options_word & 0x0003) as u8) * 2) - 1; // convert to red text speed
                self.options.animations = options_word & 0x0400 == 0x0000;
                self.options.switching = options_word & 0x0200 == 0x0000;

                if old_switching != self.options.switching {
                    println!("Warps {}", if self.options.switching {"Enabled"} else {"Disabled"});
                }

                let key = LittleEndian::read_u32(&bizhawk.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap());
                self.money = LittleEndian::read_u32(&bizhawk.read_slice_custom("*03005008+290/4".to_string(), 0x04).unwrap()) ^ key;

                self.owned.copy_from_slice(&bizhawk.read_slice_custom("*0300500C+28/13".to_string(), 19).unwrap());
                self.seen.copy_from_slice(&bizhawk.read_slice_custom("*0300500C+5C/13".to_string(), 19).unwrap());
            }
        }
    }

    pub fn write_misc(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                let options_byte = (self.options.text_speed & 0x0F) | 
                                   (if self.options.animations { 0x00 } else { 0x80 }) |
                                   (if self.options.switching { 0x00 } else { 0x40 });

                bizhawk.write_u8(MemRegion::WRAM, 0x1355, options_byte).unwrap();

                let mut money_vec = vec![0;3];

                LittleEndian::write_u24(&mut money_vec, u32::from_str_radix(&format!("{:06}", self.money), 16).unwrap());

                bizhawk.write_slice(MemRegion::WRAM, 0x1347, &money_vec).unwrap(); // BCD

                bizhawk.write_slice(MemRegion::WRAM, 0x12f7, &self.owned).unwrap();
                bizhawk.write_slice(MemRegion::WRAM, 0x130a, &self.seen).unwrap();
            },
            Game::FIRERED => {
                let pointer1 = bizhawk.read_u32(MemRegion::IWRAM, 0x500C).unwrap() & 0x00FFFFFF;
                let pointer2 = bizhawk.read_u32(MemRegion::IWRAM, 0x5008).unwrap() & 0x00FFFFFF;

                // get the byte with the options we change from the game masked off
                let mut options_word = LittleEndian::read_u16(&bizhawk.read_slice_custom("*0300500C+14/2".to_string(), 0x02).unwrap()) & 0xF9FC;

                // apply our options
                options_word |= ((0x03 - ((self.options.text_speed as u16 + 1) / 2)) & 0x0003) | 
                                (if self.options.animations { 0x0000 } else { 0x0400 }) |
                                (if self.options.switching { 0x0000 } else { 0x0200 });

                // write it back
                bizhawk.write_u16(MemRegion::EWRAM, pointer1 + 0x14, options_word).unwrap();

                let key = LittleEndian::read_u32(&bizhawk.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap());

                bizhawk.write_u32(MemRegion::EWRAM, pointer2 + 0x290, self.money ^ key).unwrap();

                bizhawk.write_slice(MemRegion::EWRAM, pointer1 + 0x28, &self.owned).unwrap();
                bizhawk.write_slice(MemRegion::EWRAM, pointer1 + 0x5C, &self.seen).unwrap();
            }
        }
    }

    pub fn read_items(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                // Dont use this function
            },
            Game::FIRERED => {
                let key = LittleEndian::read_u32(&bizhawk.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap()) as u16;

                let mut item_memory = bizhawk.read_slice_custom("*03005008+310/2E8".to_string(), 0x2E8).unwrap();

                // Decrypt
                for i in 0..(item_memory.len() / 0x04) {
                    let current_section = LittleEndian::read_u16(&item_memory[(i*0x04 + 0x02)..(i*0x04) + 0x04]);
                    LittleEndian::write_u16(&mut item_memory[(i*0x04 + 0x02)..(i*0x04) + 0x04], current_section ^ key);
                }
                
                /*println!("Address 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
                for i in 0..(item_memory.len() / 0x10) {
                    print!("{:07X}", i);
                    for j in 0..0x10 {
                        print!(" {:02X}", item_memory[(i*0x10)+j]);
                    }
                    println!("");
                }*/

                for i in 0..4 {
                    let pocket = match i {
                        0 => &item_memory[0x0000..0x00A8],
                        1 => &item_memory[0x00A8..0x0120],
                        2 => &item_memory[0x0120..0x0154],
                        _ => &item_memory[0x0154..0x023C]
                    };

                    let pocket_to_update = match i {
                        0 => &mut self.firered_items.general,
                        1 => &mut self.firered_items.key,
                        2 => &mut self.firered_items.balls,
                        _ => &mut self.firered_items.tmhm,
                    };

                    for j in 0..pocket.len() / 0x04 {
                        let id = LittleEndian::read_u16(&pocket[j*0x04..0x02 + j*0x04]);
                        let count = LittleEndian::read_u16(&pocket[0x02 + j*0x04..0x04 + j*0x04]);
                        if id == 0 || count == 0 {
                            break;
                        }
                        if let Some(entry) = pocket_to_update.get_mut(&id) {
                            *entry = count;
                        } else {
                            pocket_to_update.insert(id, count);
                        }
                    }
                }
            }
        }
    }

    pub fn write_items(&mut self, bizhawk: &Bizhawk) {
        match &self.game {
            Game::RED => {
                // Dont use this function
            },
            Game::FIRERED => {
                
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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
pub struct Bag {
    pub general: BTreeMap<u16, u16>, // ID, Count
    pub key: BTreeMap<u16, u16>,
    pub balls: BTreeMap<u16, u16>,
    pub tmhm: BTreeMap<u16, u16>
}

impl Bag {
    pub fn new() -> Bag {
        Bag {
            general: BTreeMap::new(),
            key: BTreeMap::new(),
            balls: BTreeMap::new(),
            tmhm: BTreeMap::new()
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