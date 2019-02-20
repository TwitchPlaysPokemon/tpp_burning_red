use crate::constants::*;
use crate::bizhawk::*;
use crate::gamestate::*;
use rocket::{get, routes};
use rocket::config::{Config, Environment, LoggingLevel};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ApiState {
    pub inventory: [Pocket;5],
    pub locked: bool
}

impl ApiState {
    pub fn new() -> ApiState {
        ApiState {
            inventory: [Pocket::new(&[0x86, 0xA4, 0xAD, 0xA4, 0xB1, 0xA0, 0xAB, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50], P_GENR), // "General"
                        Pocket::new(&[0x8A, 0xA4, 0xB8, 0x7F, 0x88, 0xB3, 0xA4, 0xAC, 0xB2, 0x50, 0x50, 0x50, 0x50], P_KEYI), // "Key Items"
                        Pocket::new(&[0x8F, 0xAE, 0xAA, 0xBA, 0x7F, 0x81, 0xA0, 0xAB, 0xAB, 0xB2, 0x50, 0x50, 0x50], P_BALL), // "Poké Balls"
                        Pocket::new(&[0x93, 0x8C, 0xB2, 0x7F, 0xF3, 0x7F, 0x87, 0x8C, 0xB2, 0x50, 0x50, 0x50, 0x50], P_TMHM), // "TMs / HMs"
                        Pocket::new(&[0x8F, 0x82, 0x7F, 0x88, 0xB3, 0xA4, 0xAC, 0xB2, 0x50, 0x50, 0x50, 0x50, 0x50], P_PC)], // "PC Items"
            locked: false
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ApiResponse {
  NONE,
  CODE,
  APIBUFFER,
  PAGE
}

pub fn start_item_api() {
    let config = Config::build(Environment::Staging)
        .address("localhost")
        .port(5340)
        .log_level(LoggingLevel::Off)
        .finalize().unwrap();
    rocket::custom(config).mount("/", routes![item_api_handler]).launch();
}

#[get("/item_api")]
fn item_api_handler() -> &'static str {
    if CALLBACK.try_lock().is_err() { 
        // Another thread is currently trying to use the bizhawk gui thread.
        // Doing anything other than returning, even a memory write, will lockup the emulator.
        // So we are forced to return without writing anything.
        // Callback was changed from onWrite to onRead to make this a non-issue.
        return "ok"
    }
    if let Ok(mut api_state) = RED_ITEM_STATE.try_lock() {
        let mut item_memory = BIZHAWK.read_slice_chained_sym(&[(&SYM["wItemAPICommand"], 0x11), (&SYM["wNumItems"], 0x7A)]).unwrap();

        let code = item_memory[0x00];
        item_memory[0x00] = ITEM_FALSE; // Default response

        let mut response = ApiResponse::NONE;

        //println!("ItemAPI req: code:{:x}, params{:?}", code, &item_memory[0x01..0x11]);

        if code > 0x03 {
            if code < 0x40 {
                if api_state.locked {
                    if code == ITEM_UNLOCK {
                        // if the api buffer == "InitItemAPI@"
                        if item_memory[0x01..0x0D] == [0x88, 0xAD, 0xA8, 0xB3, 0x88, 0xB3, 0xA4, 0xAC, 0x80, 0x8F, 0x88, 0x50] {
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        }
                    }
                } else {
                    match code {
                        ITEM_LOCK => {
                            api_state.locked = true;
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        },
                        ITEM_UNLOCK => {
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        },
                        ITEM_INITIALIZE_ITEM_LISTS => { // Dont touch save files in this implementation
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        },
                        ITEM_ERASE_SAVED_DATA => { // Do nothing
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        },
                        ITEM_SAVE => { // Do nothing
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        },
                        ITEM_LOAD => { // Do nothing
                            item_memory[0x00] = ITEM_TRUE;
                            response = ApiResponse::CODE;
                        },
                        ITEM_CAN_GET_ITEM => {
                            let (item, count, page) = (item_memory[0x01] as u16, item_memory[0x02] as u16, item_memory[0x03] as usize);
                            let expected_page = RED_ITEM_POCKETS[item as usize] as usize;
                            let content = &api_state.inventory[expected_page].content;
                            
                            response = ApiResponse::CODE;

                            if page == expected_page || page == 0xFF { // We have the correct pocket
                                if let Some(slot) = content.iter().find(|&&x| {x[0] == item}) {
                                    if slot[1] < (1000 - count) { // We have the item & the total - req count is under 999
                                        item_memory[0x00] = ITEM_TRUE; 
                                    }
                                } else {
                                    if content.len() < MAX_ITEM_COUNTS[expected_page] { // We dont have the item, but have empty slots
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                }
                            }
                        },
                        ITEM_ADD_ITEM => {
                            let (item, count, page) = (item_memory[0x01] as u16, item_memory[0x02] as u16, item_memory[0x03] as usize);
                            let expected_page = RED_ITEM_POCKETS[item as usize] as usize;
                            let content = &mut api_state.inventory[expected_page].content;
                            
                            response = ApiResponse::CODE;

                            if page == expected_page || page == 0xFF { // We have the correct pocket
                                if let Some(slot) = content.iter_mut().find(|&&mut x| {x[0] == item}) {
                                    if slot[1] < (1000 - count) { // We have the item & the total - req count is under 999
                                        slot[1] += count; // Increase the count of the pocket
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                } else {
                                    if content.len() < MAX_ITEM_COUNTS[expected_page] { // We dont have the item, but have empty slots
                                        content.push([item, count]); // Add the item to the pocket
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                }
                            }
                        },
                        ITEM_HAS_ITEM => {
                            let (item, count) = (item_memory[0x01] as u16, item_memory[0x02] as u16);

                            response = ApiResponse::APIBUFFER;

                            let hide_balls = *OAKS_PARCEL_OBTAINED.lock().unwrap();
                            for i in 0x00..0x04 {
                                if !(hide_balls && (i as u8) == P_BALL) { // Skip the ball pocket if we have oaks parcel
                                    let content = &api_state.inventory[i].content;
                                    if let Some(index) = content.iter().position(|&x| {x[0] == item}) {
                                        if content[index][1] >= count { // We have at least the count requested
                                            item_memory[0x00] = ITEM_TRUE;
                                            item_memory[0x01] = i as u8;
                                            item_memory[0x02] = index as u8;
                                            item_memory[0x03] = if content[index][1] > 99 { 99 } else { content[index][1] as u8 };
                                            break;
                                        }
                                    }
                                }
                            }
                        },
                        ITEM_REMOVE_ITEM => {
                            let (index, count, page) = (item_memory[0x01] as usize, item_memory[0x02] as u16, item_memory[0x03] as usize);
                            let content = &mut api_state.inventory[page].content;

                            response = ApiResponse::CODE;

                            if content.len() > index { // if the index is a valid position
                                if content[index][0] == 0x0046 {
                                    *OAKS_PARCEL_OBTAINED.lock().unwrap() = false; // removing oaks parcel
                                }
                                if content[index][1] >= count { // We have at least the count requested
                                    content[index][1] -= count;
                                    if content[index][1] == 0x00 { // If we empty the item stack
                                        content.remove(index);
                                        response = ApiResponse::APIBUFFER;
                                        item_memory[0x00] = ITEM_NULL;
                                        item_memory[0x01] = page as u8;
                                        item_memory[0x02] = content.len() as u8;
                                    } else {
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                }
                            }
                        },
                        ITEM_CAN_GET_PC_ITEM => { // only one page, so ignore page param
                            let (item, count) = (item_memory[0x01] as u16, item_memory[0x02] as u16);
                            let content = &api_state.inventory[P_PC as usize].content;
                            
                            response = ApiResponse::CODE;

                            if let Some(slot) = content.iter().find(|&&x| {x[0] == item}) {
                                if slot[1] < (100 - count) { // We have the item & the total - req count is under 99
                                    item_memory[0x00] = ITEM_TRUE; 
                                }
                            } else {
                                if content.len() < MAX_ITEM_COUNTS[P_PC as usize] { // We dont have the item, but have empty slots
                                    item_memory[0x00] = ITEM_TRUE;
                                }
                            }
                        },
                        ITEM_ADD_ITEM_TO_PC => {
                            let (item, count) = (item_memory[0x01] as u16, item_memory[0x02] as u16);
                            let content = &mut api_state.inventory[P_PC as usize].content;
                            
                            response = ApiResponse::CODE;

                            if let Some(slot) = content.iter_mut().find(|&&mut x| {x[0] == item}) {
                                if slot[1] < (1000 - count) { // We have the item & the total - req count is under 999
                                    slot[1] += count; // Increase the count of the pocket
                                    item_memory[0x00] = ITEM_TRUE;
                                }
                            } else {
                                if content.len() < MAX_ITEM_COUNTS[P_PC as usize] { // We dont have the item, but have empty slots
                                    content.push([item, count]); // Add the item to the pocket
                                    item_memory[0x00] = ITEM_TRUE;
                                }
                            }
                        },
                        ITEM_HAS_ITEM_IN_PC => {
                            let (item, count) = (item_memory[0x01] as u16, item_memory[0x02] as u16);

                            response = ApiResponse::APIBUFFER;

                            let content = &api_state.inventory[P_PC as usize].content;
                            if let Some(index) = content.iter().position(|&x| {x[0] == item}) {
                                if content[index][1] >= count { // We have at least the count requested
                                    item_memory[0x00] = ITEM_TRUE;
                                    item_memory[0x01] = 0x00;
                                    item_memory[0x02] = index as u8;
                                    item_memory[0x03] = content[index][1] as u8;
                                }
                            }
                        },
                        ITEM_REMOVE_ITEM_FROM_PC => {
                            let (index, count) = (item_memory[0x01] as usize, item_memory[0x02] as u16);
                            let content = &mut api_state.inventory[P_PC as usize].content;

                            response = ApiResponse::CODE;

                            if content.len() > index { // if the index is a valid position
                                if content[index][1] >= count { // We have at least the count requested
                                    content[index][1] -= count;
                                    if content[index][1] == 0x00 { // If we empty the item stack
                                        content.remove(index);
                                        response = ApiResponse::APIBUFFER;
                                        item_memory[0x00] = ITEM_NULL;
                                        item_memory[0x01] = 0x00;
                                        item_memory[0x02] = content.len() as u8;
                                    } else {
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                }
                            }
                        },
                        ITEM_DEPOSIT => {
                            let (page, index, count) = (item_memory[0x01] as usize, item_memory[0x02] as usize, item_memory[0x03] as u16);
                            let content = &mut api_state.inventory[page].content;

                            let mut transfered_item = [0,0];

                            response = ApiResponse::CODE;

                            if content.len() > index { // if the index is a valid position
                                if content[index][1] >= count { // We have at least the count requested
                                    transfered_item = [content[index][0], count];
                                    item_memory[0x00] = ITEM_TRUE;
                                }
                            }

                            let content = &mut api_state.inventory[P_PC as usize].content;
                            
                            if item_memory[0x00] != ITEM_FALSE { // if we passed the previous check
                                if let Some(slot) = content.iter_mut().find(|&&mut x| {x[0] == transfered_item[0]}) {
                                    if slot[1] < (100 - count) { // We have the item & the total - req count is under 99
                                        slot[1] += count; // Increase the count of the pocket
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                } else {
                                    if content.len() < MAX_ITEM_COUNTS[P_PC as usize] { // We dont have the item, but have empty slots
                                        content.push(transfered_item); // Add the item to the pocket
                                        item_memory[0x00] = ITEM_TRUE;
                                    }
                                }
                            }

                            let content = &mut api_state.inventory[page].content;

                            if item_memory[0x00] != ITEM_FALSE { // if we passed the previous check
                                content[index][1] -= count;
                                if content[index][1] == 0x00 { // If we empty the item stack
                                    content.remove(index);
                                }
                                item_memory[0x00] = ITEM_TRUE;
                            }
                        },
                        ITEM_WITHDRAW => {
                            let (index, count) = (item_memory[0x02] as usize, item_memory[0x03] as u16);
                            let content = &mut api_state.inventory[P_PC as usize].content;

                            let mut transfered_item = [0,0];

                            response = ApiResponse::CODE;

                            if content.len() > index { // if the index is a valid position
                                if content[index][1] >= count { // We have at least the count requested
                                    transfered_item = [content[index][0], count];
                                    item_memory[0x00] = ITEM_TRUE;
                                }
                            }

                            for i in 0x00..0x04 {
                                let content = &mut api_state.inventory[i].content;
                            
                                if item_memory[0x00] != ITEM_FALSE { // if we passed the previous check
                                    if let Some(slot) = content.iter_mut().find(|&&mut x| {x[0] == transfered_item[0]}) {
                                        if slot[1] < (1000 - count) { // We have the item & the total - req count is under 999
                                            slot[1] += count; // Increase the count of the pocket
                                            item_memory[0x00] = ITEM_TRUE;
                                            break;
                                        }
                                    } else {
                                        if content.len() < MAX_ITEM_COUNTS[P_PC as usize] { // We dont have the item, but have empty slots
                                            content.push(transfered_item); // Add the item to the pocket
                                            item_memory[0x00] = ITEM_TRUE;
                                            break;
                                        }
                                    }
                                } 
                            }
                            
                            let content = &mut api_state.inventory[P_PC as usize].content;

                            if item_memory[0x00] != ITEM_FALSE { // if we passed the previous check
                                content[index][1] -= count;
                                if content[index][1] == 0x00 { // If we empty the item stack
                                    content.remove(index);
                                }
                                item_memory[0x00] = ITEM_TRUE;
                            }
                        },
                        ITEM_SWAP_ITEMS => {
                            let (page1, index1, page2, index2) = (item_memory[0x01] as usize, item_memory[0x02] as usize, item_memory[0x03] as usize, item_memory[0x04] as usize);

                            response = ApiResponse::CODE;

                            if page1 == page2 { // Swaps must occur on the same page
                                let content = &mut api_state.inventory[page1].content;
                                if index1 < content.len() && index2 < content.len() {
                                    content.swap(index1, index2);
                                    item_memory[0x00] = ITEM_TRUE;
                                }
                            }
                        },
                        ITEM_SWAP_PC_ITEMS => {
                            let (index1, index2) = (item_memory[0x02] as usize, item_memory[0x04] as usize);

                            response = ApiResponse::CODE;

                            let content = &mut api_state.inventory[P_PC as usize].content;
                            if index1 < content.len() && index2 < content.len() {
                                content.swap(index1, index2);
                                item_memory[0x00] = ITEM_TRUE;
                            }
                        },
                        ITEM_IS_BAG_EMPTY => {
                            response = ApiResponse::CODE;

                            item_memory[0x00] = ITEM_TRUE;

                            for i in 0x00..0x04 {
                                if !api_state.inventory[i].content.is_empty() {
                                    item_memory[0x00] = ITEM_FALSE;
                                    break;
                                }
                            }
                        },
                        ITEM_IS_PC_EMPTY => {
                            response = ApiResponse::CODE;

                            if api_state.inventory[P_PC as usize].content.is_empty() {
                                item_memory[0x00] = ITEM_TRUE;
                            }
                        },
                        ITEM_GET_ITEM_QUANTITIES => {
                            let item_id_buffer = item_memory[0x01..0x11].to_vec();

                            for i in &mut item_memory[0x01..0x11] {
                                *i = 0x00;
                            }

                            response = ApiResponse::CODE;

                            for (i, item) in item_id_buffer.iter().enumerate() {
                                for j in 0x00..0x04 {
                                    let content = &api_state.inventory[j].content;
                                    if let Some(slot) = content.iter().find(|&&x| {x[0] as u8 == *item}) {
                                        response = ApiResponse::APIBUFFER;
                                        item_memory[0x00] = ITEM_TRUE;
                                        item_memory[i] = slot[1] as u8;
                                    }
                                }
                            }
                        },
                        ITEM_GET_PAGE_LIMITS => {
                            response = ApiResponse::APIBUFFER;

                            item_memory[0x00] = ITEM_TRUE;
                            item_memory[0x01] = 0x04;
                            item_memory[0x02] = 0x01;
                        },
                        _ => {}
                    }; 
                }
            } else if code < 0x80 {
                let content = &api_state.inventory[P_PC as usize].content;
                response = ApiResponse::PAGE;
                item_memory[0x00] = ITEM_TRUE;
                item_memory[0x01..0x0E].clone_from_slice(&api_state.inventory[P_PC as usize].name);
                item_memory[0x11] = content.len() as u8;
                for (i, item) in content.iter().enumerate() {
                    item_memory[0x12 + i*2] = item[0] as u8;
                    item_memory[0x13 + i*2] = if item[1] > 99 { 99 } else { item[1] as u8 };
                }
                item_memory[0x12 + content.len() * 2] = 0xff;
            } else {
                let page = code & 0x7f;
                let content = &api_state.inventory[page as usize].content;
                response = ApiResponse::PAGE;
                item_memory[0x00] = ITEM_TRUE;
                item_memory[0x01..0x0E].clone_from_slice(&api_state.inventory[page as usize].name);
                item_memory[0x11] = content.len() as u8;
                for (i, item) in content.iter().enumerate() {
                    item_memory[0x12 + i*2] = item[0] as u8;
                    item_memory[0x13 + i*2] = if item[1] > 99 { 99 } else { item[1] as u8 };
                }
                item_memory[0x12 + content.len() * 2] = 0xff;
            }
            
            match response {
                ApiResponse::NONE => {},
                ApiResponse::CODE => {
                    BIZHAWK.write_u8_sym(&SYM["wItemAPICommand"], item_memory[0x00]).unwrap();
                    //println!("ItemAPI resp: code:{:x}", item_memory[0x00]);
                },
                ApiResponse::APIBUFFER => {
                    BIZHAWK.write_slice_sym(&SYM["wItemAPICommand"], &item_memory[0x00..0x11]).unwrap();
                    //println!("ItemAPI resp: code:{:x}, params{:?}", item_memory[0x00], &item_memory[0x01..0x11]);
                },
                ApiResponse::PAGE => {
                    BIZHAWK.write_slice_sym(&SYM["wItemAPICommand"], &item_memory[0x00..0x11]).unwrap();
                    BIZHAWK.write_slice_sym(&SYM["wNumItems"], &item_memory[0x11..]).unwrap();
                    //println!("ItemAPI resp: code:{:x}, params:{:?}, items:{:?}", item_memory[0x00], &item_memory[0x01..0x11], &item_memory[0x11..]);
                }
            }
        }
    } else {
        // Bag is being accessed by another thread, write timeout
        BIZHAWK.write_u8_sym(&SYM["wItemAPICommand"], 0x03).unwrap();
    }
    "ok"
}