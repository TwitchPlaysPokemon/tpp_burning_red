use crate::gamestate::WarpState;
use crate::bizhawk;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time;
use crate::item_api::ApiState;
use crate::warp_connections::*;
use std::sync::{Arc, Mutex};

// Red Map IDs

pub const PALLET_TOWN: u8 = 0x00;
pub const VIRIDIAN_CITY: u8 = 0x01;
pub const PEWTER_CITY: u8 = 0x02;
pub const CERULEAN_CITY: u8 = 0x03;
pub const LAVENDER_TOWN: u8 = 0x04;
pub const VERMILION_CITY: u8 = 0x05;
pub const CELADON_CITY: u8 = 0x06;
pub const FUCHSIA_CITY: u8 = 0x07;
pub const CINNABAR_ISLAND: u8 = 0x08;
pub const INDIGO_PLATEAU: u8 = 0x09;
pub const SAFFRON_CITY: u8 = 0x0A;
pub const ROUTE_1: u8 = 0x0C;
pub const ROUTE_2: u8 = 0x0D;
pub const ROUTE_3: u8 = 0x0E;
pub const ROUTE_4: u8 = 0x0F;
pub const ROUTE_5: u8 = 0x10;
pub const ROUTE_6: u8 = 0x11;
pub const ROUTE_7: u8 = 0x12;
pub const ROUTE_8: u8 = 0x13;
pub const ROUTE_9: u8 = 0x14;
pub const ROUTE_10: u8 = 0x15;
pub const ROUTE_11: u8 = 0x16;
pub const ROUTE_12: u8 = 0x17;
pub const ROUTE_13: u8 = 0x18;
pub const ROUTE_14: u8 = 0x19;
pub const ROUTE_15: u8 = 0x1A;
pub const ROUTE_16: u8 = 0x1B;
pub const ROUTE_17: u8 = 0x1C;
pub const ROUTE_18: u8 = 0x1D;
pub const ROUTE_19: u8 = 0x1E;
pub const ROUTE_20: u8 = 0x1F;
pub const ROUTE_21: u8 = 0x20;
pub const ROUTE_22: u8 = 0x21;
pub const ROUTE_23: u8 = 0x22;
pub const ROUTE_24: u8 = 0x23;
pub const ROUTE_25: u8 = 0x24;
pub const REDS_HOUSE_1F: u8 = 0x25;
pub const REDS_HOUSE_2F: u8 = 0x26;
pub const BLUES_HOUSE: u8 = 0x27;
pub const OAKS_LAB: u8 = 0x28;
pub const VIRIDIAN_POKECENTER: u8 = 0x29;
pub const VIRIDIAN_MART: u8 = 0x2A;
pub const VIRIDIAN_SCHOOL: u8 = 0x2B;
pub const VIRIDIAN_HOUSE: u8 = 0x2C;
pub const VIRIDIAN_GYM: u8 = 0x2D;
pub const DIGLETTS_CAVE_EXIT: u8 = 0x2E;
pub const VIRIDIAN_FOREST_EXIT: u8 = 0x2F;
pub const ROUTE_2_HOUSE: u8 = 0x30;
pub const ROUTE_2_GATE: u8 = 0x31;
pub const VIRIDIAN_FOREST_ENTRANCE: u8 = 0x32;
pub const VIRIDIAN_FOREST: u8 = 0x33;
pub const MUSEUM_1F: u8 = 0x34;
pub const MUSEUM_2F: u8 = 0x35;
pub const PEWTER_GYM: u8 = 0x36;
pub const PEWTER_HOUSE_1: u8 = 0x37;
pub const PEWTER_MART: u8 = 0x38;
pub const PEWTER_HOUSE_2: u8 = 0x39;
pub const PEWTER_POKECENTER: u8 = 0x3A;
pub const MT_MOON_1: u8 = 0x3B;
pub const MT_MOON_2: u8 = 0x3C;
pub const MT_MOON_3: u8 = 0x3D;
pub const TRASHED_HOUSE: u8 = 0x3E;
pub const CERULEAN_HOUSE_1: u8 = 0x3F;
pub const CERULEAN_POKECENTER: u8 = 0x40;
pub const CERULEAN_GYM: u8 = 0x41;
pub const BIKE_SHOP: u8 = 0x42;
pub const CERULEAN_MART: u8 = 0x43;
pub const MT_MOON_POKECENTER: u8 = 0x44;
pub const ROUTE_5_GATE: u8 = 0x46;
pub const PATH_ENTRANCE_ROUTE_5: u8 = 0x47;
pub const DAYCAREM: u8 = 0x48;
pub const ROUTE_6_GATE: u8 = 0x49;
pub const PATH_ENTRANCE_ROUTE_6: u8 = 0x4A;
pub const ROUTE_7_GATE: u8 = 0x4C;
pub const PATH_ENTRANCE_ROUTE_7: u8 = 0x4D;
pub const ROUTE_8_GATE: u8 = 0x4F;
pub const PATH_ENTRANCE_ROUTE_8: u8 = 0x50;
pub const ROCK_TUNNEL_POKECENTER: u8 = 0x51;
pub const ROCK_TUNNEL_1: u8 = 0x52;
pub const POWER_PLANT: u8 = 0x53;
pub const ROUTE_11_GATE_1F: u8 = 0x54;
pub const DIGLETTS_CAVE_ENTRANCE: u8 = 0x55;
pub const ROUTE_11_GATE_2F: u8 = 0x56;
pub const ROUTE_12_GATE_1F: u8 = 0x57;
pub const BILLS_HOUSE: u8 = 0x58;
pub const VERMILION_POKECENTER: u8 = 0x59;
pub const POKEMON_FAN_CLUB: u8 = 0x5A;
pub const VERMILION_MART: u8 = 0x5B;
pub const VERMILION_GYM: u8 = 0x5C;
pub const VERMILION_HOUSE_1: u8 = 0x5D;
pub const VERMILION_DOCK: u8 = 0x5E;
pub const SS_ANNE_1: u8 = 0x5F;
pub const SS_ANNE_2: u8 = 0x60;
pub const SS_ANNE_3: u8 = 0x61;
pub const SS_ANNE_4: u8 = 0x62;
pub const SS_ANNE_5: u8 = 0x63;
pub const SS_ANNE_6: u8 = 0x64;
pub const SS_ANNE_7: u8 = 0x65;
pub const SS_ANNE_8: u8 = 0x66;
pub const SS_ANNE_9: u8 = 0x67;
pub const SS_ANNE_10: u8 = 0x68;
pub const VICTORY_ROAD_1: u8 = 0x6C;
pub const LANCES_ROOM: u8 = 0x71;
pub const HALL_OF_FAME: u8 = 0x76;
pub const UNDERGROUND_PATH_NS: u8 = 0x77;
pub const CHAMPIONS_ROOM: u8 = 0x78;
pub const UNDERGROUND_PATH_WE: u8 = 0x79;
pub const CELADON_MART_1: u8 = 0x7A;
pub const CELADON_MART_2: u8 = 0x7B;
pub const CELADON_MART_3: u8 = 0x7C;
pub const CELADON_MART_4: u8 = 0x7D;
pub const CELADON_MART_ROOF: u8 = 0x7E;
pub const CELADON_MART_ELEVATOR: u8 = 0x7F;
pub const CELADON_MANSION_1: u8 = 0x80;
pub const CELADON_MANSION_2: u8 = 0x81;
pub const CELADON_MANSION_3: u8 = 0x82;
pub const CELADON_MANSION_4: u8 = 0x83;
pub const CELADON_MANSION_5: u8 = 0x84;
pub const CELADON_POKECENTER: u8 = 0x85;
pub const CELADON_GYM: u8 = 0x86;
pub const GAME_CORNER: u8 = 0x87;
pub const CELADON_MART_5: u8 = 0x88;
pub const CELADON_PRIZE_ROOM: u8 = 0x89;
pub const CELADON_DINER: u8 = 0x8A;
pub const CELADON_HOUSE: u8 = 0x8B;
pub const CELADON_HOTEL: u8 = 0x8C;
pub const LAVENDER_POKECENTER: u8 = 0x8D;
pub const POKEMONTOWER_1: u8 = 0x8E;
pub const POKEMONTOWER_2: u8 = 0x8F;
pub const POKEMONTOWER_3: u8 = 0x90;
pub const POKEMONTOWER_4: u8 = 0x91;
pub const POKEMONTOWER_5: u8 = 0x92;
pub const POKEMONTOWER_6: u8 = 0x93;
pub const POKEMONTOWER_7: u8 = 0x94;
pub const LAVENDER_HOUSE_1: u8 = 0x95;
pub const LAVENDER_MART: u8 = 0x96;
pub const LAVENDER_HOUSE_2: u8 = 0x97;
pub const FUCHSIA_MART: u8 = 0x98;
pub const FUCHSIA_HOUSE_1: u8 = 0x99;
pub const FUCHSIA_POKECENTER: u8 = 0x9A;
pub const FUCHSIA_HOUSE_2: u8 = 0x9B;
pub const SAFARI_ZONE_ENTRANCE: u8 = 0x9C;
pub const FUCHSIA_GYM: u8 = 0x9D;
pub const FUCHSIA_MEETING_ROOM: u8 = 0x9E;
pub const SEAFOAM_ISLANDS_2: u8 = 0x9F;
pub const SEAFOAM_ISLANDS_3: u8 = 0xA0;
pub const SEAFOAM_ISLANDS_4: u8 = 0xA1;
pub const SEAFOAM_ISLANDS_5: u8 = 0xA2;
pub const VERMILION_HOUSE_2: u8 = 0xA3;
pub const FUCHSIA_HOUSE_3: u8 = 0xA4;
pub const MANSION_1: u8 = 0xA5;
pub const CINNABAR_GYM: u8 = 0xA6;
pub const CINNABAR_LAB_1: u8 = 0xA7;
pub const CINNABAR_LAB_2: u8 = 0xA8;
pub const CINNABAR_LAB_3: u8 = 0xA9;
pub const CINNABAR_LAB_4: u8 = 0xAA;
pub const CINNABAR_POKECENTER: u8 = 0xAB;
pub const CINNABAR_MART: u8 = 0xAC;
pub const INDIGO_PLATEAU_LOBBY: u8 = 0xAE;
pub const COPYCATS_HOUSE_1F: u8 = 0xAF;
pub const COPYCATS_HOUSE_2F: u8 = 0xB0;
pub const FIGHTING_DOJO: u8 = 0xB1;
pub const SAFFRON_GYM: u8 = 0xB2;
pub const SAFFRON_HOUSE_1: u8 = 0xB3;
pub const SAFFRON_MART: u8 = 0xB4;
pub const SILPH_CO_1F: u8 = 0xB5;
pub const SAFFRON_POKECENTER: u8 = 0xB6;
pub const SAFFRON_HOUSE_2: u8 = 0xB7;
pub const ROUTE_15_GATE_1F: u8 = 0xB8;
pub const ROUTE_15_GATE_2F: u8 = 0xB9;
pub const ROUTE_16_GATE_1F: u8 = 0xBA;
pub const ROUTE_16_GATE_2F: u8 = 0xBB;
pub const ROUTE_16_HOUSE: u8 = 0xBC;
pub const ROUTE_12_HOUSE: u8 = 0xBD;
pub const ROUTE_18_GATE_1F: u8 = 0xBE;
pub const ROUTE_18_GATE_2F: u8 = 0xBF;
pub const SEAFOAM_ISLANDS_1: u8 = 0xC0;
pub const ROUTE_22_GATE: u8 = 0xC1;
pub const VICTORY_ROAD_2: u8 = 0xC2;
pub const ROUTE_12_GATE_2F: u8 = 0xC3;
pub const VERMILION_HOUSE_3: u8 = 0xC4;
pub const DIGLETTS_CAVE: u8 = 0xC5;
pub const VICTORY_ROAD_3: u8 = 0xC6;
pub const ROCKET_HIDEOUT_1: u8 = 0xC7;
pub const ROCKET_HIDEOUT_2: u8 = 0xC8;
pub const ROCKET_HIDEOUT_3: u8 = 0xC9;
pub const ROCKET_HIDEOUT_4: u8 = 0xCA;
pub const ROCKET_HIDEOUT_ELEVATOR: u8 = 0xCB;
pub const SILPH_CO_2F: u8 = 0xCF;
pub const SILPH_CO_3F: u8 = 0xD0;
pub const SILPH_CO_4F: u8 = 0xD1;
pub const SILPH_CO_5F: u8 = 0xD2;
pub const SILPH_CO_6F: u8 = 0xD3;
pub const SILPH_CO_7F: u8 = 0xD4;
pub const SILPH_CO_8F: u8 = 0xD5;
pub const MANSION_2: u8 = 0xD6;
pub const MANSION_3: u8 = 0xD7;
pub const MANSION_4: u8 = 0xD8;
pub const SAFARI_ZONE_EAST: u8 = 0xD9;
pub const SAFARI_ZONE_NORTH: u8 = 0xDA;
pub const SAFARI_ZONE_WEST: u8 = 0xDB;
pub const SAFARI_ZONE_CENTER: u8 = 0xDC;
pub const SAFARI_ZONE_REST_HOUSE_1: u8 = 0xDD;
pub const SAFARI_ZONE_SECRET_HOUSE: u8 = 0xDE;
pub const SAFARI_ZONE_REST_HOUSE_2: u8 = 0xDF;
pub const SAFARI_ZONE_REST_HOUSE_3: u8 = 0xE0;
pub const SAFARI_ZONE_REST_HOUSE_4: u8 = 0xE1;
pub const UNKNOWN_DUNGEON_2: u8 = 0xE2;
pub const UNKNOWN_DUNGEON_3: u8 = 0xE3;
pub const UNKNOWN_DUNGEON_1: u8 = 0xE4;
pub const NAME_RATERS_HOUSE: u8 = 0xE5;
pub const CERULEAN_HOUSE_2: u8 = 0xE6;
pub const ROCK_TUNNEL_2: u8 = 0xE8;
pub const SILPH_CO_9F: u8 = 0xE9;
pub const SILPH_CO_10F: u8 = 0xEA;
pub const SILPH_CO_11F: u8 = 0xEB;
pub const SILPH_CO_ELEVATOR: u8 = 0xEC;
pub const TRADE_CENTER: u8 = 0xEF;
pub const COLOSSEUM: u8 = 0xF0;
pub const LORELEIS_ROOM: u8 = 0xF5;
pub const BRUNOS_ROOM: u8 = 0xF6;
pub const AGATHAS_ROOM: u8 = 0xF7;
pub const GLITCH_CELEDON_CITY: u8 = 0xF8;
pub const BATTLE_TENT_CORRUPT: u8 = 0xF9;

// Red Item API codes

pub const ITEM_FALSE: u8 = 0x00;
pub const ITEM_TRUE: u8 = 0x01;
pub const ITEM_NULL: u8 = 0x02;
pub const ITEM_TIMEOUT: u8 = 0x03;

pub const ITEM_LOCK: u8 = 0x04;
pub const ITEM_UNLOCK: u8 = 0x05;
pub const ITEM_INITIALIZE_ITEM_LISTS: u8 = 0x06;
pub const ITEM_ERASE_SAVED_DATA: u8 = 0x07;
pub const ITEM_SAVE: u8 = 0x08;
pub const ITEM_LOAD: u8 = 0x09;
pub const ITEM_CAN_GET_ITEM: u8 = 0x10;
pub const ITEM_ADD_ITEM: u8 = 0x11;
pub const ITEM_HAS_ITEM: u8 = 0x12;
pub const ITEM_REMOVE_ITEM: u8 = 0x13;
pub const ITEM_CAN_GET_PC_ITEM: u8 = 0x14;
pub const ITEM_ADD_ITEM_TO_PC: u8 = 0x15;
pub const ITEM_HAS_ITEM_IN_PC: u8 = 0x16;
pub const ITEM_REMOVE_ITEM_FROM_PC: u8 = 0x17;
pub const ITEM_DEPOSIT: u8 = 0x18;
pub const ITEM_WITHDRAW: u8 = 0x19;
pub const ITEM_SWAP_ITEMS: u8 = 0x1A;
pub const ITEM_SWAP_PC_ITEMS: u8 = 0x1B;
pub const ITEM_IS_BAG_EMPTY: u8 = 0x1C;
pub const ITEM_IS_PC_EMPTY: u8 = 0x1D;
pub const ITEM_GET_ITEM_QUANTITIES: u8 = 0x1E;
pub const ITEM_GET_PAGE_LIMITS: u8 = 0x1F;

pub const POLL_DELAY: time::Duration = time::Duration::from_millis(5);

#[derive(Debug, Clone)]
pub struct PKBase {
    pub stats: [u16;6],
    pub _type: [u8;2],
    pub catch_rate: u8,
    pub exp_growth: u8,
    pub gender_ratio: u8,
}

pub struct BaseStats {
    pub gen1: PKBase,
    pub gen3: PKBase
}

impl BaseStats {
    pub fn from_bytes(gen1_bytes: &[u8], gen2_bytes: &[u8], gen3_bytes: &[u8]) -> Vec<BaseStats> {
        let mut stats_vec = Vec::new();
        for i in 0..152 {
            let mut current_mon = &gen1_bytes[(i*0x1C)..(i*0x1C)+0x1C];
            let gen1 = PKBase {
                stats: [u16::from(current_mon[0x01]),
                        u16::from(current_mon[0x02]),
                        u16::from(current_mon[0x03]),
                        u16::from(current_mon[0x04]),
                        u16::from(current_mon[0x05]),
                        u16::from(current_mon[0x05])],
                _type: [current_mon[0x06], current_mon[0x07]],
                catch_rate: current_mon[0x08],
                exp_growth: current_mon[0x13],
                gender_ratio: gen2_bytes[i*0x20 + 0xD]
            };
            current_mon = &gen3_bytes[(i*0x1C)..(i*0x1C)+0x1C];
            let gen3 = PKBase {
                stats: [u16::from(current_mon[0x00]),
                        u16::from(current_mon[0x01]),
                        u16::from(current_mon[0x02]),
                        u16::from(current_mon[0x03]),
                        u16::from(current_mon[0x04]),
                        u16::from(current_mon[0x05])],
                _type: [current_mon[0x06], current_mon[0x07]],
                catch_rate: 0x00,
                exp_growth: current_mon[0x13],
                gender_ratio: current_mon[0x10],
            };
              stats_vec.push(BaseStats {gen1, gen3});
        }
        stats_vec
    }
}

lazy_static! {
    pub static ref RED_FIRERED_WARP_MAP: HashMap<(u8, u8, u8), (u16, u8)> = get_connections_red_firered();
    pub static ref FIRERED_RED_WARP_MAP: HashMap<(u16, u8), (u8, u8, u8)> = get_connections_firered_red();
    static ref GEN_1_BYTES: &'static [u8] = include_bytes!("personal_rb");
    static ref GEN_2_BYTES: &'static [u8] = include_bytes!("personal_gs"); // for gender ratios
    static ref GEN_3_BYTES: &'static [u8] = include_bytes!("personal_fr");
    pub static ref BASE_STATS: Vec<BaseStats> = BaseStats::from_bytes(&GEN_1_BYTES, &GEN_2_BYTES, &GEN_3_BYTES);
    pub static ref BIZHAWK: bizhawk::Bizhawk = bizhawk::Bizhawk::new(5337);
    pub static ref RED_ITEM_STATE: Arc<Mutex<ApiState>> = Arc::new(Mutex::new(ApiState::new()));
    pub static ref SYMFILE_STRING: String = bizhawk::load_symfile_text().unwrap();
    pub static ref SYM: HashMap<&'static str, bizhawk::SymEntry> = bizhawk::load_symfile(&SYMFILE_STRING);
    pub static ref HUD: reqwest::Client = reqwest::Client::new();
    pub static ref OAKS_PARCEL_OBTAINED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref CONTROLLER_PATH: std::path::PathBuf = std::env::current_dir().unwrap();
    pub static ref WARP_MODE: Arc<Mutex<WarpState>> = Arc::new(Mutex::new(WarpState::RANDOM));
}
                                          
pub static NATURE_EFFECTS: [[f32;5];25] = [/*ATK, DEF, SPE, SPA, SPD*/
                                /*Hardy 00*/[1.0, 1.0, 1.0, 1.0, 1.0],
                               /*Lonely 01*/[1.1, 0.9, 1.0, 1.0, 1.0],
                                /*Brave 02*/[1.1, 1.0, 0.9, 1.0, 1.0],
                              /*Adamant 03*/[1.1, 1.0, 1.0, 0.9, 1.0],
                              /*Naughty 04*/[1.1, 1.0, 1.0, 1.0, 0.9],
                                 /*Bold 05*/[0.9, 1.1, 1.0, 1.0, 1.0],
                               /*Docile 06*/[1.0, 1.0, 1.0, 1.0, 1.0],
                              /*Relaxed 07*/[1.0, 1.1, 0.9, 1.0, 1.0],
                               /*Impish 08*/[1.0, 1.1, 1.0, 0.9, 1.0],
                                  /*Lax 09*/[1.0, 1.1, 1.0, 1.0, 0.9],
                                /*Timid 10*/[0.9, 1.0, 1.1, 1.0, 1.0],
                                /*Hasty 11*/[1.0, 0.9, 1.1, 1.0, 1.0],
                              /*Serious 12*/[1.0, 1.0, 1.0, 1.0, 1.0],
                                /*Jolly 13*/[1.0, 1.0, 1.1, 0.9, 1.0],
                                /*Naive 14*/[1.0, 1.0, 1.1, 1.0, 0.9],
                               /*Modest 15*/[0.9, 1.0, 1.0, 1.1, 1.0],
                                 /*Mild 16*/[1.0, 0.9, 1.0, 1.1, 1.0],
                                /*Quiet 17*/[1.0, 1.0, 0.9, 1.1, 1.0],
                              /*Bashful 18*/[1.0, 1.0, 1.0, 1.0, 1.0],
                                 /*Rash 19*/[1.0, 1.0, 1.0, 1.1, 0.9],
                                 /*Calm 20*/[0.9, 1.0, 1.0, 1.0, 1.1],
                               /*Gentle 21*/[1.0, 0.9, 1.0, 1.0, 1.1],
                                /*Sassy 22*/[1.0, 1.0, 0.9, 1.0, 1.1],
                              /*Careful 23*/[1.0, 1.0, 1.0, 0.9, 1.1],
                               /*Quirky 24*/[1.0, 1.0, 1.0, 1.0, 1.0]];

// ID Offset used for keeping the Unique ID of a pokemon the same, even after it evolves
pub static ID_OFFSETS: [u8;256] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0x00*/0x00, 0x00, 0x01, 0x02, 0x00, 0x01, 0x02, 0x00, 0x01, 0x02, 0x00, 0x01, 0x02, 0x00, 0x01, 0x02, 
/*0x10*/0x00, 0x01, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x02, 
/*0x20*/0x00, 0x01, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x02, 0x00, 0x01, 
/*0x30*/0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x02, 0x00, 
/*0x40*/0x01, 0x02, 0x00, 0x01, 0x02, 0x00, 0x01, 0x02, 0x00, 0x01, 0x00, 0x01, 0x02, 0x00, 0x01, 0x00, 
/*0x50*/0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x02, 0x00, 
/*0x60*/0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 
/*0x70*/0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0x80*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 
/*0x90*/0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xA0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xB0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xC0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xD0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xE0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xF0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

// Base mon struct, before adding the specifics to the pokemon, for gen 3
pub static GEN_3_BASE_MON: [u8;100] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0x00*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
/*0x10*/0xFF, 0xFF, 0x02, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0x20*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0x30*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0x40*/0x00, 0x00, 0x00, 0x00, 0x00, 0xB7, 0x00, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0x50*/0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0x60*/0x00, 0x00, 0x00, 0x00];

// Red -> Fire red species map
pub static RED_FIRERED_SPECIES: [u8;256] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0x00*/0x00, 0x70, 0x73, 0x20, 0x23, 0x15, 0x64, 0x22, 0x50, 0x02, 0x67, 0x6C, 0x66, 0x58, 0x5E, 0x1D, 
/*0x10*/0x1F, 0x68, 0x6F, 0x83, 0x3B, 0x97, 0x82, 0x5A, 0x48, 0x5C, 0x7B, 0x78, 0x09, 0x7F, 0x72, 0x00, 
/*0x20*/0x00, 0x3A, 0x5F, 0x16, 0x10, 0x4F, 0x40, 0x4B, 0x71, 0x43, 0x7A, 0x6A, 0x6B, 0x18, 0x2F, 0x36, 
/*0x30*/0x60, 0x4C, 0x00, 0x7E, 0x00, 0x7D, 0x52, 0x6D, 0x00, 0x38, 0x56, 0x32, 0x80, 0x00, 0x00, 0x00, 
/*0x40*/0x53, 0x30, 0x95, 0x00, 0x00, 0x00, 0x54, 0x3C, 0x7C, 0x92, 0x90, 0x91, 0x84, 0x34, 0x62, 0x00, 
/*0x50*/0x00, 0x00, 0x25, 0x26, 0x19, 0x1A, 0x00, 0x00, 0x93, 0x94, 0x8C, 0x8D, 0x74, 0x75, 0x00, 0x00, 
/*0x60*/0x1B, 0x1C, 0x8A, 0x8B, 0x27, 0x28, 0x85, 0x88, 0x87, 0x86, 0x42, 0x29, 0x17, 0x2E, 0x3D, 0x3E, 
/*0x70*/0x0D, 0x0E, 0x0F, 0x00, 0x55, 0x39, 0x33, 0x31, 0x57, 0x00, 0x00, 0x0A, 0x0B, 0x0C, 0x44, 0x00, 
/*0x80*/0x37, 0x61, 0x2A, 0x96, 0x8F, 0x81, 0x00, 0x00, 0x59, 0x00, 0x63, 0x5B, 0x00, 0x65, 0x24, 0x6E, 
/*0x90*/0x35, 0x69, 0x00, 0x5D, 0x3F, 0x41, 0x11, 0x12, 0x79, 0x01, 0x03, 0x49, 0x00, 0x76, 0x77, 0x00, 
/*0xA0*/0x00, 0x00, 0x00, 0x4D, 0x4E, 0x13, 0x14, 0x21, 0x1E, 0x4A, 0x89, 0x8E, 0x00, 0x51, 0x00, 0x00, 
/*0xB0*/0x04, 0x07, 0x05, 0x08, 0x06, 0x00, 0x00, 0x00, 0x00, 0x2B, 0x2C, 0x2D, 0x45, 0x46, 0x47, 0x00, 
/*0xC0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xD0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xE0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xF0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

// Fire red -> Red species map
pub static FIRERED_RED_SPECIES: [u8;256] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0x00*/0x00, 0x99, 0x09, 0x9A, 0xB0, 0xB2, 0xB4, 0xB1, 0xB3, 0x1C, 0x7B, 0x7C, 0x7D, 0x70, 0x71, 0x72, 
/*0x10*/0x24, 0x96, 0x97, 0xA5, 0xA6, 0x05, 0x23, 0x6C, 0x2D, 0x54, 0x55, 0x60, 0x61, 0x0F, 0xA8, 0x10, 
/*0x20*/0x03, 0xA7, 0x07, 0x04, 0x8E, 0x52, 0x53, 0x64, 0x65, 0x6B, 0x82, 0xB9, 0xBA, 0xBB, 0x6D, 0x2E, 
/*0x30*/0x41, 0x77, 0x3B, 0x76, 0x4D, 0x90, 0x2F, 0x80, 0x39, 0x75, 0x21, 0x14, 0x47, 0x6E, 0x6F, 0x94, 
/*0x40*/0x26, 0x95, 0x6A, 0x29, 0x7E, 0xBC, 0xBD, 0xBE, 0x18, 0x9B, 0xA9, 0x27, 0x31, 0xA3, 0xA4, 0x25, 
/*0x50*/0x08, 0xAD, 0x36, 0x40, 0x46, 0x74, 0x3A, 0x78, 0x0D, 0x88, 0x17, 0x8B, 0x19, 0x93, 0x0E, 0x22, 
/*0x60*/0x30, 0x81, 0x4E, 0x8A, 0x06, 0x8D, 0x0C, 0x0A, 0x11, 0x91, 0x2B, 0x2C, 0x0B, 0x37, 0x8F, 0x12, 
/*0x70*/0x01, 0x28, 0x1E, 0x02, 0x5C, 0x5D, 0x9D, 0x9E, 0x1B, 0x98, 0x2A, 0x1A, 0x48, 0x35, 0x33, 0x1D, 
/*0x80*/0x3C, 0x85, 0x16, 0x13, 0x4C, 0x66, 0x69, 0x68, 0x67, 0xAA, 0x62, 0x63, 0x5A, 0x5B, 0xAB, 0x84, 
/*0x90*/0x4A, 0x4B, 0x49, 0x58, 0x59, 0x42, 0x83, 0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xA0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xB0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xC0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xD0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xE0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
/*0xF0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

// Red -> Fire Red character map
pub static RED_FIRERED_TEXT: [u8;256] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0x00*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0x10*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0x20*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0x30*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0x40*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0x50*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0x60*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF0, 0xFF, 0xFF,
/*0x70*/0xB3, 0xF4, 0xB1, 0xB2, 0xFF, 0xB0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
/*0x80*/0xBB, 0xBC, 0xBD, 0xBE, 0xBF, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA,
/*0x90*/0xCB, 0xCC, 0xCD, 0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0x5C, 0x5D, 0xF0, 0x36, 0x5C, 0x5D,
/*0xA0*/0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xDC, 0xDD, 0xDE, 0xDF, 0xE0, 0xE1, 0xE2, 0xE3, 0xE4,
/*0xB0*/0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0xC0*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0xD0*/0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
/*0xE0*/0xB3, 0x53, 0x54, 0xAE, 0xFF, 0xFF, 0xAC, 0xAB, 0xAD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xB5,
/*0xF0*/0xB7, 0xB9, 0xAD, 0xBA, 0xB8, 0xB6, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA];

// Fire Red -> Red character map
pub static FIRERED_RED_TEXT: [u8;256] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0x00*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x10*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x20*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x30*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x9D, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x40*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x50*/0x50, 0x50, 0x50, 0xE1, 0xE2, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x9A, 0x9B, 0x50, 0x50,
/*0x60*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x70*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x80*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0x90*/0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
/*0xA0*/0x50, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFE, 0xE7, 0xE6, 0xE8, 0xE3, 0x50,
/*0xB0*/0x75, 0x72, 0x73, 0x70, 0x71, 0xEF, 0xF5, 0xF0, 0xF4, 0xF1, 0xF3, 0x80, 0x81, 0x82, 0x83, 0x84,
/*0xC0*/0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F, 0x90, 0x91, 0x92, 0x93, 0x94,
/*0xD0*/0x95, 0x96, 0x97, 0x98, 0x99, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA,
/*0xE0*/0xAB, 0xAC, 0xAD, 0xAE, 0xAF, 0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0x50,
/*0xF0*/0x9C, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50];

// Fire Red -> Red item map "0x00" means ignore when transfering
pub static FIRERED_RED_ITEMS: [u8;384] = [
/*      0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F*/
/*0000*/0x00, 0x01, 0x02, 0x03, 0x04, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x0B, 0x0C,
/*0010*/0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x34, 0x35, 0x36, 0x3C, 0x3D, 0x3E, 0x13, 0x00, 0x00,
/*0020*/0x00, 0x00, 0x50, 0x51, 0x52, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0030*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23,
/*0040*/0x24, 0x25, 0x26, 0x27, 0x28, 0x4F, 0x00, 0x00, 0x00, 0x37, 0x3A, 0x41, 0x42, 0x43, 0x2E, 0x44,
/*0050*/0x33, 0x00, 0x00, 0x38, 0x39, 0x1D, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x20,
/*0060*/0x21, 0x22, 0x2F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x00,
/*0070*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0080*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0090*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*00A0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*00B0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*00C0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*00D0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*00E0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*00F0*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0100*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0110*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0120*/0x00, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7,
/*0130*/0xD8, 0xD9, 0xDA, 0xDB, 0xDC, 0xDD, 0xDE, 0xDF, 0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7,
/*0140*/0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7,
/*0150*/0xF8, 0xF9, 0xFA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0160*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
/*0170*/0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

// Red -> Fire Red item map "0x0000" means ignore when transfering
pub static RED_FIRERED_ITEMS: [u16;256] = [
/*       0x00    0x01    0x02    0x03    0x04    0x05    0x06    0x07    0x08    0x09    0x0A    0x0B    0x0C    0x0D    0x0E    0x0F*/
/*0x00*/0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0000, 0x0000, 0x0000, 0x0005, 0x0000, 0x005E, 0x000E, 0x000F, 0x0010, 0x0011, 0x0012,
/*0x10*/0x0013, 0x0014, 0x0015, 0x0016, 0x000D, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0055, 0x0056, 0x0000,
/*0x20*/0x005F, 0x0060, 0x0061, 0x003F, 0x0040, 0x0041, 0x0042, 0x0043, 0x0044, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x004E, 0x0062,
/*0x30*/0x0000, 0x006E, 0x0000, 0x0050, 0x0027, 0x0018, 0x0019, 0x0049, 0x0053, 0x0054, 0x004A, 0x0000, 0x001A, 0x001B, 0x001C, 0x0000,
/*0x40*/0x0000, 0x004B, 0x004C, 0x004D, 0x004F, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0045,
/*0x50*/0x0023, 0x0024, 0x0025, 0x0026, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0x60*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0x70*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0x80*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0x90*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0xA0*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0xB0*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
/*0xC0*/0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0121, 0x0122, 0x0123, 0x0124, 0x0125, 0x0126, 0x0127,
/*0xD0*/0x0128, 0x0129, 0x012A, 0x012B, 0x012C, 0x012D, 0x012E, 0x012F, 0x0130, 0x0131, 0x0132, 0x0133, 0x0134, 0x0135, 0x0136, 0x0137,
/*0xE0*/0x0138, 0x0139, 0x013A, 0x013B, 0x013C, 0x013D, 0x013E, 0x013F, 0x0140, 0x0141, 0x0142, 0x0143, 0x0144, 0x0145, 0x0146, 0x0147,
/*0xF0*/0x0148, 0x0149, 0x014A, 0x014B, 0x014C, 0x014D, 0x014E, 0x014F, 0x0150, 0x0151, 0x0152, 0x0153, 0x0154, 0x0155, 0x0156, 0x0157];

// Item Pockets

pub const P_GENR: u8 = 0x00;
pub const P_KEYI: u8 = 0x01;
pub const P_BALL: u8 = 0x02;
pub const P_TMHM: u8 = 0x03;
pub const P_PC: u8 = 0x04;
pub const P_BERY: u8 = 0x05;

pub const MAX_ITEM_COUNTS: [usize;5] = [0x2A, 0x1E, 0x0D, 0x3A, 0x1E];

pub static RED_ITEM_POCKETS: [u8;256] = [
/*       0x00    0x01    0x02    0x03    0x04    0x05    0x06    0x07    0x08    0x09    0x0A    0x0B    0x0C    0x0D    0x0E    0x0F*/
/*0x00*/P_GENR, P_BALL, P_BALL, P_BALL, P_BALL, P_KEYI, P_KEYI, P_GENR, P_BALL, P_KEYI, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0x10*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_GENR, P_GENR, P_KEYI,
/*0x20*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_KEYI, P_KEYI, P_KEYI, P_GENR, P_KEYI, P_GENR, P_GENR,
/*0x30*/P_KEYI, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_KEYI,
/*0x40*/P_KEYI, P_GENR, P_GENR, P_GENR, P_GENR, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_KEYI, P_GENR,
/*0x50*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0x60*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0x70*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0x80*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0x90*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0xA0*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0xB0*/P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR, P_GENR,
/*0xC0*/P_GENR, P_GENR, P_GENR, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM,
/*0xD0*/P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM,
/*0xE0*/P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM,
/*0xF0*/P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM, P_TMHM];

// Progress gates

pub static LEVEL_CAPS: [(u32, u8);11] = [
    (G_BOULDERBADGE, 23),
    (G_CASCADEBADGE, 29),
    (G_THUNDERBADGE, 32),
    (G_RAINBOWBADGE, 39),
    (G_SILPHSCOPE,   43),
    (G_POKEFLUTE,    47),
    (G_SOULBADGE,    51),
    (G_MARSHBADGE,   51),
    (G_VOLCANOBADGE, 55),
    (G_EARTHBADGE,   59),
    (0x80000000,     100)
 ];

pub const G_NONE:         u32 = 0x00000000;
pub const G_BOULDERBADGE: u32 = 0x00000001; // level_cap = 23 
pub const G_CASCADEBADGE: u32 = 0x00000002; // level_cap = 29
pub const G_THUNDERBADGE: u32 = 0x00000004; // level_cap = 32
pub const G_RAINBOWBADGE: u32 = 0x00000008; // level_cap = 39
pub const G_SOULBADGE:    u32 = 0x00000010; // level_cap = 51
pub const G_MARSHBADGE:   u32 = 0x00000020; // level_cap = 51
pub const G_VOLCANOBADGE: u32 = 0x00000040; // level_cap = 55
pub const G_EARTHBADGE:   u32 = 0x00000080; // level_cap = 59
pub const G_SILPHSCOPE:   u32 = 0x00000100; // level_cap = 43
pub const G_POKEFLUTE:    u32 = 0x00000200; // level_cap = 47
pub const G_SSTICKET:     u32 = 0x00000400; 
pub const G_SECRETKEY:    u32 = 0x00000800; 
pub const G_CUT:          u32 = 0x00001000; 
pub const G_SURF:         u32 = 0x00002000; 
pub const G_STRENGTH:     u32 = 0x00004000;
pub const G_GUARDRUNK:    u32 = 0x00008000; 
pub const G_DLVRD_PARCEL: u32 = 0x00010000; // level_cap = _

