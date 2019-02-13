use crate::item_api::*;
use crate::pokemon::*;
use crate::constants::*;
use crate::bizhawk::*;
use std::cmp::min;
use itertools::rev;
use bincode;
use std::collections::HashMap;
use std::fs::File;
use rand;
use byteorder::{ByteOrder, LittleEndian, BigEndian};
use std::sync::{Arc, Mutex};
use serde_json::to_string;
use chrono::{DateTime, Utc};

/* for the HUD's API */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HUDData {
    #[serde(skip_serializing_if = "Option::is_none")]
    game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    badges: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Items>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updates_paused: Option<bool>
}

/* for the HUD's API */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Items {
    items: Vec<Item>,
    key_red: Vec<Item>,
    key_firered: Vec<Item>,
    balls: Vec<Item>,
    berries: Vec<Item>,
    tms: Vec<Item>,
    pc_red: Vec<Item>,
    pc_firered: Vec<Item>
}

impl Items {
    pub fn from_pocket_data(red: &[Pocket;5], fire_red: &[Pocket;6], game: &Game) -> Items {
        Items {
            items: match game {
                Game::RED => {
                    red[P_GENR as usize].content.iter().map(Item::from_slice).collect()
                },
                Game::FIRERED => {
                    fire_red[P_GENR as usize].content.iter().map(Item::from_slice).collect()
                }
            },

            key_red: red[P_KEYI as usize].content.iter().map(Item::from_slice).collect(),
            key_firered: fire_red[P_KEYI as usize].content.iter().map(Item::from_slice).collect(),

            balls: match game {
                Game::RED => {
                    red[P_BALL as usize].content.iter().map(Item::from_slice).collect()
                },
                Game::FIRERED => {
                    fire_red[P_BALL as usize].content.iter().map(Item::from_slice).collect()
                }
            },

            berries: fire_red[P_BERY as usize].content.iter().map(Item::from_slice).collect(),
            tms: match game {
                Game::RED => {
                    red[P_TMHM as usize].content.iter().map(Item::from_slice).collect()
                },
                Game::FIRERED => {
                    fire_red[P_TMHM as usize].content.iter().map(Item::from_slice).collect()
                }
            },
            pc_red: red[P_PC as usize].content.iter().map(Item::from_slice).collect(),
            pc_firered: fire_red[P_PC as usize].content.iter().map(Item::from_slice).collect()
        }
    }
}

/* for the HUD's API */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    id: u16,
    count: u16
}

impl Item {
    pub fn from_slice(source: &[u16; 2]) -> Item {
        Item {
            id: source[0],
            count: source[1]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub trainer: TrainerInfo,
    red_items: [Pocket;5],
    firered_items: [Pocket;6],
    red_progress: u32,
    firered_progress: u32,
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
    pub first_warp: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub level_cap: u8,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            trainer: TrainerInfo::new(),
            red_items: [Pocket::new(&[0x86, 0xA4, 0xAD, 0xA4, 0xB1, 0xA0, 0xAB, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50], P_GENR), // "General"
                        Pocket::new(&[0x8A, 0xA4, 0xB8, 0x7F, 0x88, 0xB3, 0xA4, 0xAC, 0xB2, 0x50, 0x50, 0x50, 0x50], P_KEYI), // "Key Items"
                        Pocket::new(&[0x8F, 0xAE, 0xAA, 0xBA, 0x7F, 0x81, 0xA0, 0xAB, 0xAB, 0xB2, 0x50, 0x50, 0x50], P_BALL), // "Poké Balls"
                        Pocket::new(&[0x93, 0x8C, 0xB2, 0x7F, 0xF3, 0x7F, 0x87, 0x8C, 0xB2, 0x50, 0x50, 0x50, 0x50], P_TMHM),// "TMs / HMs"
                        Pocket::new(&[0x8F, 0x82, 0x7F, 0x88, 0xB3, 0xA4, 0xAC, 0xB2, 0x50, 0x50, 0x50, 0x50, 0x50], P_PC)], // "PC Items"
            firered_items: [Pocket::new(&[0xFF], P_GENR), // We dont care about pocket name in FR
                            Pocket::new(&[0xFF], P_KEYI),
                            Pocket::new(&[0xFF], P_BALL),
                            Pocket::new(&[0xFF], P_TMHM),
                            Pocket::new(&[0xFF], P_PC),
                            Pocket::new(&[0xFF], P_BERY)],
            red_progress: 0,
            firered_progress: 0,
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
            first_warp: true,
            level_cap: 0
        }
    }

    pub fn from_file() -> std::io::Result<GameState> {
        if let Ok(f) = File::open("GameState") {
            println!("GameState found, loading from disk.\n");
            let loaded: GameState = bincode::deserialize_from(f).unwrap();
            let mut items = RED_ITEM_STATE.lock().unwrap();
            items.inventory = loaded.red_items.clone();
            Ok(loaded)
        } else {
            println!("No existing GameState found, using default.\n");
            let mut default = GameState::new();
            default.get_current_game();
            default.read_trainer_data();
            default.collect_mapstate();
            let mut items = RED_ITEM_STATE.lock().unwrap();
            items.inventory[P_KEYI as usize].content.push([0x0046, 0x0001]); // add oaks parcel when init
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

    pub fn save_state(&mut self) -> std::io::Result<()> {
        //println!("Saving GameState to disk.\n");
        let items = RED_ITEM_STATE.lock().unwrap();
        self.red_items = items.inventory.clone();
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

        self.read_items();

        let warp_enable = warp_enable.lock().unwrap();

        if *warp_enable {
            match &self.game {
                Game::RED => { 
                    println!("Supported map, starting transition to FIRE RED map: {:04x}, warp: {:02x}", map, warp);

                    self.hud_enable(false);

                    BIZHAWK.load_rom("firered.gba").unwrap();
                    BIZHAWK.load_state("firered_warp").unwrap();

                    self.game = Game::FIRERED;

                    self.write_trainer_data();

                    self.write_party();

                    self.write_misc();

                    self.write_items();

                    BIZHAWK.write_u16(&MemRegion::EWRAM, 0x0003_1DBC, map).unwrap();
                    BIZHAWK.write_u8(&MemRegion::EWRAM, 0x0003_1DBE, warp).unwrap();

                    BIZHAWK.frameadvance().unwrap();

                    BIZHAWK.write_u32(&MemRegion::EWRAM, 0x0003_1DB4, 0xFFFF_FFFF).unwrap();

                    self.map_state.previous_lastmap = 0xFFFF_FFFF;

                    self.send_hud_data();

                    self.hud_enable(true);

                    BIZHAWK.write_u8(&MemRegion::EWRAM, 0x0000_FC4, 0x01).unwrap();

                    BIZHAWK.play().unwrap();
                },
                Game::FIRERED => {
                    BIZHAWK.mute().unwrap();

                    println!("Supported map, starting transition to RED map: {:02x}, warp: {:02x}, Lastmap {:02x}", map, warp, last_map);

                    self.hud_enable(false);

                    BIZHAWK.load_rom("red.gb").unwrap();
                    if self.first_warp {
                        BIZHAWK.load_state("red_default").unwrap();
                        self.first_warp = false;
                        println!("first warp");
                    } else {
                        BIZHAWK.load_state("red_warp").unwrap();
                    }

                    self.game = Game::RED;

                    self.write_trainer_data();

                    self.write_party();

                    self.write_misc();

                    self.write_items();

                    BIZHAWK.on_memory_write("item_api", SYM["wItemAPICommand"].bus_addr as u32, 0x04, "http://localhost:5340/item_api").ok();

                    BIZHAWK.write_u8_sym(&SYM["wDestinationWarpID"], warp).unwrap();
                    BIZHAWK.write_u8_sym(&SYM["wCurMap"], map as u8).unwrap();
                    BIZHAWK.write_u8_sym(&SYM["wCurMapTileset"], 0x16).unwrap(); // Writing the 0x16 tileset fixes a bug with overworld maps
                    BIZHAWK.write_u8(&MemRegion::HRAM, 0x000B, map as u8).unwrap();

                    BIZHAWK.stop_drawing().unwrap();
                    BIZHAWK.clear_screen(0x00000000).unwrap();

                    self.send_hud_data();

                    self.hud_enable(true);

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
                        match *WARP_MODE.lock().unwrap() {
                            WarpState::RANDOM => {
                                if rand::random::<u16>() <= 0x2800 {
                                    self.handle_map_change(destination.0, destination.1, 0, warp_enable);
                                }
                            }, 
                            WarpState::LOCK_RED => {},
                            WarpState::LOCK_FIRERED => {
                                self.handle_map_change(destination.0, destination.1, 0, warp_enable);
                            }
                        }
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
                        match *WARP_MODE.lock().unwrap() {
                            WarpState::RANDOM => {
                                if rand::random::<u16>() <= 0x2800 {
                                    BIZHAWK.pause().unwrap();
                                    BIZHAWK.framerewind().unwrap();
                                    BIZHAWK.save_state("firered_warp").unwrap();
                                    self.save_state().unwrap();
                                    self.handle_map_change(destination.0 as u16, destination.1, destination.2, warp_enable);
                                }
                            }, 
                            WarpState::LOCK_RED => {
                                    BIZHAWK.pause().unwrap();
                                    BIZHAWK.framerewind().unwrap();
                                    BIZHAWK.save_state("firered_warp").unwrap();
                                    self.save_state().unwrap();
                                    self.handle_map_change(destination.0 as u16, destination.1, destination.2, warp_enable);
                            },
                            WarpState::LOCK_FIRERED => {}
                        }
                    } else {
                        BIZHAWK.play().unwrap();
                        println!("Map unknown or not supported.");
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

                self.badges[0] |= BIZHAWK.read_u8_sym(&SYM["wObtainedBadges"]).unwrap();
            },
            Game::FIRERED => {
                let options_word = LittleEndian::read_u16(&BIZHAWK.read_slice_custom("*0300500C+14/2".to_string(), 0x02).unwrap());

                self.options.text_speed = ((0x03 - min((options_word & 0x0003) as u8, 0x02)) * 2) - 1; // convert to red text speed
                self.options.animations = options_word & 0x0400 == 0x0000;
                self.options.switching = options_word & 0x0200 == 0x0000;

                let key = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap());
                self.money = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*03005008+290/4".to_string(), 0x04).unwrap()) ^ key;

                self.owned.copy_from_slice(&BIZHAWK.read_slice_custom("*0300500C+28/13".to_string(), 19).unwrap());
                self.seen.copy_from_slice(&BIZHAWK.read_slice_custom("*0300500C+5C/13".to_string(), 19).unwrap());

                self.badges[1] |= BIZHAWK.read_slice_custom("*03005008+EE0+104/1".to_string(), 0x01).unwrap()[0];
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

                // AND both game's badges together
                BIZHAWK.write_u8_sym(&SYM["wObtainedBadges"], &self.badges[0] & &self.badges[1]).unwrap();
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

                // AND both game's badges together
                BIZHAWK.write_u8(&MemRegion::EWRAM, pointer2 + 0xEE0 + 0x104, &self.badges[0] & &self.badges[1]).unwrap();
            }
        }
    }

    pub fn read_items(&mut self) {
        match &self.game {
            Game::RED => {
                let bag = RED_ITEM_STATE.lock().unwrap();
                for pocket in &mut self.firered_items.iter_mut() {
                    if pocket.id == P_GENR || pocket.id == P_BALL || pocket.id == P_TMHM {

                        let mut indices_to_remove: Vec<usize> = Vec::new();
                        let red_content = &bag.inventory[pocket.id as usize].content;
                        let fire_red_content = &mut pocket.content;

                        // Update quantities and record empty stacks for removal

                        for (j, fire_red_item) in fire_red_content.iter_mut().enumerate() {
                            let id = FIRERED_RED_ITEMS[fire_red_item[0] as usize] as u16;
                            if id != 0x0000 { // If the item ID is a valid item to be transfered
                                if let Some(red_item) = red_content.iter().find(|&x| {x[0] == id}) { // If we find it
                                    fire_red_item[1] = red_item[1]; // Update the count
                                } else {
                                    indices_to_remove.push(j); // Mark the index for removal
                                }
                            }
                        }

                        // Remove empty stacks, in reverse order

                        for i in rev(indices_to_remove) {
                            fire_red_content.remove(i);
                        }

                        // Add any new stacks into the pocket

                        for red_item in red_content.iter() {
                            let id = RED_FIRERED_ITEMS[red_item[0] as usize];
                            if id != 0x0000 { // If the item ID is a valid item to be transfered
                                if fire_red_content.iter().find(|&x| {x[0] == id}).is_none() { // If we dont find it
                                    fire_red_content.push([id, red_item[1]]); // Add the item to the pocket
                                }
                            }
                        }
                    }
                }
            },
            Game::FIRERED => {
                let key = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap()) as u16;

                let mut item_memory = BIZHAWK.read_slice_custom("*03005008+298/360".to_string(), 0x360).unwrap();

                // Decrypt
                for i in 0..(item_memory.len() / 0x04) {
                    let current_section = LittleEndian::read_u16(&item_memory[(i*0x04 + 0x02)..(i*0x04) + 0x04]);
                    LittleEndian::write_u16(&mut item_memory[(i*0x04 + 0x02)..(i*0x04) + 0x04], current_section ^ key);
                }

                for i in 0..5 {
                    let pocket = match i {
                        0 => &item_memory[0x0078..0x0120],
                        1 => &item_memory[0x0120..0x0198],
                        2 => &item_memory[0x0198..0x01CC],
                        3 => &item_memory[0x01CC..0x02B4],
                        4 => &item_memory[0x0000..0x0078], // PC
                        _ => &item_memory[0x02B4..0x0360]  // BERRY
                    };

                    self.firered_items[i].content.clear();

                    for j in 0..pocket.len() / 0x04 {
                        let id = LittleEndian::read_u16(&pocket[j*0x04..0x02 + j*0x04]);
                        let count = LittleEndian::read_u16(&pocket[0x02 + j*0x04..0x04 + j*0x04]);

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
                let mut bag = RED_ITEM_STATE.lock().unwrap();
                for pocket in &mut bag.inventory.iter_mut() {
                    if pocket.id == P_GENR || pocket.id == P_BALL || pocket.id == P_TMHM {

                        let mut indices_to_remove: Vec<usize> = Vec::new();
                        let red_content = &mut pocket.content;
                        let fire_red_content = &self.firered_items[pocket.id as usize].content;

                        // Update quantities and record empty stacks for removal

                        for (j, red_item) in red_content.iter_mut().enumerate() {
                            let id = RED_FIRERED_ITEMS[red_item[0] as usize];
                            if id != 0x0000 { // If the item ID is a valid item to be transfered
                                if let Some(fire_red_item) = fire_red_content.iter().find(|&x| {x[0] == id}) { // If we find it
                                    red_item[1] = fire_red_item[1]; // Update the count
                                } else {
                                    indices_to_remove.push(j); // Mark the index for removal
                                }
                            }
                        }

                        // Remove empty stacks, in reverse order

                        for i in rev(indices_to_remove) {
                            red_content.remove(i);
                        }

                        // Add any new stacks into the pocket

                        for fire_red_item in fire_red_content.iter() {
                            let id = FIRERED_RED_ITEMS[fire_red_item[0] as usize] as u16;
                            if id != 0x0000 { // If the item ID is a valid item to be transfered
                                if red_content.iter().find(|&x| {x[0] == id}).is_none() { // If we dont find it
                                    red_content.push([id, fire_red_item[1]]); // Add the item to the pocket
                                }
                            }
                        }
                    }
                }
            },
            Game::FIRERED => {
                let items_pointer = BIZHAWK.read_u32(&MemRegion::IWRAM, 0x5008).unwrap() & 0x00FFFFFF;
                let key = LittleEndian::read_u32(&BIZHAWK.read_slice_custom("*0300500C+F20/4".to_string(), 0x04).unwrap()) as u16;

                let mut item_memory = vec![0u8;0x1C4];

                for i in 0..3 {
                    let section = match i {
                        0 => &mut item_memory[0x0000..0x00A8],
                        1 => &mut item_memory[0x00A8..0x00DC],
                        _ => &mut item_memory[0x00DC..0x01C4]
                    };

                    let pocket = match i {
                        0 => &self.firered_items[0].content,
                        1 => &self.firered_items[2].content,
                        _ => &self.firered_items[3].content,
                    };

                    for j in 0..section.len() / 0x04 {
                        if j < pocket.len() {
                            LittleEndian::write_u16(&mut section[j*0x04..0x02 + j*0x04], pocket[j][0]);
                            LittleEndian::write_u16(&mut section[0x02 + j*0x04..0x04 + j*0x04], pocket[j][1] ^ key);
                        } else {
                            LittleEndian::write_u16(&mut section[j*0x04..0x02 + j*0x04], 0x00);
                            LittleEndian::write_u16(&mut section[0x02 + j*0x04..0x04 + j*0x04], 0x00 ^ key);
                        }
                    }

                    BIZHAWK.write_slice(&MemRegion::EWRAM, items_pointer + 0x310, &item_memory[0x0000..0x00A8]).unwrap();
                    BIZHAWK.write_slice(&MemRegion::EWRAM, items_pointer + 0x430, &item_memory[0x00A8..0x01C4]).unwrap();
                }
            }
        }
    }
    pub fn send_hud_data(&mut self) {
        let data = match &self.game {
            Game::RED => {
                HUDData {
                    game: Some("red.gb".to_string()),
                    badges: Some(((self.badges[0] as u16) << 8) | ((self.badges[1] as u16) & 0xFF)),
                    items: Some(Items::from_pocket_data(&RED_ITEM_STATE.lock().unwrap().inventory, &self.firered_items, &self.game)),
                    updates_paused: None
                }
            },
            Game::FIRERED => {
                HUDData {
                    game: Some("firered.gba".to_string()),
                    badges: Some(((self.badges[0] as u16) << 8) | ((self.badges[1] as u16) & 0xFF)),
                    items: Some(Items::from_pocket_data(&RED_ITEM_STATE.lock().unwrap().inventory, &self.firered_items, &self.game)),
                    updates_paused: None
                }
            }
        };
        HUD.post("http://localhost:1337/override").body(to_string(&data).unwrap()).send().ok();
    }
    pub fn hud_enable(&mut self, enable: bool) {
        let data = HUDData {
            game: None,
            badges: None,
            items: None,
            updates_paused: Some(!enable)
        };
        HUD.post("http://localhost:1337/override").body(to_string(&data).unwrap()).send().ok();
    }
    pub fn recurring_functions(&mut self) {
        self.check_for_new_mon();
        self.check_progress_gates();
        self.get_lockdown_level_cap();
        self.enforce_level_cap();
        /*println!("Level cap: {}, Warp_state: {}", self.level_cap, match *WARP_MODE.lock().unwrap() {
            WarpState::RANDOM => "RANDOM",
            WarpState::LOCK_FIRERED => "LOCK FIRERED",
            WarpState::LOCK_RED => "LOCK RED"
        });*/
    }
    pub fn check_for_new_mon(&mut self) {
        match &self.game {
            Game::RED => {
                // PARTY
                let party_count = BIZHAWK.read_u8_sym(&SYM["wPartyCount"]).unwrap() as u32;
                for i in 0..6 {
                    if i < party_count {
                        let pk1 = &BIZHAWK.read_slice(&MemRegion::WRAM, SYM["wPartyMons"].addr as u32 + (44*i), 44).unwrap();
                        let nickname = &BIZHAWK.read_slice(&MemRegion::WRAM, SYM["wPartyMonNicks"].addr as u32 + (11*i), 11).unwrap();

                        if RED_FIRERED_SPECIES[pk1[0x00] as usize] != 0 {
                            let mut uid = get_uid(RED_FIRERED_SPECIES[pk1[0x00] as usize], pk1[0x07]);

                            if uid == 0x0000 {
                                println!("found new mon in party, {}", RED_FIRERED_SPECIES[pk1[0x00] as usize]);
                                let mut j = 1;
                                loop {
                                    uid = get_uid(RED_FIRERED_SPECIES[pk1[0x00] as usize], j as u8);
                                    if uid != 0x0000 && !self.pokemon_list.contains_key(&uid) {
                                        let new_pokemon = Pokemon::from_pk1(pk1, nickname, &self.trainer, uid);
                                        if let Some(ref gen1) = new_pokemon.gen1 {
                                            BIZHAWK.write_u8(&MemRegion::WRAM, SYM["wPartyMons"].addr as u32 + (44*i) + 0x07, gen1.bytes[0x07]).unwrap(); // write the second part of the ID to the mon
                                        } else {
                                            panic!("Trying to write Gen 1 data but Gen 1 data is not generated yet");
                                        }
                                        self.pokemon_list.insert(uid, new_pokemon);
                                        break;
                                    }
                                    j += 1;
                                }
                            } 
                        }
                    } else {
                        break;
                    }
                }
                // CURRENT BOX
                let box_count = BIZHAWK.read_u8_sym(&SYM["wNumInBox"]).unwrap() as u32;
                for i in 0..20 {
                    if i < box_count {
                        let pk1 = &mut BIZHAWK.read_slice(&MemRegion::WRAM, SYM["wBoxMons"].addr as u32 + (33*i), 33).unwrap().to_vec();
                        pk1.extend(vec![0xAA;11]);
                        let nickname = &BIZHAWK.read_slice(&MemRegion::WRAM, SYM["wBoxMonNicks"].addr as u32 + (11*i), 11).unwrap();

                        if RED_FIRERED_SPECIES[pk1[0x00] as usize] != 0 {
                            let mut uid = get_uid(RED_FIRERED_SPECIES[pk1[0x00] as usize], pk1[0x07]);

                            if uid == 0x0000 {
                                println!("found new mon in box, {}", RED_FIRERED_SPECIES[pk1[0x00] as usize]);
                                let mut j = 1;
                                loop {
                                    uid = get_uid(RED_FIRERED_SPECIES[pk1[0x00] as usize], j as u8);
                                    if uid != 0x0000 && !self.pokemon_list.contains_key(&uid) {
                                        let new_pokemon = Pokemon::from_pk1(pk1, nickname, &self.trainer, uid);
                                        if let Some(ref gen1) = new_pokemon.gen1 {
                                            BIZHAWK.write_u8(&MemRegion::WRAM, SYM["wBoxMons"].addr as u32 + (33*i) + 0x07, gen1.bytes[0x07]).unwrap(); // write the second part of the ID to the mon
                                        } else {
                                            panic!("Trying to write Gen 1 data but Gen 1 data is not generated yet");
                                        }
                                        self.pokemon_list.insert(uid, new_pokemon);
                                        break;
                                    }
                                    j += 1;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            },
            Game::FIRERED => {
                
            }
        }
    }

    pub fn get_lockdown_level_cap(&mut self) {
        for i in LEVEL_CAPS.iter() {
            match self.compare_progress(i.0) {
                0 => {}, // continue to the next one
                1 => {
                    *WARP_MODE.lock().unwrap() = WarpState::LOCK_FIRERED;
                    self.level_cap = i.1;
                    break;
                },
                2 => {
                    *WARP_MODE.lock().unwrap() = WarpState::LOCK_RED;
                    self.level_cap = i.1;
                    break;
                },
                3 => {
                    *WARP_MODE.lock().unwrap() = WarpState::RANDOM;
                    self.level_cap = i.1;
                    break;
                },
                _ => {}
            }
        }
    }

    pub fn compare_progress(&self, flag: u32) -> u8 {
        if self.red_progress & self.firered_progress & flag != 0 {
            return 0; // they both have the flag
        } else if self.red_progress & flag != 0 {
            return 1; // red has the flag set, but not firered
        } else if self.firered_progress & flag != 0 {
            return 2; // firered has the flag set, but not red
        } else {
            return 3; // neither are set
        }
    }

    pub fn enforce_level_cap(&mut self) {
         match &self.game {
            Game::RED => {
                let party_count = BIZHAWK.read_u8_sym(&SYM["wPartyCount"]).unwrap();
                let mut allowed_exp = BIZHAWK.read_u8_sym(&SYM["wPartyGainExpFlags"]).unwrap(); // get the current state of allowed exp
                for i in 0..party_count {
                    if BIZHAWK.read_u8(&MemRegion::WRAM, SYM["wPartyMons"].addr as u32 + (44*i as u32) + 0x21).unwrap() >= self.level_cap {
                        allowed_exp &= !(0x01 << i); // reset any bit position where the mon is at or over the level cap, to disable EXP gain
                    }
                }
                BIZHAWK.write_u8_sym(&SYM["wPartyGainExpFlags"], allowed_exp).unwrap(); // write the new value

            },
            Game::FIRERED => {
                let mut allowed_exp = BIZHAWK.read_u8(&MemRegion::EWRAM, 0x0002_3F4E).unwrap(); // get the current state of allowed exp
                for i in 0..6 {
                    if BIZHAWK.read_u8(&MemRegion::EWRAM, 0x0002_4284 + (100*i as u32) + 0x54).unwrap() >= self.level_cap {
                        allowed_exp &= !(0x01 << i); // reset any bit position where the mon is at or over the level cap, to disable EXP gain
                    }
                }
                BIZHAWK.write_u8(&MemRegion::EWRAM, 0x0002_3F4E, allowed_exp).unwrap();
            }
        };
    }

    pub fn check_progress_gates(&mut self) {

        self.read_misc();
        self.read_items();

        // badges
        self.red_progress |= self.badges[0] as u32;
        self.firered_progress |= self.badges[1] as u32;
        match &self.game {
            Game::RED => {
                if self.red_progress & G_GUARDRUNK == 0x00 {
                    if BIZHAWK.read_u8_sym(&SYM["wd728"]).unwrap() & 0x40 != 0x00 { // bit 6 is guard drink
                        self.red_progress |= G_GUARDRUNK;
                    }
                }
                if self.red_progress & G_DLVRD_PARCEL == 0x00 {
                    if BIZHAWK.read_u8(&MemRegion::WRAM, SYM["wEventFlags"].addr as u32 + 7).unwrap() & 0x01 != 0x00 { // bit 0 is delivered oaks parcel
                        self.red_progress |= G_DLVRD_PARCEL;
                    }
                }
            },
            Game::FIRERED => {
                if self.firered_progress & G_GUARDRUNK == 0x00 {
                    if LittleEndian::read_u16(&BIZHAWK.read_slice_custom("*03005008+1000+C4/2".to_string(), 0x02).unwrap()) == 0x0001 {
                        self.firered_progress |= G_GUARDRUNK;
                    }
                }
                if self.firered_progress & G_DLVRD_PARCEL == 0x00 {
                    if BIZHAWK.read_slice_custom("*03005008+EE0+7/1".to_string(), 0x01).unwrap()[0] & 0x01 != 0x00 { // bit 2 is delivered oaks parcel
                        self.firered_progress |= G_DLVRD_PARCEL;
                    }
                }
            }
        }

        let mut red_items = RED_ITEM_STATE.lock().unwrap();
        // key items / hms // This both checks the flag and removes the items if the other games doesent have them.
        self.check_item(&mut red_items, G_SILPHSCOPE, 0x48, 0x0167, P_TMHM);
        self.check_item(&mut red_items, G_POKEFLUTE, 0x49, 0x015E, P_TMHM);
        self.check_item(&mut red_items, G_SSTICKET, 0x3F, 0x0109, P_TMHM);
        self.check_item(&mut red_items, G_SECRETKEY, 0x2B, 0x015F, P_TMHM);
        self.check_item(&mut red_items, G_CUT, 0xC4, 0x0153, P_TMHM);
        self.check_item(&mut red_items, G_SURF, 0xC6, 0x0155, P_TMHM);
        self.check_item(&mut red_items, G_STRENGTH, 0xC7, 0x0156, P_TMHM);

    }

    pub fn check_item(&mut self, red_items: &mut ApiState, flag_to_check: u32, item_to_check: u16, item_to_add: u16, pocket: u8) {
        self.check_item_red(red_items, flag_to_check, item_to_check, item_to_add, pocket as usize);
        self.check_item_firered(red_items, flag_to_check, item_to_check, item_to_add, pocket as usize);
    }

    pub fn check_item_red(&mut self, red_items: &mut ApiState, flag_to_check: u32, item_to_check: u16, item_to_add: u16, pocket: usize) {
        if self.red_progress & flag_to_check == 0x00 { // if the flag is unset
            if let Some(index) = &red_items.inventory[pocket].has_item(item_to_check) { // if we have the item
                self.red_progress |= flag_to_check; // set the flag
                if self.firered_progress & flag_to_check == 0x00 { // if the other game doesent have the item
                    red_items.inventory[pocket].remove_item(*index); // remove it from our inventory
                } else {
                    self.firered_items[pocket].add_item(item_to_add, 0x01); // else add it to the other game ( it would have been removed by this point )
                }
            }
        }
    }
    pub fn check_item_firered(&mut self, red_items: &mut ApiState, flag_to_check: u32, item_to_check: u16, item_to_add: u16, pocket: usize) {
        if self.firered_progress & flag_to_check == 0x00 {
            if let Some(index) = &self.firered_items[pocket].has_item(item_to_check) {
                self.firered_progress |= flag_to_check;
                if self.red_progress & flag_to_check == 0x00 {
                    self.firered_items[pocket].remove_item(*index);
                } else {
                    red_items.inventory[pocket].add_item(item_to_add, 0x01);
                }
            }
        }
    }
}

pub fn make_backup(manual: bool) {
    let pathstr = &CONTROLLER_PATH.to_str().unwrap();
    let backup_name = format!("{} {}", if manual {"ManualSave"} else {"AutoSave"}, Utc::now().format("%Y-%m-%d-%H%M%S"));
    std::fs::create_dir(format!("{}\\Backups\\{}", pathstr, backup_name)).ok();
    std::fs::copy(format!("{}\\States\\red_warp.State", pathstr),
                  format!("{}\\Backups\\{}\\red_warp.State", pathstr, backup_name)).ok();
    std::fs::copy(format!("{}\\States\\firered_warp.State", pathstr),
                  format!("{}\\Backups\\{}\\firered_warp.State", pathstr, backup_name)).ok();
    std::fs::copy(format!("{}\\Gamestate", pathstr),
                  format!("{}\\Backups\\{}\\Gamestate", pathstr, backup_name)).ok();
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
pub enum WarpState {
    RANDOM,
    LOCK_RED,
    LOCK_FIRERED
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
    pub fn has_item(&mut self, item: u16) -> Option<usize> {
        self.content.iter().position(|&x| {x[0] == item})
    }
    pub fn remove_item(&mut self, index: usize) {
        self.content.remove(index);
    }
    pub fn add_item(&mut self, item: u16, count: u16) {
        self.content.push([item, count]);
    }
}