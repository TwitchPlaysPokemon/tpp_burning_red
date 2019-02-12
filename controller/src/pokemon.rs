use byteorder::{ByteOrder, LittleEndian, BigEndian};
use crate::constants::*;
use crate::gamestate::*;
use rand;

const BLOCK_POS: [[usize;24];4] = [[0, 0, 0, 0, 0, 0, 1, 1, 2, 3, 2, 3, 1, 1, 2, 3, 2, 3, 1, 1, 2, 3, 2, 3],
                                   [1, 1, 2, 3, 2, 3, 0, 0, 0, 0, 0, 0, 2, 3, 1, 1, 3, 2, 2, 3, 1, 1, 3, 2],
                                   [2, 3, 1, 1, 3, 2, 2, 3, 1, 1, 3, 2, 0, 0, 0, 0, 0, 0, 3, 2, 3, 2, 1, 1],
                                   [3, 2, 3, 2, 1, 1, 3, 2, 3, 2, 1, 1, 3, 2, 3, 2, 1, 1, 0, 0, 0, 0, 0, 0]];

const INVERT_BLOCK_POS: [u8;24] = [0, 1, 2, 4, 3, 5, 6, 7, 12, 18, 13, 19, 8, 10, 14, 20, 16, 22, 9, 11, 15, 21, 17, 23];

#[derive(Serialize, Deserialize, Clone)]
pub struct Ivs {
    stats: [u16;6],
    raw: u32
}
/* Gen 3 IVs */
impl From<u32> for Ivs {
    fn from(raw: u32) -> Ivs {
        Ivs {
            stats: [(raw & 0x1F) as u16,
                    ((raw >> 5) & 0x1F) as u16, 
                    ((raw >> 10) & 0x1F) as u16, 
                    ((raw >> 15) & 0x1F) as u16, 
                    ((raw >> 15) & 0x1F) as u16, 
                    ((raw >> 25) & 0x1F) as u16],
            raw
        }
    }
}
/* Gen 1 IVs */
impl From<u16> for Ivs {
    fn from(raw: u16) -> Ivs {
        let stats = [(((raw & 0x1000) >> 9) | 
                    ((raw & 0x0100) >> 6) | 
                    ((raw & 0x0010) >> 3) | 
                    (raw & 0x0001)) as u16,
                    ((raw & 0xF000) >> 12) as u16,
                    ((raw & 0x0F00) >> 8) as u16,
                    ((raw & 0x00F0) >> 4) as u16,
                    (raw & 0x000F) as u16,
                    (raw & 0x000F) as u16];

        let raw: u32 = ((stats[0] << 1) as u32) |
                       (((stats[1] << 1) as u32) << 5) |
                       (((stats[2] << 1) as u32) << 10) |
                       (((stats[3] << 1) as u32) << 15) |
                       (((stats[4] << 1) as u32) << 20) |
                       (((stats[4] << 1) as u32) << 25);
        Ivs {
            stats,
            raw
        }
    }
}

impl Ivs {
    pub fn to_u16(&mut self, shiny: bool) -> u16 {
        if shiny {
            self.stats[1] |= 0x02;
            self.stats[2] = 0x0A;
            self.stats[3] = 0x0A;
            self.stats[4] = 0x0A;
            (self.stats[1] >> 1) << 12 | 0x2AAA
        } else {
            (self.stats[1] >> 1) << 12 | 
            (self.stats[2] >> 1) << 8  |
            (self.stats[3] >> 1) << 4  |
            (self.stats[4] >> 1)    
        }
    }
} 

#[derive(Serialize, Deserialize, Clone)]
pub struct Pokemon {
    uid: u16,
    pv: u32,
    pub species: u8,
    moves: [u8;4],
    pp: [u8;4],
    exp: u32,
    ivs: Ivs,
    status_condition: u8,
    level: u8,
    pokerus_status: u8,
    pokerus_remaining: u8,
    trainer: TrainerInfo,
    item: u16,
    shiny: bool,
    gender: bool,
    hp_ratio: f32,
    pub gen1: Option<PKX>,
    pub gen3: Option<PKX>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PKX {
    pub name: Vec<u8>,
    evs: [u16;6], 
    current_hp: u16,
    stats: [u16;6],
    pub bytes: Vec<u8>
}

impl Pokemon {
    pub fn new(trainer: &TrainerInfo) -> Pokemon {
        Pokemon {
            uid: 0,
            pv: 0,
            species: 0,
            moves: [0;4],
            pp: [0;4],
            exp: 0,
            ivs: Ivs::from(0u32),
            status_condition: 0,
            level: 0,
            pokerus_status: 0x00,
            pokerus_remaining: 0xFF,
            trainer: trainer.clone(),
            item: 0,
            shiny: false,
            gender: false,
            hp_ratio: 0f32,
            gen1: None,
            gen3: None
        }
    }

    pub fn from_pk1(data: &[u8], nickname: &[u8], trainer: &TrainerInfo, uid: u16) -> Pokemon {

        let mut data = data.to_vec();

        let ivs = BigEndian::read_u16(&data[0x1B..0x1D]);

        data[0x07] = (uid & 0x00FF) as u8;

        let mut pokemon = Pokemon {
            uid,
            pv: 0,
            species: RED_FIRERED_SPECIES[data[0x00] as usize],
            moves: [data[0x08], data[0x09], data[0x0A], data[0x0B]],
            pp: [data[0x1D] & 0x3F, data[0x1E] & 0x3F, data[0x1F] & 0x3F, data[0x20] & 0x3F],
            exp: BigEndian::read_u32(&data[0x0D..0x11]) & 0x00FF_FFFF,
            ivs: Ivs::from(ivs),
            status_condition: data[0x04] >> 1,
            level: if data[0x21] == 0xAA { data[0x03] } else { data[0x21] },
            pokerus_status: 0x00,
            pokerus_remaining: 0xFF,
            trainer: trainer.clone(),
            item: 0,
            shiny: (ivs & 0x0FFF == 0x0AAA) && (ivs & 0x2000 > 0x0000),
            gender: false,
            hp_ratio: (BigEndian::read_u16(&data[0x01..0x03]) as f32) / (BigEndian::read_u16(&data[0x22..0x24]) as f32),
            gen1: Some(PKX {
                name: nickname.to_vec(),
                evs: [BigEndian::read_u16(&data[0x11..0x13]),
                      BigEndian::read_u16(&data[0x13..0x15]),
                      BigEndian::read_u16(&data[0x15..0x17]),
                      BigEndian::read_u16(&data[0x17..0x19]),
                      BigEndian::read_u16(&data[0x19..0x1B]),
                      0],
                current_hp: BigEndian::read_u16(&data[0x01..0x03]),
                stats: [BigEndian::read_u16(&data[0x22..0x24]),
                        BigEndian::read_u16(&data[0x24..0x26]),
                        BigEndian::read_u16(&data[0x26..0x28]),
                        BigEndian::read_u16(&data[0x28..0x2A]),
                        BigEndian::read_u16(&data[0x2A..0x2C]),
                        0],
                bytes: data[0..44].to_vec()
            }),
            gen3: None
        };

        pokemon.gender = (pokemon.ivs.stats[1] * 16) as u8 <= BASE_STATS[pokemon.species as usize].gen1.gender_ratio;

        pokemon.gen_pk3_data().unwrap();

        pokemon
    }

    pub fn gen_pk1_data(&mut self) -> Result<(), String> {

        /* Make Base Mon data struct, 44 bytes long */

        let mut data = vec![0;44];

        /* Get Gen 3 data */
        if let Some(ref gen3) = self.gen3 {
            data[0x00] = FIRERED_RED_SPECIES[self.species as usize];

            data[0x03] = self.level;
            data[0x04] = self.status_condition << 1; // Status Condition
            data[0x05] = BASE_STATS[self.species as usize].gen1._type[0]; // Type 1
            data[0x06] = BASE_STATS[self.species as usize].gen1._type[1]; // Type 2

            data[0x07] = (self.uid & 0x00FF) as u8;

            data[0x08] = self.moves[0] as u8; // Moves
            data[0x09] = self.moves[1] as u8;
            data[0x0A] = self.moves[2] as u8;
            data[0x0B] = self.moves[3] as u8;

            BigEndian::write_u32(&mut data[0x0D..0x11], self.exp); // Exp
            /* EXP will write 4 bytes here, so we need to write the previous loaction 
            after it to overwrite 0x0D with the correct value */
            BigEndian::write_u16(&mut data[0x0C..0x0E], self.trainer.tid); // TID
            /*
            let shiny = trainer.tid ^
                        trainer.sid ^
                        ((source.personality_value >> 16) as u16) ^
                        ((source.personality_value & 0xFFFF) as u16) <
                        8;
            */
            BigEndian::write_u16(&mut data[0x1B..0x1D], self.ivs.to_u16(self.shiny));

            data[0x1D] = (gen3.bytes[0x28] << 6) | self.pp[0];
            data[0x1E] = ((gen3.bytes[0x28] & 0x0C) << 4) | self.pp[1];
            data[0x1F] = ((gen3.bytes[0x28] & 0x30) << 2) | self.pp[2];
            data[0x20] = (gen3.bytes[0x28] & 0xC0) | self.pp[3];

            data[0x21] = self.level;

            /* Stats */ 
            let stats = recalc_stats_gen1(self.species, self.ivs.stats, [0;6], self.level);

            self.hp_ratio = gen3.current_hp as f32 / gen3.stats[0] as f32;
            let current_hp = (stats[0] as f32 * self.hp_ratio).floor() as u16;

            BigEndian::write_u16(&mut data[0x01..0x03], current_hp);
            BigEndian::write_u16(&mut data[0x22..0x24], stats[0]);
            BigEndian::write_u16(&mut data[0x24..0x26], stats[1]);
            BigEndian::write_u16(&mut data[0x26..0x28], stats[2]);
            BigEndian::write_u16(&mut data[0x28..0x2A], stats[3]);
            BigEndian::write_u16(&mut data[0x2A..0x2C], stats[4]);

            let mut name = gen3.name.clone();

            for character in &mut name {
                *character = FIRERED_RED_TEXT[*character as usize];
            }

            name.push(0x50);

            self.gen1 = Some(PKX {
                name: name,
                evs: [0;6],
                current_hp: BigEndian::read_u16(&data[0x01..0x03]),
                stats,
                bytes: data[0..44].to_vec()
            });

            Ok(())
        } else {
            Err("Gen 3 data not populated, cannot create PK1".to_string())
        }
    }

    pub fn update_from_pk1(&mut self, data: &[u8], nickname: &[u8]) -> Result<(), String> {

        let mut new_name = Vec::new();
        let mut recalc_stats = false;
        let mut species_change = false;

        // Update gen 1 data struct
        if let Some(gen1) = &mut self.gen1 {
            gen1.evs = [BigEndian::read_u16(&data[0x11..0x13]),
                        BigEndian::read_u16(&data[0x13..0x15]),
                        BigEndian::read_u16(&data[0x15..0x17]),
                        BigEndian::read_u16(&data[0x17..0x19]),
                        BigEndian::read_u16(&data[0x19..0x1B]),
                        0];
            gen1.current_hp = BigEndian::read_u16(&data[0x01..0x03]);
            gen1.stats = [BigEndian::read_u16(&data[0x22..0x24]),
                          BigEndian::read_u16(&data[0x24..0x26]),
                          BigEndian::read_u16(&data[0x26..0x28]),
                          BigEndian::read_u16(&data[0x28..0x2A]),
                          BigEndian::read_u16(&data[0x2A..0x2C]),
                          0];
            gen1.bytes = data[0..44].to_vec();
            gen1.bytes[0x07] = (self.uid & 0x00ff) as u8;
            if gen1.name != nickname {
                gen1.name = nickname.to_vec();
                new_name = gen1.name.clone();

                new_name.pop();

                for character in &mut new_name {
                    *character = RED_FIRERED_TEXT[*character as usize];
                }
            }
            self.hp_ratio = gen1.current_hp as f32 / gen1.stats[0] as f32;
        } else {
            return Err("Gen 1 data not populated, cannot update stats".to_string())
        }

        if self.species != RED_FIRERED_SPECIES[data[0x00] as usize] {
            self.species = RED_FIRERED_SPECIES[data[0x00] as usize];
            species_change = true;
        }

        self.moves = [data[0x08], data[0x09], data[0x0A], data[0x0B]];
        self.pp = [data[0x1D] & 0x3F, data[0x1E] & 0x3F, data[0x1F] & 0x3F, data[0x20] & 0x3F];
        self.exp = BigEndian::read_u32(&data[0x0D..0x11]) & 0x00FF_FFFF;
        self.status_condition = data[0x04] >> 1;

        if self.level != data[0x21] {
            self.level = data[0x21];
            recalc_stats = true;
        }

        if let Some(gen3) = &mut self.gen3 {
            if !new_name.is_empty() {
                gen3.bytes[0x08..0x12].copy_from_slice(&new_name); // Nickname
            }

            gen3.bytes[0x14..0x1B].copy_from_slice(&self.trainer.gen_3_name[0x00..0x07]); // OT Nickname

            /* Fill Block A */
            if species_change {
                LittleEndian::write_u16(&mut gen3.bytes[0x20..0x22], self.species as u16); // Species
            }

            LittleEndian::write_u32(&mut gen3.bytes[0x24..0x28], self.exp); // Exp

            gen3.bytes[0x28] = ((data[0x1D] >> 6) & 0x03) | 
                               ((data[0x1E] >> 4) & 0x0C) | 
                               ((data[0x1F] >> 2) & 0x30) | 
                               ( data[0x20]       & 0xC0); // PP Up

            LittleEndian::write_u16(&mut gen3.bytes[0x2C..0x2E], self.moves[0] as u16); // Moves
            LittleEndian::write_u16(&mut gen3.bytes[0x2E..0x30], self.moves[1] as u16);
            LittleEndian::write_u16(&mut gen3.bytes[0x30..0x32], self.moves[2] as u16);
            LittleEndian::write_u16(&mut gen3.bytes[0x32..0x34], self.moves[3] as u16);

            gen3.bytes[0x34] = self.pp[0] & 0x3F; // PP
            gen3.bytes[0x35] = self.pp[1] & 0x3F;
            gen3.bytes[0x36] = self.pp[2] & 0x3F;
            gen3.bytes[0x37] = self.pp[3] & 0x3F;

            LittleEndian::write_u32(&mut gen3.bytes[0x50..0x54], self.status_condition as u32); // Status Condition

            if recalc_stats {
                let stats = recalc_stats_gen3(self.species, self.ivs.stats, [0;6], (self.pv % 25) as u8, self.level);

                LittleEndian::write_u16(&mut gen3.bytes[0x58..0x5A], stats[0]);
                LittleEndian::write_u16(&mut gen3.bytes[0x5A..0x5C], stats[1]);
                LittleEndian::write_u16(&mut gen3.bytes[0x5C..0x5E], stats[2]);
                LittleEndian::write_u16(&mut gen3.bytes[0x5E..0x60], stats[3]);
                LittleEndian::write_u16(&mut gen3.bytes[0x60..0x62], stats[4]);
                LittleEndian::write_u16(&mut gen3.bytes[0x62..0x64], stats[5]);

                gen3.bytes[0x54] = self.level; // Level
            }

            let current_hp = (gen3.stats[0] as f32 * self.hp_ratio).floor() as u16;
            LittleEndian::write_u16(&mut gen3.bytes[0x56..0x58], current_hp);

            let checksum = get_checksum_u16(&gen3.bytes[0x20..0x50]);

            LittleEndian::write_u16(&mut gen3.bytes[0x1C..0x1E], checksum);
        } else {
            return Err("Gen 3 data not populated, cannot update stats".to_string());
        }
        Ok(())
    }

    pub fn from_pk3(data: &[u8], trainer: &TrainerInfo, uid: u16) -> Pokemon {
        let mut data = data.to_vec();

        decrypt_unshuffle_pk3(&mut data);

        data[0x3E] = (uid & 0x00FF) as u8;

        let mut pokemon = Pokemon {
            uid,
            pv: LittleEndian::read_u32(&data[0x00..0x04]),
            species: data[0x20],
            moves: [data[0x2C], data[0x2E], data[0x30], data[0x32]],
            pp: [data[0x34], data[0x35], data[0x36], data[0x37]],
            exp: LittleEndian::read_u32(&data[0x24..0x28]),
            ivs: Ivs::from(LittleEndian::read_u32(&data[0x48..0x4C])),
            status_condition: data[0x50],
            level: data[0x54],
            pokerus_status: data[0x44],
            pokerus_remaining: data[0x55],
            trainer: trainer.clone(),
            item: LittleEndian::read_u16(&data[0x22..0x24]),
            shiny: false,
            gender: false,
            hp_ratio: LittleEndian::read_u16(&data[0x56..0x58]) as f32 / LittleEndian::read_u16(&data[0x58..0x5A]) as f32,
            gen1: None,
            gen3: Some(PKX {
               name: data[0x08..0x12].to_vec(),
               evs: [data[0x38] as u16, data[0x39] as u16, data[0x3A] as u16, data[0x3B] as u16, data[0x3C] as u16, data[0x3D] as u16],
               current_hp: LittleEndian::read_u16(&data[0x56..0x58]),
               stats: [LittleEndian::read_u16(&data[0x58..0x5A]),
                       LittleEndian::read_u16(&data[0x5A..0x5C]),
                       LittleEndian::read_u16(&data[0x5C..0x5E]),
                       LittleEndian::read_u16(&data[0x5E..0x60]),
                       LittleEndian::read_u16(&data[0x60..0x62]),
                       LittleEndian::read_u16(&data[0x62..0x64])],
               bytes: data[0..100].to_vec()
            })
        };

        pokemon.shiny = trainer.tid ^
                        trainer.sid ^
                        ((pokemon.pv >> 16) as u16) ^
                        ((pokemon.pv & 0xFFFF) as u16) <
                        8;
        pokemon.gender = ((pokemon.pv & 0xFF) as u8) < BASE_STATS[pokemon.species as usize].gen3.gender_ratio;

        pokemon.gen_pk1_data().unwrap();

        pokemon
    }

    pub fn gen_pk3_data(&mut self) -> Result<(), String> {

        /* Copy Base Gen 3 Mon data struct, 100 bytes long */

        let mut data = GEN_3_BASE_MON.to_vec();

        /* Determine PID attributes & calc PID */

        if let Some(ref gen1) = self.gen1 {

            self.pv = gen_random_pid(self.species as u16, self.gender, self.shiny, 0xff, &self.trainer);

            /* Begin filling data structure */

            LittleEndian::write_u32(&mut data[0x00..0x04], self.pv); // Personality Value
            LittleEndian::write_u16(&mut data[0x04..0x06], self.trainer.tid); // TID
            LittleEndian::write_u16(&mut data[0x06..0x08], self.trainer.sid); // SID

            let mut name = gen1.name.clone();

            name.pop();

            for character in &mut name {
                *character = RED_FIRERED_TEXT[*character as usize];
            }

            data[0x08..0x12].copy_from_slice(&name); // Nickname

            data[0x14..0x1B].copy_from_slice(&self.trainer.gen_3_name[0x00..0x07]); // OT Nickname

            /* Fill Block A */

            LittleEndian::write_u16(&mut data[0x20..0x22], self.species as u16); // Species

            LittleEndian::write_u32(&mut data[0x24..0x28], self.exp); // Exp

            data[0x28] = ((gen1.bytes[0x1D] >> 6) & 0x03) | 
                         ((gen1.bytes[0x1E] >> 4) & 0x0C) | 
                         ((gen1.bytes[0x1F] >> 2) & 0x30) | 
                         ( gen1.bytes[0x20]       & 0xC0); // PP Up

            /* Fill Block B */

            LittleEndian::write_u16(&mut data[0x2C..0x2E], self.moves[0] as u16); // Moves
            LittleEndian::write_u16(&mut data[0x2E..0x30], self.moves[1] as u16);
            LittleEndian::write_u16(&mut data[0x30..0x32], self.moves[2] as u16);
            LittleEndian::write_u16(&mut data[0x32..0x34], self.moves[3] as u16);

            data[0x34] = gen1.bytes[0x1D] & 0x3F; // PP
            data[0x35] = gen1.bytes[0x1E] & 0x3F;
            data[0x36] = gen1.bytes[0x1F] & 0x3F;
            data[0x37] = gen1.bytes[0x20] & 0x3F;

            data[0x3E] = (self.uid & 0x00FF) as u8;

            /* Ignore Block C */

            /* Block D */

            LittleEndian::write_u32(&mut data[0x48..0x4C], self.ivs.raw);

            LittleEndian::write_u32(&mut data[0x50..0x54], self.status_condition as u32); // Status Condition

            data[0x54] = self.level; // Level

            /* Stats */ 
            let stats = recalc_stats_gen3(self.species, self.ivs.stats, [0;6], (self.pv % 25) as u8, self.level);

            let hp_ratio = gen1.current_hp as f32 / gen1.stats[0] as f32;
            let current_hp = (stats[0] as f32 * hp_ratio).floor() as u16;

            LittleEndian::write_u16(&mut data[0x56..0x58], current_hp);
            LittleEndian::write_u16(&mut data[0x58..0x5A], stats[0]);
            LittleEndian::write_u16(&mut data[0x5A..0x5C], stats[1]);
            LittleEndian::write_u16(&mut data[0x5C..0x5E], stats[2]);
            LittleEndian::write_u16(&mut data[0x5E..0x60], stats[3]);
            LittleEndian::write_u16(&mut data[0x60..0x62], stats[4]);
            LittleEndian::write_u16(&mut data[0x62..0x64], stats[5]);

            let checksum = get_checksum_u16(&data[0x20..0x50]);
            LittleEndian::write_u16(&mut data[0x1C..0x1E], checksum);
            //let checksum = get_checksum_u16(&data[0x20..0x50]);

            //LittleEndian::write_u16(&mut data[0x1C..0x1E], checksum);

            self.gen3 = Some(PKX {
                name,
                evs: [0;6],
                current_hp,
                stats,
                bytes: data[0..100].to_vec()
            });

            Ok(())
        } else {
            Err("Gen 1 data not populated, cannot create PK3".to_string())
        }
    }

    pub fn update_from_pk3(&mut self, data: &[u8]) -> Result<(), String> {

        let mut data = data.to_vec();

        decrypt_unshuffle_pk3(&mut data);

        let mut new_name = Vec::new();
        let mut recalc_stats = false;
        let mut species_change = false;
        // Update gen 3 data struct
        if let Some(gen3) = &mut self.gen3 {
            gen3.evs = [data[0x38] as u16, 
                        data[0x39] as u16, 
                        data[0x3A] as u16, 
                        data[0x3B] as u16, 
                        data[0x3C] as u16, 
                        data[0x3D] as u16];
            gen3.current_hp = LittleEndian::read_u16(&data[0x56..0x58]);
            gen3.stats = [LittleEndian::read_u16(&data[0x58..0x5A]),
                          LittleEndian::read_u16(&data[0x5A..0x5C]),
                          LittleEndian::read_u16(&data[0x5C..0x5E]),
                          LittleEndian::read_u16(&data[0x5E..0x60]),
                          LittleEndian::read_u16(&data[0x60..0x62]),
                          LittleEndian::read_u16(&data[0x62..0x64])];
            gen3.bytes = data[0..100].to_vec();
            gen3.bytes[0x3E] = (self.uid & 0x00ff) as u8;
            if gen3.name != &data[0x08..0x12] {
                gen3.name = data[0x08..0x12].to_vec();
                new_name = gen3.name.clone();

                for character in &mut new_name {
                    *character = FIRERED_RED_TEXT[*character as usize];
                }

                new_name.push(0x50);
            }

            let checksum = get_checksum_u16(&gen3.bytes[0x20..0x50]);
            LittleEndian::write_u16(&mut gen3.bytes[0x1C..0x1E], checksum);

            self.hp_ratio = gen3.current_hp as f32 / gen3.stats[0] as f32;
        } else {
            return Err("Gen 3 data not populated, cannot update stats".to_string())
        }

        if self.species != data[0x20] {
            self.species = data[0x20];
            species_change = true;
        }

        self.moves = [data[0x2C], data[0x2E], data[0x30], data[0x32]];
        self.pp = [data[0x34], data[0x35], data[0x36], data[0x37]];
        self.exp = LittleEndian::read_u32(&data[0x24..0x28]);
        self.status_condition = data[0x50];

        if self.level != data[0x54] {
            self.level = data[0x54];
            recalc_stats = true;
        }

        self.pokerus_status = data[0x44];
        self.pokerus_remaining = data[0x55];
        self.item = LittleEndian::read_u16(&data[0x22..0x24]);

        if let Some(gen1) = &mut self.gen1 {
            if species_change {
                gen1.bytes[0x00] = FIRERED_RED_SPECIES[self.species as usize]; // species
                gen1.bytes[0x05] = BASE_STATS[self.species as usize].gen1._type[0]; // Type 1
                gen1.bytes[0x06] = BASE_STATS[self.species as usize].gen1._type[1]; // Type 2
            }

            gen1.bytes[0x04] = self.status_condition << 1; // Status Condition

            gen1.bytes[0x08] = self.moves[0] as u8; // Moves
            gen1.bytes[0x09] = self.moves[1] as u8;
            gen1.bytes[0x0A] = self.moves[2] as u8;
            gen1.bytes[0x0B] = self.moves[3] as u8;

            gen1.bytes[0x1D] = (data[0x28] << 6) | self.pp[0]; // Move PP
            gen1.bytes[0x1E] = ((data[0x28] & 0x0C) << 4) | self.pp[1];
            gen1.bytes[0x1F] = ((data[0x28] & 0x30) << 2) | self.pp[2];
            gen1.bytes[0x20] = (data[0x28] & 0xC0) | self.pp[3];

            if !new_name.is_empty() {
                gen1.name = new_name;
            }

            BigEndian::write_u32(&mut gen1.bytes[0x0D..0x11], self.exp); // Exp
            /* EXP will write 4 bytes here, so we need to write the previous loaction 
            after it to overwrite 0x0D with the correct value */
            BigEndian::write_u16(&mut gen1.bytes[0x0C..0x0E], self.trainer.tid); // TID

            if recalc_stats {
                gen1.bytes[0x03] = self.level;
                gen1.bytes[0x21] = self.level;

                gen1.stats = recalc_stats_gen1(self.species, self.ivs.stats, gen1.evs, self.level);

                BigEndian::write_u16(&mut gen1.bytes[0x22..0x24], gen1.stats[0]);
                BigEndian::write_u16(&mut gen1.bytes[0x24..0x26], gen1.stats[1]);
                BigEndian::write_u16(&mut gen1.bytes[0x26..0x28], gen1.stats[2]);
                BigEndian::write_u16(&mut gen1.bytes[0x28..0x2A], gen1.stats[3]);
                BigEndian::write_u16(&mut gen1.bytes[0x2A..0x2C], gen1.stats[4]);
            }

            gen1.current_hp = (gen1.stats[0] as f32 * self.hp_ratio).floor() as u16;

            BigEndian::write_u16(&mut gen1.bytes[0x01..0x03], gen1.current_hp);
        } else {
            return Err("Gen 1 data not populated, cannot update stats".to_string());
        }
        Ok(())
    }

    pub fn get_encrypted_pk3(&self) -> Result<Vec<u8>, String> {
        if let Some(ref gen3) = self.gen3 {
            let mut bytes = gen3.bytes.to_vec();
            encrypt_shuffle_pk3(&mut bytes);
            Ok(bytes)
        } else {
            Err("Gen 3 data not populated, cannot export PK3".to_string())
        }
    }

}

pub fn get_uid(species: u8, sub_value: u8) -> u16 {
    if sub_value != BASE_STATS[species as usize].gen1.catch_rate && sub_value != 0 {
        (((species - ID_OFFSETS[species as usize]) as u16) << 8) | (sub_value as u16)
    } else {
        0x0000
    }
}

pub fn shuffle_blocks(data: &mut [u8], sv: u8) {
    let temp_buffer = data.to_vec();
    let block_size = data.len() / 4;

    for block in 0..4 {
        for byte in 0..block_size {
            data[block_size*BLOCK_POS[block][sv as usize]+byte] = temp_buffer[(block_size*block)+byte];
        }
    }
}

pub fn unshuffle_blocks(data: &mut [u8], sv: u8) {
    shuffle_blocks(data, INVERT_BLOCK_POS[sv as usize]);
}

pub fn decrypt_unshuffle_pk3(data: &mut [u8]) {
    let pv = LittleEndian::read_u32(&data[0x00..0x04]);
    let ot_id = LittleEndian::read_u32(&data[0x04..0x08]);
    let key = pv ^ ot_id;

    //Decrypt
    for i in 0..12 {
        let current_dword = LittleEndian::read_u32(&data[0x20 + i*4..0x20 + i*4+4]);
        LittleEndian::write_u32(&mut data[0x20 + i*4..0x20 + i*4+4], current_dword ^ key);
    }

    unshuffle_blocks(&mut data[0x20..0x50], (pv % 24) as u8);
}

pub fn encrypt_shuffle_pk3(data: &mut [u8]) {
    let pv = LittleEndian::read_u32(&data[0x00..0x04]);
    let ot_id = LittleEndian::read_u32(&data[0x04..0x08]);
    let key = pv ^ ot_id;

    //encrypt
    for i in 0..12 {
        let current_dword = LittleEndian::read_u32(&data[0x20 + i*4..0x20 + i*4+4]);
        LittleEndian::write_u32(&mut data[0x20 + i*4..0x20 + i*4+4], current_dword ^ key);
    }

    shuffle_blocks(&mut data[0x20..0x50], (pv % 24) as u8);
}

pub fn get_uid_gen3(data: &[u8]) -> u16 {
    let pv = LittleEndian::read_u32(&data[0x00..0x04]);
    let ot_id = LittleEndian::read_u32(&data[0x04..0x08]);
    let key = pv ^ ot_id;

    let block_pos = BLOCK_POS[0][INVERT_BLOCK_POS[(pv % 24) as usize] as usize]*0x0C;
    let sub_id_block_pos = BLOCK_POS[2][INVERT_BLOCK_POS[(pv % 24) as usize] as usize]*0x0C + 4;

    let species = (LittleEndian::read_u32(&data[0x20 + block_pos..0x20 + block_pos+4]) ^ key) as u8;
    let sub_id = ((LittleEndian::read_u32(&data[0x20 + sub_id_block_pos..0x20 + sub_id_block_pos+4]) ^ key) >> 16) & 0xFF;
    get_uid(species, sub_id as u8)
}

pub fn get_checksum_u16(data: &[u8]) -> u16 {
    let mut checksum: u16 = 0;
    for i in 0..data.len() / 2 {
        checksum = checksum.wrapping_add(LittleEndian::read_u16(&data[i*2..i*2+2]));
    }
    checksum
}

pub fn gen_random_pid(species: u16, gender: bool, shiny: bool, nature: u8, trainer: &TrainerInfo) -> u32 {
    let mut pid;
    let gender_ratio = BASE_STATS[species as usize].gen3.gender_ratio;
    loop {
        pid = rand::random::<u32>();

        // Match gender
        match gender_ratio {
            0x0 => {},
            0xFE => {},
            0xFF => {},
            _ => {
                if (((pid & 0xFF) as u8) < gender_ratio) != gender {
                    continue;
                }
            }
        }

        // Match nature
        if nature != 0xFF && ((pid % 25) as u8) != nature {
            continue;
        }

        // Match shiny status
        if (trainer.tid ^
           trainer.sid ^
           ((pid >> 16) as u16) ^
           ((pid & 0xFFFF) as u16) <
           8) != shiny {
            continue;
        }

        break;
    }
    pid
}

pub fn recalc_stats_gen3(species: u8, ivs: [u16;6], evs: [u8;6], nature: u8, level: u8) -> [u16;6] {
    let level = level as u16;
    let mut stats = [0u16;6];

    /* HP */
    let main_part = (((2 * BASE_STATS[species as usize].gen3.stats[0] + (ivs[0] * 2) + (evs[0] as f32 / 4f32).floor() as u16) * level) as f32 / 100f32).floor() as u16;
    stats[0] = main_part + level + 10;
    /* All the rest */
    for i in 1..6 {
        let main_part = (((2 * BASE_STATS[species as usize].gen3.stats[i] + (ivs[i] * 2) + (evs[i] as f32 / 4f32).floor() as u16) * level) as f32 / 100f32).floor();
        stats[i] =  ((main_part + 5f32) * NATURE_EFFECTS[nature as usize][i-1]).floor() as u16;
    }
    stats
}

pub fn recalc_stats_gen1(species: u8, ivs: [u16;6], evs: [u16;6], level: u8) -> [u16;6] {

    let level = level as u16;

    let mut stats = [0u16;6];

    /* HP */
    let ev_portion = ((evs[0] as f32).sqrt() / 4f32).floor() as u16;
    let main_part = ((((BASE_STATS[species as usize].gen3.stats[0] + ivs[0]) * 2 + ev_portion) * level) as f32 / 100f32).floor() as u16;
    stats[0] = main_part + level + 10;
    /* All the rest */
    for i in 1..5 {
        let ev_portion = ((evs[i] as f32).sqrt() / 4f32).floor() as u16;
        stats[i] = ((((BASE_STATS[species as usize].gen3.stats[i] + ivs[i]) * 2 + ev_portion) * level) as f32 / 100f32).floor() as u16 + 5;
    }
    stats
}