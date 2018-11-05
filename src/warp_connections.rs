use maplit::{hashmap};
use std::collections::HashMap;
use crate::constants::*;

fn get_connections() -> HashMap<(u8, u8, u8), (u16, u8)> {
    hashmap!(
//      (Map ID                     , Warp, Last Map                    ) => (MapIDMapBank, Warp ID)
        // PALLET_TOWN
        (REDS_HOUSE_1F              , 0x00, PALLET_TOWN                 ) => (0x0004, 0x01), 
        (BLUES_HOUSE                , 0x00, PALLET_TOWN                 ) => (0x0204, 0x00), 
        (OAKS_LAB                   , 0x01, PALLET_TOWN                 ) => (0x0304, 0x00),

        //REDS_HOUSE_1F
        (LAST_MAP                   , 0x00, PALLET_TOWN                 ) => (0x0003, 0x00),   
        (REDS_HOUSE_2F              , 0x00, PALLET_TOWN                 ) => (0x0104, 0x00),      

        //REDS_HOUSE_2F
        (REDS_HOUSE_1F              , 0x02, PALLET_TOWN                 ) => (0x0004, 0x02),   

        //BLUES_HOUSE
        (LAST_MAP                   , 0x01, PALLET_TOWN                 ) => (0x0003, 0x01),   

        //OAKS_LAB
        (LAST_MAP                   , 0x02, PALLET_TOWN                 ) => (0x0003, 0x02),

        // VIRIDIAN_CITY
        (VIRIDIAN_POKECENTER        , 0x00, VIRIDIAN_CITY               ) => (0x0405, 0x01),
        (VIRIDIAN_MART              , 0x00, VIRIDIAN_CITY               ) => (0x0305, 0x01),
        (VIRIDIAN_SCHOOL            , 0x00, VIRIDIAN_CITY               ) => (0x0205, 0x01),
        (VIRIDIAN_HOUSE             , 0x00, VIRIDIAN_CITY               ) => (0x0005, 0x01),
        (VIRIDIAN_GYM               , 0x00, VIRIDIAN_CITY               ) => (0x0105, 0x01),

        //VIRIDIAN_POKECENTER
        (LAST_MAP                   , 0x00, VIRIDIAN_CITY               ) => (0x0103, 0x00),

        //VIRIDIAN_MART
        (LAST_MAP                   , 0x01, VIRIDIAN_CITY               ) => (0x0103, 0x04),

        //VIRIDIAN_GYM
        (LAST_MAP                   , 0x04, VIRIDIAN_CITY               ) => (0x0103, 0x02),

        //VIRIDIAN_HOUSE
        (LAST_MAP                   , 0x03, VIRIDIAN_CITY               ) => (0x0103, 0x01),

        //VIRIDIAN_SCHOOL
        (LAST_MAP                   , 0x02, VIRIDIAN_CITY               ) => (0x0103, 0x03),

        // PEWTER_CITY
        (MUSEUM_1F                  , 0x00, PEWTER_CITY                 ) => (0x0006, 0x01),
        (MUSEUM_1F                  , 0x02, PEWTER_CITY                 ) => (0x0006, 0x03),
        (PEWTER_GYM                 , 0x00, PEWTER_CITY                 ) => (0x0206, 0x01),
        (PEWTER_HOUSE_1             , 0x00, PEWTER_CITY                 ) => (0x0406, 0x01),
        (PEWTER_MART                , 0x00, PEWTER_CITY                 ) => (0x0306, 0x01),
        (PEWTER_HOUSE_2             , 0x00, PEWTER_CITY                 ) => (0x0706, 0x01),
        (PEWTER_POKECENTER          , 0x00, PEWTER_CITY                 ) => (0x0506, 0x01),

        //PEWTER_POKECENTER
        (LAST_MAP                   , 0x06, PEWTER_CITY                 ) => (0x0203, 0x05),

        //PEWTER_MART
        (LAST_MAP                   , 0x04, PEWTER_CITY                 ) => (0x0203, 0x03),

        //PEWTER_GYM
        (LAST_MAP                   , 0x02, PEWTER_CITY                 ) => (0x0203, 0x02),

        //PEWTER_HOUSE_1
        (LAST_MAP                   , 0x03, PEWTER_CITY                 ) => (0x0203, 0x04),

        //PEWTER_HOUSE_2
        (LAST_MAP                   , 0x05, PEWTER_CITY                 ) => (0x0203, 0x06),

        //MUSEUM_1F
        (LAST_MAP                   , 0x00, PEWTER_CITY                 ) => (0x0203, 0x00),
        (LAST_MAP                   , 0x01, PEWTER_CITY                 ) => (0x0203, 0x01),
        (MUSEUM_2F                  , 0x00, PEWTER_CITY                 ) => (0x0106, 0x00),

        //MUSEUM_2F
        (MUSEUM_1F                  , 0x04, PEWTER_CITY                 ) => (0x0006, 0x05),

        // CERULEAN_CITY
        (TRASHED_HOUSE              , 0x00, CERULEAN_CITY               ) => (0x0107, 0x01),
        (CERULEAN_HOUSE_1           , 0x00, CERULEAN_CITY               ) => (0x0207, 0x01),
        (CERULEAN_POKECENTER        , 0x00, CERULEAN_CITY               ) => (0x0307, 0x01),
        (CERULEAN_GYM               , 0x00, CERULEAN_CITY               ) => (0x0507, 0x01),
        (BIKE_SHOP                  , 0x00, CERULEAN_CITY               ) => (0x0607, 0x01),
        (CERULEAN_MART              , 0x00, CERULEAN_CITY               ) => (0x0707, 0x01),
        (UNKNOWN_DUNGEON_1          , 0x00, CERULEAN_CITY               ) => (0x4801, 0x00),
        (TRASHED_HOUSE              , 0x02, CERULEAN_CITY               ) => (0x0107, 0x03),
        (CERULEAN_HOUSE_2           , 0x01, CERULEAN_CITY               ) => (0x0007, 0x01),
        (CERULEAN_HOUSE_2           , 0x00, CERULEAN_CITY               ) => (0x0007, 0x03),

        //CERULEAN_POKECENTER
        (LAST_MAP                   , 0x02, CERULEAN_CITY               ) => (0x0303, 0x03),

        //CERULEAN_MART
        (LAST_MAP                   , 0x05, CERULEAN_CITY               ) => (0x0303, 0x06),

        //CERULEAN_GYM
        (LAST_MAP                   , 0x03, CERULEAN_CITY               ) => (0x0303, 0x04),

        //CERULEAN_HOUSE_1
        (LAST_MAP                   , 0x01, CERULEAN_CITY               ) => (0x0303, 0x02),

        //CERULEAN_HOUSE_2
        (LAST_MAP                   , 0x09, CERULEAN_CITY               ) => (0x0303, 0x08),
        (LAST_MAP                   , 0x08, CERULEAN_CITY               ) => (0x0303, 0x00),

        //TRASHED_HOUSE
        (LAST_MAP                   , 0x00, CERULEAN_CITY               ) => (0x0303, 0x01),
        (LAST_MAP                   , 0x07, CERULEAN_CITY               ) => (0x0303, 0x09),

        //BIKE_SHOP
        (LAST_MAP                   , 0x04, CERULEAN_CITY               ) => (0x0303, 0x05),

        //UNKNOWN_DUNGEON_1
        (LAST_MAP                   , 0x06, CERULEAN_CITY               ) => (0x0303, 0x07),
        (UNKNOWN_DUNGEON_2          , 0x00, CERULEAN_CITY               ) => (0x4901, 0x00),
        (UNKNOWN_DUNGEON_2          , 0x01, CERULEAN_CITY               ) => (0x4901, 0x03),
        (UNKNOWN_DUNGEON_2          , 0x02, CERULEAN_CITY               ) => (0x4901, 0x04),
        (UNKNOWN_DUNGEON_2          , 0x03, CERULEAN_CITY               ) => (0x4901, 0x01),
        (UNKNOWN_DUNGEON_2          , 0x04, CERULEAN_CITY               ) => (0x4901, 0x05),
        (UNKNOWN_DUNGEON_2          , 0x05, CERULEAN_CITY               ) => (0x4901, 0x02),
        (UNKNOWN_DUNGEON_3          , 0x00, CERULEAN_CITY               ) => (0x4A01, 0x00),

        //UNKNOWN_DUNGEON_2
        (UNKNOWN_DUNGEON_1          , 0x02, CERULEAN_CITY               ) => (0x4801, 0x01),
        (UNKNOWN_DUNGEON_1          , 0x03, CERULEAN_CITY               ) => (0x4801, 0x05),
        (UNKNOWN_DUNGEON_1          , 0x04, CERULEAN_CITY               ) => (0x4801, 0x06),
        (UNKNOWN_DUNGEON_1          , 0x05, CERULEAN_CITY               ) => (0x4801, 0x03),
        (UNKNOWN_DUNGEON_1          , 0x06, CERULEAN_CITY               ) => (0x4801, 0x07),
        (UNKNOWN_DUNGEON_1          , 0x07, CERULEAN_CITY               ) => (0x4801, 0x04),

        //UNKNOWN_DUNGEON_3
        (UNKNOWN_DUNGEON_1          , 0x08, CERULEAN_CITY               ) => (0x4801, 0x02),

        // LAVENDER_TOWN
        (LAVENDER_POKECENTER        , 0x00, LAVENDER_TOWN               ) => (0x0008, 0x01), 
        (POKEMONTOWER_1             , 0x00, LAVENDER_TOWN               ) => (0x5801, 0x01),
        (LAVENDER_HOUSE_1           , 0x00, LAVENDER_TOWN               ) => (0x0208, 0x01),
        (LAVENDER_MART              , 0x00, LAVENDER_TOWN               ) => (0x0508, 0x01),
        (LAVENDER_HOUSE_2           , 0x00, LAVENDER_TOWN               ) => (0x0308, 0x01),
        (NAME_RATERS_HOUSE          , 0x00, LAVENDER_TOWN               ) => (0x0408, 0x01),

        //LAVENDER_POKECENTER
        (LAST_MAP                   , 0x00, LAVENDER_TOWN               ) => (0x0403, 0x01),

        //LAVENDER_MART
        (LAST_MAP                   , 0x03, LAVENDER_TOWN               ) => (0x0403, 0x05),

        //LAVENDER_HOUSE_1
        (LAST_MAP                   , 0x02, LAVENDER_TOWN               ) => (0x0403, 0x02),

        //LAVENDER_HOUSE_2
        (LAST_MAP                   , 0x04, LAVENDER_TOWN               ) => (0x0403, 0x04),

        //POKEMONTOWER_1
        (LAST_MAP                   , 0x01, LAVENDER_TOWN               ) => (0x0403, 0x00),
        (POKEMONTOWER_2             , 0x01, LAVENDER_TOWN               ) => (0x5901, 0x01),

        //POKEMONTOWER_2
        (POKEMONTOWER_3             , 0x00, LAVENDER_TOWN               ) => (0x5A01, 0x00),
        (POKEMONTOWER_1             , 0x02, LAVENDER_TOWN               ) => (0x5801, 0x03),

        //POKEMONTOWER_3
        (POKEMONTOWER_2             , 0x00, LAVENDER_TOWN               ) => (0x5901, 0x00),
        (POKEMONTOWER_4             , 0x01, LAVENDER_TOWN               ) => (0x5B01, 0x01),

        //POKEMONTOWER_4
        (POKEMONTOWER_5             , 0x00, LAVENDER_TOWN               ) => (0x5C01, 0x00),
        (POKEMONTOWER_3             , 0x01, LAVENDER_TOWN               ) => (0x5A01, 0x01),

        //POKEMONTOWER_5
        (POKEMONTOWER_4             , 0x00, LAVENDER_TOWN               ) => (0x5B01, 0x00),
        (POKEMONTOWER_6             , 0x00, LAVENDER_TOWN               ) => (0x5D01, 0x01),

        //POKEMONTOWER_6
        (POKEMONTOWER_5             , 0x01, LAVENDER_TOWN               ) => (0x5C01, 0x01),
        (POKEMONTOWER_7             , 0x00, LAVENDER_TOWN               ) => (0x5E01, 0x00),

        //POKEMONTOWER_7
        (POKEMONTOWER_6             , 0x01, LAVENDER_TOWN               ) => (0x5D01, 0x00),

        //NAME_RATERS_HOUSE
        (LAST_MAP                   , 0x05, LAVENDER_TOWN               ) => (0x0403, 0x03),

        // VERMILION_CITY
        (VERMILION_POKECENTER       , 0x00, VERMILION_CITY              ) => (0x0109, 0x01), 
        (POKEMON_FAN_CLUB           , 0x00, VERMILION_CITY              ) => (0x0309, 0x01),
        (VERMILION_MART             , 0x00, VERMILION_CITY              ) => (0x0509, 0x01),
        (VERMILION_GYM              , 0x00, VERMILION_CITY              ) => (0x0609, 0x01),
        (VERMILION_HOUSE_1          , 0x00, VERMILION_CITY              ) => (0x0709, 0x01),
        (VERMILION_DOCK             , 0x00, VERMILION_CITY              ) => (0x0401, 0x01),
        (VERMILION_HOUSE_3          , 0x00, VERMILION_CITY              ) => (0x0409, 0x01),
        (VERMILION_HOUSE_2          , 0x00, VERMILION_CITY              ) => (0x0009, 0x01),

        //VERMILION_POKECENTER
        (LAST_MAP                   , 0x00, VERMILION_CITY              ) => (0x0503, 0x04),

        //VERMILION_MART
        (LAST_MAP                   , 0x02, VERMILION_CITY              ) => (0x0503, 0x07),

        //VERMILION_GYM
        (LAST_MAP                   , 0x03, VERMILION_CITY              ) => (0x0503, 0x09),

        //VERMILION_HOUSE_1
        (LAST_MAP                   , 0x04, VERMILION_CITY              ) => (0x0503, 0x08),

        //VERMILION_HOUSE_2
        (LAST_MAP                   , 0x08, VERMILION_CITY              ) => (0x0503, 0x03),

        //VERMILION_HOUSE_3
        (LAST_MAP                   , 0x07, VERMILION_CITY              ) => (0x0503, 0x06),

        //VERMILION_DOCK
        (LAST_MAP                   , 0x05, VERMILION_CITY              ) => (0x0503, 0x01),
        (SS_ANNE_1                  , 0x01, VERMILION_CITY              ) => (0x0501, 0x02),

        //SS_ANNE_1
        (VERMILION_DOCK             , 0x01, VERMILION_CITY              ) => (0x0401, 0x02),
        (SS_ANNE_8                  , 0x00, VERMILION_CITY              ) => (0x1101, 0x00),
        (SS_ANNE_8                  , 0x01, VERMILION_CITY              ) => (0x1001, 0x00),
        (SS_ANNE_8                  , 0x02, VERMILION_CITY              ) => (0x0F01, 0x00),
        (SS_ANNE_8                  , 0x03, VERMILION_CITY              ) => (0x0E01, 0x00),
        (SS_ANNE_8                  , 0x04, VERMILION_CITY              ) => (0x0D01, 0x00),
        (SS_ANNE_8                  , 0x05, VERMILION_CITY              ) => (0x0C01, 0x00),
        (SS_ANNE_2                  , 0x06, VERMILION_CITY              ) => (0x0601, 0x00),
        (SS_ANNE_4                  , 0x05, VERMILION_CITY              ) => (0x0801, 0x00),
        (SS_ANNE_6                  , 0x00, VERMILION_CITY              ) => (0x0A01, 0x00),

        //SS_ANNE_2
        (SS_ANNE_9                  , 0x00, VERMILION_CITY              ) => (0x1201, 0x00),
        (SS_ANNE_9                  , 0x02, VERMILION_CITY              ) => (0x1301, 0x00),
        (SS_ANNE_9                  , 0x04, VERMILION_CITY              ) => (0x1401, 0x00),
        (SS_ANNE_9                  , 0x06, VERMILION_CITY              ) => (0x1501, 0x00),
        (SS_ANNE_9                  , 0x08, VERMILION_CITY              ) => (0x1601, 0x00),
        (SS_ANNE_9                  , 0x0A, VERMILION_CITY              ) => (0x1701, 0x00),
        (SS_ANNE_1                  , 0x08, VERMILION_CITY              ) => (0x0501, 0x00),
        (SS_ANNE_3                  , 0x01, VERMILION_CITY              ) => (0x0701, 0x01),
        (SS_ANNE_7                  , 0x00, VERMILION_CITY              ) => (0x0B01, 0x00),

        //SS_ANNE_3
        (SS_ANNE_5                  , 0x00, VERMILION_CITY              ) => (0x0601, 0x01),
        (SS_ANNE_2                  , 0x07, VERMILION_CITY              ) => (0x0901, 0x00),

        //SS_ANNE_4
        (SS_ANNE_10                 , 0x08, VERMILION_CITY              ) => (0x1C01, 0x00),
        (SS_ANNE_10                 , 0x06, VERMILION_CITY              ) => (0x1B01, 0x00),
        (SS_ANNE_10                 , 0x04, VERMILION_CITY              ) => (0x1A01, 0x00),
        (SS_ANNE_10                 , 0x02, VERMILION_CITY              ) => (0x1901, 0x00),
        (SS_ANNE_10                 , 0x00, VERMILION_CITY              ) => (0x1801, 0x00),
        (SS_ANNE_1                  , 0x09, VERMILION_CITY              ) => (0x0501, 0x05),

        //SS_ANNE_5
        (SS_ANNE_3                  , 0x00, VERMILION_CITY              ) => (0x0701, 0x00),

        //SS_ANNE_6
        (SS_ANNE_1                  , 0x0A, VERMILION_CITY              ) => (0x0501, 0x01),

        //SS_ANNE_7
        (SS_ANNE_2                  , 0x08, VERMILION_CITY              ) => (0x0601, 0x02),

        //SS_ANNE_8
        (SS_ANNE_1                  , 0x02, VERMILION_CITY              ) => (0x0501, 0x0B),
        (SS_ANNE_1                  , 0x03, VERMILION_CITY              ) => (0x0501, 0x0A),
        (SS_ANNE_1                  , 0x04, VERMILION_CITY              ) => (0x0501, 0x09),
        (SS_ANNE_1                  , 0x05, VERMILION_CITY              ) => (0x0501, 0x08),
        (SS_ANNE_1                  , 0x06, VERMILION_CITY              ) => (0x0501, 0x07),
        (SS_ANNE_1                  , 0x07, VERMILION_CITY              ) => (0x0501, 0x06),

        //SS_ANNE_9
        (SS_ANNE_2                  , 0x00, VERMILION_CITY              ) => (0x0601, 0x03),
        (SS_ANNE_2                  , 0x01, VERMILION_CITY              ) => (0x0601, 0x04),
        (SS_ANNE_2                  , 0x02, VERMILION_CITY              ) => (0x0601, 0x05),
        (SS_ANNE_2                  , 0x03, VERMILION_CITY              ) => (0x0601, 0x06),
        (SS_ANNE_2                  , 0x04, VERMILION_CITY              ) => (0x0601, 0x07),
        (SS_ANNE_2                  , 0x05, VERMILION_CITY              ) => (0x0601, 0x08),

        //SS_ANNE_10
        (SS_ANNE_4                  , 0x04, VERMILION_CITY              ) => (0x0801, 0x05),
        (SS_ANNE_4                  , 0x03, VERMILION_CITY              ) => (0x0801, 0x04),
        (SS_ANNE_4                  , 0x02, VERMILION_CITY              ) => (0x0801, 0x03),
        (SS_ANNE_4                  , 0x01, VERMILION_CITY              ) => (0x0801, 0x02),
        (SS_ANNE_4                  , 0x00, VERMILION_CITY              ) => (0x0801, 0x01),

        //POKEMON_FAN_CLUB
        (LAST_MAP                   , 0x01, VERMILION_CITY              ) => (0x0503, 0x05),

        // CELADON_CITY
        (CELADON_MART_1             , 0x00, CELADON_CITY                ) => (0x000A, 0x01), 
        (CELADON_MART_1             , 0x02, CELADON_CITY                ) => (0x000A, 0x04),
        (CELADON_MANSION_1          , 0x00, CELADON_CITY                ) => (0x070A, 0x01),
        (CELADON_MANSION_1          , 0x02, CELADON_CITY                ) => (0x070A, 0x05),
        (CELADON_POKECENTER         , 0x00, CELADON_CITY                ) => (0x0C0A, 0x01),
        (CELADON_GYM                , 0x00, CELADON_CITY                ) => (0x100A, 0x01),
        (GAME_CORNER                , 0x00, CELADON_CITY                ) => (0x0E0A, 0x01),
        (CELADON_PRIZE_ROOM         , 0x00, CELADON_CITY                ) => (0x0F0A, 0x01),
        (CELADON_DINER              , 0x00, CELADON_CITY                ) => (0x110A, 0x01),
        (CELADON_HOUSE              , 0x00, CELADON_CITY                ) => (0x120A, 0x01),
        (CELADON_HOTEL              , 0x00, CELADON_CITY                ) => (0x130A, 0x01),

        //CELADON_POKECENTER
        (LAST_MAP                   , 0x05, CELADON_CITY                ) => (0x0603, 0x04),

        //CELADON_MART_1
        (LAST_MAP                   , 0x00, CELADON_CITY                ) => (0x0603, 0x01),
        (LAST_MAP                   , 0x01, CELADON_CITY                ) => (0x0603, 0x02),
        (CELADON_MART_2             , 0x00, CELADON_CITY                ) => (0x0110, 0x01),

        //CELADON_MART_2
        (CELADON_MART_1             , 0x04, CELADON_CITY                ) => (0x0010, 0x07),
        (CELADON_MART_3             , 0x01, CELADON_CITY                ) => (0x0210, 0x01),

        //CELADON_MART_3
        (CELADON_MART_4             , 0x00, CELADON_CITY                ) => (0x0310, 0x01),
        (CELADON_MART_2             , 0x01, CELADON_CITY                ) => (0x0110, 0x02),

        //CELDAON_MART_4
        (CELADON_MART_3             , 0x00, CELADON_CITY                ) => (0x0210, 0x02),
        (CELADON_MART_5             , 0x01, CELADON_CITY                ) => (0x0410, 0x01),

        //CELADON_MART_5
        (CELADON_MART_ROOF          , 0x00, CELADON_CITY                ) => (0x0510, 0x00),
        (CELADON_MART_4             , 0x01, CELADON_CITY                ) => (0x0310, 0x02),

        //CELADON_MART_ROOF
        (CELADON_MART_5             , 0x00, CELADON_CITY                ) => (0x0410, 0x02),

        //CELADON_GYM
        (LAST_MAP                   , 0x06, CELADON_CITY                ) => (0x0603, 0x06),

        //GAME_CORNER
        (LAST_MAP                   , 0x07, CELADON_CITY                ) => (0x0603, 0x00),
        (ROCKET_HIDEOUT_1           , 0x01, CELADON_CITY                ) => (0x2A01, 0x00),

        //ROCKET_HIDEOUT_1
        (ROCKET_HIDEOUT_2           , 0x00, CELADON_CITY                ) => (0x2B01, 0x01),
        (GAME_CORNER                , 0x02, CELADON_CITY                ) => (0x0E0A, 0x03),
        (ROCKET_HIDEOUT_2           , 0x03, CELADON_CITY                ) => (0x2B01, 0x02),

        //ROCKET_HIDEOUT_2
        (ROCKET_HIDEOUT_1           , 0x00, CELADON_CITY                ) => (0x2A01, 0x01),
        (ROCKET_HIDEOUT_3           , 0x00, CELADON_CITY                ) => (0x2C01, 0x00),
        (ROCKET_HIDEOUT_1           , 0x03, CELADON_CITY                ) => (0x2A01, 0x02),

        //ROCKET_HIDEOUT_3
        (ROCKET_HIDEOUT_2           , 0x01, CELADON_CITY                ) => (0x2B01, 0x00),
        (ROCKET_HIDEOUT_4           , 0x00, CELADON_CITY                ) => (0x2D01, 0x00),

        //ROCKET_HIDEOUT_4
        (ROCKET_HIDEOUT_3           , 0x01, CELADON_CITY                ) => (0x2C01, 0x01),

        //CELADON_MANSION_1
        (LAST_MAP                   , 0x02, CELADON_CITY                ) => (0x0603, 0x03),
        (LAST_MAP                   , 0x04, CELADON_CITY                ) => (0x0603, 0x0B),
        (CELADON_MANSION_2          , 0x01, CELADON_CITY                ) => (0x080A, 0x03),
        (CELADON_MANSION_2          , 0x02, CELADON_CITY                ) => (0x080A, 0x00),

        //CELADON_MANSION_2
        (CELADON_MANSION_3          , 0x00, CELADON_CITY                ) => (0x090A, 0x03),
        (CELADON_MANSION_1          , 0x03, CELADON_CITY                ) => (0x070A, 0x04),
        (CELADON_MANSION_1          , 0x04, CELADON_CITY                ) => (0x070A, 0x03),
        (CELADON_MANSION_3          , 0x03, CELADON_CITY                ) => (0x090A, 0x00),

        //CELADON_MANSION_3
        (CELADON_MANSION_2          , 0x00, CELADON_CITY                ) => (0x080A, 0x02),
        (CELADON_MANSION_4          , 0x00, CELADON_CITY                ) => (0x0A0A, 0x01),
        (CELADON_MANSION_4          , 0x01, CELADON_CITY                ) => (0x0A0A, 0x00),
        (CELADON_MANSION_2          , 0x03, CELADON_CITY                ) => (0x080A, 0x01),

        //CELADON_MANSION_4
        (CELADON_MANSION_3          , 0x01, CELADON_CITY                ) => (0x090A, 0x02),
        (CELADON_MANSION_3          , 0x02, CELADON_CITY                ) => (0x090A, 0x01),
        (CELADON_MANSION_5          , 0x00, CELADON_CITY                ) => (0x0B0A, 0x01),

        //CELADON_MANSION_5
        (CELADON_MANSION_4          , 0x02, CELADON_CITY                ) => (0x0A0A, 0x02),

        //CELADON_PRIZE_ROOM
        (LAST_MAP                   , 0x09, CELADON_CITY                ) => (0x0603, 0x05),

        //CELADON_DINER
        (LAST_MAP                   , 0x0A, CELADON_CITY                ) => (0x0603, 0x07),

        //CELADON_HOUSE
        (LAST_MAP                   , 0x0B, CELADON_CITY                ) => (0x0603, 0x08),

        //CELADON_HOTEL
        (LAST_MAP                   , 0x0C, CELADON_CITY                ) => (0x0603, 0x09),

        // FUCHSIA_CITY
        (FUCHSIA_MART               , 0x00, FUCHSIA_CITY                ) => (0x010B, 0x01),
        (FUCHSIA_HOUSE_1            , 0x00, FUCHSIA_CITY                ) => (0x040B, 0x01),
        (FUCHSIA_POKECENTER         , 0x00, FUCHSIA_CITY                ) => (0x050B, 0x01),
        (FUCHSIA_HOUSE_2            , 0x00, FUCHSIA_CITY                ) => (0x070B, 0x01),
        (SAFARI_ZONE_ENTRANCE       , 0x00, FUCHSIA_CITY                ) => (0x000B, 0x02),
        (FUCHSIA_GYM                , 0x00, FUCHSIA_CITY                ) => (0x030B, 0x01),
        (FUCHSIA_MEETING_ROOM       , 0x00, FUCHSIA_CITY                ) => (0x020B, 0x01),
        (FUCHSIA_HOUSE_3            , 0x01, FUCHSIA_CITY                ) => (0x080B, 0x01),
        (FUCHSIA_HOUSE_3            , 0x00, FUCHSIA_CITY                ) => (0x080B, 0x03),

        //FUCHSIA_POKECENTER
        (LAST_MAP                   , 0x02, FUCHSIA_CITY                ) => (0x0703, 0x06),

        //FUCHSIA_MART
        (LAST_MAP                   , 0x00, FUCHSIA_CITY                ) => (0x0703, 0x02),

        //FUCHSIA_GYM
        (LAST_MAP                   , 0x05, FUCHSIA_CITY                ) => (0x0703, 0x04),

        //FUCHSIA_HOUSE_1
        (LAST_MAP                   , 0x01, FUCHSIA_CITY                ) => (0x0703, 0x05),

        //FUCHSIA_HOUSE_2
        (LAST_MAP                   , 0x03, FUCHSIA_CITY                ) => (0x0703, 0x01),

        //FUCHSIA_HOUSE_3
        (LAST_MAP                   , 0x08, FUCHSIA_CITY                ) => (0x0703, 0x08),
        (LAST_MAP                   , 0x07, FUCHSIA_CITY                ) => (0x0703, 0x07),

        //FUCHSIA_MEETING_ROOM
        (LAST_MAP                   , 0x06, FUCHSIA_CITY                ) => (0x0703, 0x03),

        //SAFARI_ZONE_ENTRANCE
        (LAST_MAP                   , 0x04, FUCHSIA_CITY                ) => (0x0703, 0x00),
        (SAFARI_ZONE_CENTER         , 0x00, FUCHSIA_CITY                ) => (0x3F01, 0x01),
        (SAFARI_ZONE_CENTER         , 0x01, FUCHSIA_CITY                ) => (0x3F01, 0x01),

        //SAFARI_ZONE_EAST
        (SAFARI_ZONE_NORTH          , 0x06, FUCHSIA_CITY                ) => (0x4101, 0x06),
        (SAFARI_ZONE_NORTH          , 0x07, FUCHSIA_CITY                ) => (0x4101, 0x07),
        (SAFARI_ZONE_CENTER         , 0x06, FUCHSIA_CITY                ) => (0x3F01, 0x09),
        (SAFARI_ZONE_REST_HOUSE_3   , 0x00, FUCHSIA_CITY                ) => (0x4401, 0x01),

        //SAFARI_ZONE_NORTH
        (SAFARI_ZONE_WEST           , 0x00, FUCHSIA_CITY                ) => (0x4201, 0x00),
        (SAFARI_ZONE_WEST           , 0x01, FUCHSIA_CITY                ) => (0x4201, 0x01),
        (SAFARI_ZONE_WEST           , 0x02, FUCHSIA_CITY                ) => (0x4201, 0x03),
        (SAFARI_ZONE_WEST           , 0x03, FUCHSIA_CITY                ) => (0x4201, 0x04),
        (SAFARI_ZONE_CENTER         , 0x04, FUCHSIA_CITY                ) => (0x3F01, 0x03),
        (SAFARI_ZONE_CENTER         , 0x05, FUCHSIA_CITY                ) => (0x3F01, 0x04),
        (SAFARI_ZONE_EAST           , 0x00, FUCHSIA_CITY                ) => (0x4001, 0x00),
        (SAFARI_ZONE_EAST           , 0x01, FUCHSIA_CITY                ) => (0x4001, 0x01),
        (SAFARI_ZONE_REST_HOUSE_4   , 0x00, FUCHSIA_CITY                ) => (0x4501, 0x01),

        //SAFARI_ZONE_WEST
        (SAFARI_ZONE_NORTH          , 0x00, FUCHSIA_CITY                ) => (0x4101, 0x00),
        (SAFARI_ZONE_NORTH          , 0x01, FUCHSIA_CITY                ) => (0x4101, 0x01),
        (SAFARI_ZONE_NORTH          , 0x02, FUCHSIA_CITY                ) => (0x4101, 0x03),
        (SAFARI_ZONE_NORTH          , 0x03, FUCHSIA_CITY                ) => (0x4101, 0x04),
        (SAFARI_ZONE_CENTER         , 0x02, FUCHSIA_CITY                ) => (0x3F01, 0x06),
        (SAFARI_ZONE_CENTER         , 0x03, FUCHSIA_CITY                ) => (0x3F01, 0x07),
        (SAFARI_ZONE_SECRET_HOUSE   , 0x00, FUCHSIA_CITY                ) => (0x4701, 0x01),
        (SAFARI_ZONE_REST_HOUSE_2   , 0x00, FUCHSIA_CITY                ) => (0x4601, 0x01),

        //SAFARI_ZONE_CENTER
        (SAFARI_ZONE_ENTRANCE       , 0x02, FUCHSIA_CITY                ) => (0x000B, 0x00),
        (SAFARI_ZONE_ENTRANCE       , 0x03, FUCHSIA_CITY                ) => (0x000B, 0x00),
        (SAFARI_ZONE_WEST           , 0x04, FUCHSIA_CITY                ) => (0x4201, 0x06),
        (SAFARI_ZONE_WEST           , 0x05, FUCHSIA_CITY                ) => (0x4201, 0x07),
        (SAFARI_ZONE_NORTH          , 0x04, FUCHSIA_CITY                ) => (0x4101, 0x09),
        (SAFARI_ZONE_NORTH          , 0x05, FUCHSIA_CITY                ) => (0x4101, 0x0A),
        (SAFARI_ZONE_EAST           , 0x02, FUCHSIA_CITY                ) => (0x4001, 0x03),
        (SAFARI_ZONE_EAST           , 0x03, FUCHSIA_CITY                ) => (0x4001, 0x04),
        (SAFARI_ZONE_REST_HOUSE_1   , 0x00, FUCHSIA_CITY                ) => (0x4301, 0x01),

        //SAFARI_ZONE_REST_HOUSE_1
        (SAFARI_ZONE_CENTER         , 0x08, FUCHSIA_CITY                ) => (0x3F01, 0x0C),

        //SAFARI_ZONE_SECRET_HOUSE
        (SAFARI_ZONE_WEST           , 0x06, FUCHSIA_CITY                ) => (0x4201, 0x09),

        //SAFARI_ZONE_REST_HOUSE_2
        (SAFARI_ZONE_WEST           , 0x07, FUCHSIA_CITY                ) => (0x4201, 0x0A),

        //SAFARI_ZONE_REST_HOUSE_3
        (SAFARI_ZONE_EAST           , 0x04, FUCHSIA_CITY                ) => (0x4001, 0x06),

        //SAFARI_ZONE_REST_HOUSE_4
        (SAFARI_ZONE_NORTH          , 0x08, FUCHSIA_CITY                ) => (0x4101, 0x0C),

        // CINNABAR_ISLAND
        (MANSION_1                  , 0x01, CINNABAR_ISLAND             ) => (0x3B01, 0x01),
        (CINNABAR_GYM               , 0x00, CINNABAR_ISLAND             ) => (0x000C, 0x01),
        (CINNABAR_LAB_1             , 0x00, CINNABAR_ISLAND             ) => (0x010C, 0x01),
        (CINNABAR_POKECENTER        , 0x00, CINNABAR_ISLAND             ) => (0x050C, 0x01),
        (CINNABAR_MART              , 0x00, CINNABAR_ISLAND             ) => (0x070C, 0x01),

        //CINNABAR_POKECENTER
        (LAST_MAP                   , 0x03, CINNABAR_ISLAND             ) => (0x0803, 0x03),

        //CINNABAR_MART
        (LAST_MAP                   , 0x04, CINNABAR_ISLAND             ) => (0x0803, 0x04),

        //CINNABAR_GYM
        (LAST_MAP                   , 0x01, CINNABAR_ISLAND             ) => (0x0803, 0x01),

        //CINNABAR_LAB_1
        (LAST_MAP                   , 0x02, CINNABAR_ISLAND             ) => (0x0803, 0x02),
        (CINNABAR_LAB_2             , 0x00, CINNABAR_ISLAND             ) => (0x020C, 0x00),
        (CINNABAR_LAB_3             , 0x00, CINNABAR_ISLAND             ) => (0x030C, 0x00),
        (CINNABAR_LAB_4             , 0x00, CINNABAR_ISLAND             ) => (0x040C, 0x00),

        //CINNABAR_LAB_2
        (CINNABAR_LAB_1             , 0x02, CINNABAR_ISLAND             ) => (0x010C, 0x03),

        //CINNABAR_LAB_3
        (CINNABAR_LAB_1             , 0x03, CINNABAR_ISLAND             ) => (0x010C, 0x04),

        //CINNABAR_LAB_4
        (CINNABAR_LAB_1             , 0x04, CINNABAR_ISLAND             ) => (0x010C, 0x05),

        //MANSION_1
        (LAST_MAP                   , 0x00, CINNABAR_ISLAND             ) => (0x0803, 0x00),
        (MANSION_2                  , 0x00, CINNABAR_ISLAND             ) => (0x3C01, 0x02),
        (MANSION_4                  , 0x00, CINNABAR_ISLAND             ) => (0x3E01, 0x00),

        //MANSION_2
        (MANSION_1                  , 0x04, CINNABAR_ISLAND             ) => (0x3B01, 0x03),
        (MANSION_3                  , 0x00, CINNABAR_ISLAND             ) => (0x3D01, 0x02),
        (MANSION_3                  , 0x02, CINNABAR_ISLAND             ) => (0x3D01, 0x01),
        (MANSION_3                  , 0x01, CINNABAR_ISLAND             ) => (0x3D01, 0x00),

        //MANSION_3
        (MANSION_2                  , 0x01, CINNABAR_ISLAND             ) => (0x3C01, 0x03),
        (MANSION_2                  , 0x03, CINNABAR_ISLAND             ) => (0x3C01, 0x00),
        (MANSION_2                  , 0x02, CINNABAR_ISLAND             ) => (0x3C01, 0x01),

        //MANSION_4
        (MANSION_1                  , 0x05, CINNABAR_ISLAND             ) => (0x3B01, 0x04),

        // SAFFRON_CITY
        (COPYCATS_HOUSE_1F          , 0x00, SAFFRON_CITY                ) => (0x000E, 0x01),
        (FIGHTING_DOJO              , 0x00, SAFFRON_CITY                ) => (0x020E, 0x01),
        (SAFFRON_GYM                , 0x00, SAFFRON_CITY                ) => (0x030E, 0x01),
        (SAFFRON_HOUSE_1            , 0x00, SAFFRON_CITY                ) => (0x040E, 0x01),
        (SAFFRON_MART               , 0x00, SAFFRON_CITY                ) => (0x050E, 0x01),
        (SILPH_CO_1F                , 0x00, SAFFRON_CITY                ) => (0x2F01, 0x01),
        (SAFFRON_POKECENTER         , 0x00, SAFFRON_CITY                ) => (0x060E, 0x01),
        (SAFFRON_HOUSE_2            , 0x00, SAFFRON_CITY                ) => (0x080E, 0x01),

        //SAFFRON_POKECENTER
        (LAST_MAP                   , 0x06, SAFFRON_CITY                ) => (0x0A03, 0x06),

        //SAFFRON_MART
        (LAST_MAP                   , 0x04, SAFFRON_CITY                ) => (0x0A03, 0x05),

        //SAFFRON_GYM
        (LAST_MAP                   , 0x02, SAFFRON_CITY                ) => (0x0A03, 0x03),

        (SAFFRON_GYM                , 0x16, SAFFRON_CITY                ) => (0x030E, 0x16),
        (SAFFRON_GYM                , 0x0F, SAFFRON_CITY                ) => (0x030E, 0x12),
        (SAFFRON_GYM                , 0x12, SAFFRON_CITY                ) => (0x030E, 0x14),
        (SAFFRON_GYM                , 0x08, SAFFRON_CITY                ) => (0x030E, 0x07),
        (SAFFRON_GYM                , 0x1B, SAFFRON_CITY                ) => (0x030E, 0x1E),
        (SAFFRON_GYM                , 0x10, SAFFRON_CITY                ) => (0x030E, 0x11),
        (SAFFRON_GYM                , 0x05, SAFFRON_CITY                ) => (0x030E, 0x0B),
        (SAFFRON_GYM                , 0x0D, SAFFRON_CITY                ) => (0x030E, 0x0F),
        (SAFFRON_GYM                , 0x17, SAFFRON_CITY                ) => (0x030E, 0x1C),
        (SAFFRON_GYM                , 0x1E, SAFFRON_CITY                ) => (0x030E, 0x1A),
        (SAFFRON_GYM                , 0x11, SAFFRON_CITY                ) => (0x030E, 0x13),
        (SAFFRON_GYM                , 0x09, SAFFRON_CITY                ) => (0x030E, 0x0D),
        (SAFFRON_GYM                , 0x1A, SAFFRON_CITY                ) => (0x030E, 0x18),
        (SAFFRON_GYM                , 0x03, SAFFRON_CITY                ) => (0x030E, 0x0C),
        (SAFFRON_GYM                , 0x07, SAFFRON_CITY                ) => (0x030E, 0x0A),
        (SAFFRON_GYM                , 0x0C, SAFFRON_CITY                ) => (0x030E, 0x09),
        (SAFFRON_GYM                , 0x04, SAFFRON_CITY                ) => (0x030E, 0x05),
        (SAFFRON_GYM                , 0x1F, SAFFRON_CITY                ) => (0x030E, 0x20),
        (SAFFRON_GYM                , 0x18, SAFFRON_CITY                ) => (0x030E, 0x17),
        (SAFFRON_GYM                , 0x1C, SAFFRON_CITY                ) => (0x030E, 0x19),
        (SAFFRON_GYM                , 0x02, SAFFRON_CITY                ) => (0x030E, 0x04),
        (SAFFRON_GYM                , 0x0A, SAFFRON_CITY                ) => (0x030E, 0x08),
        (SAFFRON_GYM                , 0x14, SAFFRON_CITY                ) => (0x030E, 0x15),
        (SAFFRON_GYM                , 0x1D, SAFFRON_CITY                ) => (0x030E, 0x1F),
        (SAFFRON_GYM                , 0x0E, SAFFRON_CITY                ) => (0x030E, 0x10),
        (SAFFRON_GYM                , 0x06, SAFFRON_CITY                ) => (0x030E, 0x06),
        (SAFFRON_GYM                , 0x15, SAFFRON_CITY                ) => (0x030E, 0x1B),
        (SAFFRON_GYM                , 0x19, SAFFRON_CITY                ) => (0x030E, 0x1D),
        (SAFFRON_GYM                , 0x0B, SAFFRON_CITY                ) => (0x030E, 0x0E),
        (SAFFRON_GYM                , 0x13, SAFFRON_CITY                ) => (0x030E, 0x03),

        //SAFFRON_HOUSE_1
        (LAST_MAP                   , 0x03, SAFFRON_CITY                ) => (0x0A03, 0x04),

        //SAFFRON_HOUSE_2
        (LAST_MAP                   , 0x07, SAFFRON_CITY                ) => (0x0A03, 0x07),

        //COPYCATS_HOUSE_1F
        (LAST_MAP                   , 0x00, SAFFRON_CITY                ) => (0x0A03, 0x01),
        (COPYCATS_HOUSE_2F          , 0x00, SAFFRON_CITY                ) => (0x010E, 0x00),

        //COPYCATS_HOUSE_2F
        (COPYCATS_HOUSE_1F          , 0x02, SAFFRON_CITY                ) => (0x000E, 0x03),

        //FIGHTING_DOJO
        (LAST_MAP                   , 0x01, SAFFRON_CITY                ) => (0x0A03, 0x02),

        //SILPH_CO_1F
        (LAST_MAP                   , 0x05, SAFFRON_CITY                ) => (0x0A03, 0x00),
        (SILPH_CO_2F                , 0x00, SAFFRON_CITY                ) => (0x3001, 0x03),

        //SILPH_CO_2F
        (SILPH_CO_1F                , 0x02, SAFFRON_CITY                ) => (0x2F01, 0x03),
        (SILPH_CO_3F                , 0x00, SAFFRON_CITY                ) => (0x3101, 0x00),
        (SILPH_CO_3F                , 0x06, SAFFRON_CITY                ) => (0x3101, 0x08),
        (SILPH_CO_8F                , 0x04, SAFFRON_CITY                ) => (0x3601, 0x05),
        (SILPH_CO_8F                , 0x05, SAFFRON_CITY                ) => (0x3601, 0x02),
        (SILPH_CO_6F                , 0x04, SAFFRON_CITY                ) => (0x3401, 0x01),

        //SILPH_CO_3F
        (SILPH_CO_2F                , 0x01, SAFFRON_CITY                ) => (0x3001, 0x00),
        (SILPH_CO_4F                , 0x00, SAFFRON_CITY                ) => (0x3201, 0x02),
        (SILPH_CO_3F                , 0x09, SAFFRON_CITY                ) => (0x3101, 0x07),
        (SILPH_CO_5F                , 0x05, SAFFRON_CITY                ) => (0x3301, 0x01),
        (SILPH_CO_5F                , 0x06, SAFFRON_CITY                ) => (0x3301, 0x04),
        (SILPH_CO_2F                , 0x03, SAFFRON_CITY                ) => (0x3001, 0x01),
        (SILPH_CO_9F                , 0x03, SAFFRON_CITY                ) => (0x3701, 0x02),
        (SILPH_CO_7F                , 0x04, SAFFRON_CITY                ) => (0x3501, 0x04),
        (SILPH_CO_3F                , 0x03, SAFFRON_CITY                ) => (0x3101, 0x06),

        //SILPH_CO_4F
        (SILPH_CO_3F                , 0x01, SAFFRON_CITY                ) => (0x3101, 0x03),
        (SILPH_CO_5F                , 0x01, SAFFRON_CITY                ) => (0x3301, 0x00),
        (SILPH_CO_10F               , 0x03, SAFFRON_CITY                ) => (0x3801, 0x01),
        (SILPH_CO_6F                , 0x03, SAFFRON_CITY                ) => (0x3401, 0x03),
        (SILPH_CO_10F               , 0x04, SAFFRON_CITY                ) => (0x3801, 0x04),
        (SILPH_CO_10F               , 0x05, SAFFRON_CITY                ) => (0x3801, 0x02),

        //SILPH_CO_5F
        (SILPH_CO_6F                , 0x01, SAFFRON_CITY                ) => (0x3401, 0x02),
        (SILPH_CO_4F                , 0x01, SAFFRON_CITY                ) => (0x3201, 0x00),
        (SILPH_CO_7F                , 0x05, SAFFRON_CITY                ) => (0x3501, 0x02),
        (SILPH_CO_9F                , 0x04, SAFFRON_CITY                ) => (0x3701, 0x01),
        (SILPH_CO_3F                , 0x04, SAFFRON_CITY                ) => (0x3101, 0x01),
        (SILPH_CO_3F                , 0x05, SAFFRON_CITY                ) => (0x3101, 0x05),

        //SILPH_CO_6F
        (SILPH_CO_7F                , 0x01, SAFFRON_CITY                ) => (0x3501, 0x00),
        (SILPH_CO_5F                , 0x00, SAFFRON_CITY                ) => (0x3301, 0x02),
        (SILPH_CO_4F                , 0x04, SAFFRON_CITY                ) => (0x3201, 0x01),
        (SILPH_CO_2F                , 0x06, SAFFRON_CITY                ) => (0x3001, 0x04),

        //SILPH_CO_7F
        (SILPH_CO_8F                , 0x01, SAFFRON_CITY                ) => (0x3601, 0x03),
        (SILPH_CO_6F                , 0x00, SAFFRON_CITY                ) => (0x3401, 0x00),
        (SILPH_CO_11F               , 0x03, SAFFRON_CITY                ) => (0x3901, 0x01),
        (SILPH_CO_3F                , 0x08, SAFFRON_CITY                ) => (0x3101, 0x02),
        (SILPH_CO_5F                , 0x03, SAFFRON_CITY                ) => (0x3301, 0x05),

        //SILPH_CO_8F
        (SILPH_CO_9F                , 0x01, SAFFRON_CITY                ) => (0x3701, 0x00),
        (SILPH_CO_7F                , 0x00, SAFFRON_CITY                ) => (0x3501, 0x03),
        (SILPH_CO_8F                , 0x06, SAFFRON_CITY                ) => (0x3601, 0x01),
        (SILPH_CO_2F                , 0x04, SAFFRON_CITY                ) => (0x3001, 0x02),
        (SILPH_CO_2F                , 0x05, SAFFRON_CITY                ) => (0x3001, 0x05),
        (SILPH_CO_8F                , 0x03, SAFFRON_CITY                ) => (0x3601, 0x04),
        
        //SILPH_CO_9F
        (SILPH_CO_10F               , 0x00, SAFFRON_CITY                ) => (0x3801, 0x03),
        (SILPH_CO_8F                , 0x00, SAFFRON_CITY                ) => (0x3601, 0x00),
        (SILPH_CO_3F                , 0x07, SAFFRON_CITY                ) => (0x3101, 0x04),
        (SILPH_CO_5F                , 0x04, SAFFRON_CITY                ) => (0x3301, 0x03),

        //SILPH_CO_10F
        (SILPH_CO_9F                , 0x00, SAFFRON_CITY                ) => (0x3701, 0x03),
        (SILPH_CO_11F               , 0x00, SAFFRON_CITY                ) => (0x3901, 0x00),
        (SILPH_CO_4F                , 0x03, SAFFRON_CITY                ) => (0x3201, 0x03),
        (SILPH_CO_4F                , 0x05, SAFFRON_CITY                ) => (0x3201, 0x04),
        (SILPH_CO_4F                , 0x06, SAFFRON_CITY                ) => (0x3201, 0x05),

        //SILPH_CO_11F
        (SILPH_CO_10F               , 0x01, SAFFRON_CITY                ) => (0x3801, 0x00),
        (SILPH_CO_7F                , 0x03, SAFFRON_CITY                ) => (0x3501, 0x01),

        //INDIGO_PLATEAU
        (INDIGO_PLATEAU_LOBBY       , 0x00, INDIGO_PLATEAU              ) => (0x000D, 0x00),

        //INDIGO_PLATEAU_LOBBY
        (LAST_MAP                   , 0x00, INDIGO_PLATEAU              ) => (0x0903, 0x00),
        (LAST_MAP                   , 0x01, INDIGO_PLATEAU              ) => (0x0903, 0x00),
        (LORELEIS_ROOM              , 0x00, INDIGO_PLATEAU              ) => (0x4B01, 0x00),

        //LORELEIS_ROOM
        (INDIGO_PLATEAU_LOBBY       , 0x02, INDIGO_PLATEAU              ) => (0x000D, 0x01),
        (BRUNOS_ROOM                , 0x00, INDIGO_PLATEAU              ) => (0x4C01, 0x00),
        (BRUNOS_ROOM                , 0x01, INDIGO_PLATEAU              ) => (0x4C01, 0x00),

        //BRUNOS_ROOM
        (LORELEIS_ROOM              , 0x02, INDIGO_PLATEAU              ) => (0x4B01, 0x01),
        (LORELEIS_ROOM              , 0x03, INDIGO_PLATEAU              ) => (0x4B01, 0x01),
        (AGATHAS_ROOM               , 0x00, INDIGO_PLATEAU              ) => (0x4D01, 0x00),
        (AGATHAS_ROOM               , 0x01, INDIGO_PLATEAU              ) => (0x4D01, 0x00),

        //AGATHAS_ROOM
        (BRUNOS_ROOM                , 0x02, INDIGO_PLATEAU              ) => (0x4C01, 0x01),
        (BRUNOS_ROOM                , 0x03, INDIGO_PLATEAU              ) => (0x4C01, 0x01),
        (LANCES_ROOM                , 0x00, INDIGO_PLATEAU              ) => (0x4E01, 0x00),

        //LANCES_ROOM
        (AGATHAS_ROOM               , 0x02, INDIGO_PLATEAU              ) => (0x4D01, 0x01),
        (CHAMPIONS_ROOM             , 0x00, INDIGO_PLATEAU              ) => (0x4F01, 0x00),

        //CHAMPIONS_ROOM
        (LANCES_ROOM                , 0x01, INDIGO_PLATEAU              ) => (0x4E01, 0x01),
        (LANCES_ROOM                , 0x02, INDIGO_PLATEAU              ) => (0x4E01, 0x01),
        (HALL_OF_FAME               , 0x00, INDIGO_PLATEAU              ) => (0x5001, 0x00),

        //HALL_OF_FAME
        (CHAMPIONS_ROOM             , 0x02, INDIGO_PLATEAU              ) => (0x4F01, 0x01),
        (CHAMPIONS_ROOM             , 0x03, INDIGO_PLATEAU              ) => (0x4F01, 0x01),

        //ROUTE_2
        (DIGLETTS_CAVE_EXIT         , 0x00, ROUTE_2                     ) => (0x2401, 0x01),
        (VIRIDIAN_FOREST_EXIT       , 0x01, ROUTE_2                     ) => (0x030F, 0x03),
        (ROUTE_2_HOUSE              , 0x00, ROUTE_2                     ) => (0x010F, 0x01),
        (ROUTE_2_GATE               , 0x01, ROUTE_2                     ) => (0x020F, 0x03),
        (ROUTE_2_GATE               , 0x02, ROUTE_2                     ) => (0x020F, 0x01),
        (VIRIDIAN_FOREST_ENTRANCE   , 0x02, ROUTE_2                     ) => (0x000F, 0x01),

        //DIGLETTS_CAVE_EXIT
        (LAST_MAP                   , 0x00, ROUTE_2                     ) => (0x1403, 0x03),
        (DIGLETTS_CAVE              , 0x00, ROUTE_2                     ) => (0x2501, 0x00),

        //VIRIDIAN_FOREST_EXIT
        (LAST_MAP                   , 0x01, ROUTE_2                     ) => (0x1403, 0x00),
        (VIRIDIAN_FOREST            , 0x00, ROUTE_2                     ) => (0x0001, 0x02),

        //ROUTE_2_HOUSE
        (LAST_MAP                   , 0x02, ROUTE_2                     ) => (0x1403, 0x04),

        //ROUTE_2_GATE
        (LAST_MAP                   , 0x03, ROUTE_2                     ) => (0x1403, 0x06),
        (LAST_MAP                   , 0x04, ROUTE_2                     ) => (0x1403, 0x05),

        //VIRIDIAN_FOREST_ENTRANCE
        (VIRIDIAN_FOREST            , 0x03, ROUTE_2                     ) => (0x0001, 0x00),
        (VIRIDIAN_FOREST            , 0x04, ROUTE_2                     ) => (0x0001, 0x00),
        (LAST_MAP                   , 0x05, ROUTE_2                     ) => (0x1403, 0x02),
        
        //VIRIDIAN_FOREST
        (VIRIDIAN_FOREST_EXIT       , 0x02, ROUTE_2                     ) => (0x030F, 0x03),
        (VIRIDIAN_FOREST_EXIT       , 0x03, ROUTE_2                     ) => (0x030F, 0x03),
        (VIRIDIAN_FOREST_ENTRANCE   , 0x01, ROUTE_2                     ) => (0x000F, 0x01),

        //ROUTE_4
        (MT_MOON_POKECENTER         , 0x01, ROUTE_4                     ) => (0x0110, 0x01),
        (MT_MOON_1                  , 0x01, ROUTE_4                     ) => (0x0101, 0x03),
        (MT_MOON_2                  , 0x07, ROUTE_4                     ) => (0x0201, 0x07),

        //MT_MOON_POKECENTER
        (LAST_MAP                   , 0x00, ROUTE_4                     ) => (0x1603, 0x02),

        //MT_MOON_1
        (LAST_MAP                   , 0x01, ROUTE_4                     ) => (0x1603, 0x00),
        (MT_MOON_2                  , 0x00, ROUTE_4                     ) => (0x0201, 0x00),
        (MT_MOON_2                  , 0x02, ROUTE_4                     ) => (0x0201, 0x01),
        (MT_MOON_2                  , 0x03, ROUTE_4                     ) => (0x0201, 0x02),

        //MT_MOON_2
        (MT_MOON_1                  , 0x02, ROUTE_4                     ) => (0x0101, 0x00),
        (MT_MOON_3                  , 0x00, ROUTE_4                     ) => (0x0301, 0x01),
        (MT_MOON_1                  , 0x03, ROUTE_4                     ) => (0x0101, 0x01),
        (MT_MOON_1                  , 0x04, ROUTE_4                     ) => (0x0101, 0x02),
        (MT_MOON_3                  , 0x01, ROUTE_4                     ) => (0x0301, 0x00),
        (MT_MOON_3                  , 0x02, ROUTE_4                     ) => (0x0301, 0x02),
        (MT_MOON_3                  , 0x03, ROUTE_4                     ) => (0x0301, 0x03),
        (LAST_MAP                   , 0x02, ROUTE_4                     ) => (0x1603, 0x01),

        //MT_MOON_3
        (MT_MOON_2                  , 0x01, ROUTE_4                     ) => (0x0201, 0x04),
        (MT_MOON_2                  , 0x04, ROUTE_4                     ) => (0x0201, 0x03),
        (MT_MOON_2                  , 0x05, ROUTE_4                     ) => (0x0201, 0x05),
        (MT_MOON_2                  , 0x06, ROUTE_4                     ) => (0x0201, 0x06),

        //ROUTE_5
        (ROUTE_5_GATE               , 0x03, ROUTE_5                     ) => (0x0111, 0x01),
        (ROUTE_5_GATE               , 0x02, ROUTE_5                     ) => (0x0111, 0x01),
        (ROUTE_5_GATE               , 0x00, ROUTE_5                     ) => (0x0111, 0x02),
        (PATH_ENTRANCE_ROUTE_5      , 0x00, ROUTE_5                     ) => (0x1E01, 0x01),
        (DAYCAREM                   , 0x00, ROUTE_5                     ) => (0x0011, 0x01),

        //ROUTE_5_GATE
        (LAST_MAP                   , 0x02, ROUTE_5                     ) => (0x0A03, 0x09),
        (LAST_MAP                   , 0x01, ROUTE_5                     ) => (0x1703, 0x02),
        (LAST_MAP                   , 0x00, ROUTE_5                     ) => (0x1703, 0x02),

        //PATH_ENTRANCE_ROUTE_5
        (LAST_MAP                   , 0x03, ROUTE_5                     ) => (0x1703, 0x00),
        (UNDERGROUND_PATH_NS        , 0x00, ROUTE_5                     ) => (0x1F01, 0x00),

        //DAYCAREM
        (LAST_MAP                   , 0x04, ROUTE_5                     ) => (0x1703, 0x01),
        
        //UNDERGROUND_PATH_NS
        (PATH_ENTRANCE_ROUTE_5      , 0x02, ROUTE_5                     ) => (0x1E01, 0x03),
        (PATH_ENTRANCE_ROUTE_6      , 0x02, ROUTE_6                     ) => (0x2001, 0x03),

        //ROUTE_6
        (ROUTE_6_GATE               , 0x02, ROUTE_6                     ) => (0x0012, 0x00),
        (ROUTE_6_GATE               , 0x02, ROUTE_6                     ) => (0x0012, 0x00),
        (ROUTE_6_GATE               , 0x00, ROUTE_6                     ) => (0x0012, 0x02),
        (PATH_ENTRANCE_ROUTE_6      , 0x00, ROUTE_6                     ) => (0x2001, 0x01),

        //ROUTE_6_GATE
        (LAST_MAP                   , 0x02, ROUTE_6                     ) => (0x1803, 0x01),
        (LAST_MAP                   , 0x01, ROUTE_6                     ) => (0x0A03, 0x0B),

        //PATH_ENTRANCE_ROUTE_6
        (LAST_MAP                   , 0x03, ROUTE_6                     ) => (0x1803, 0x00),
        (UNDERGROUND_PATH_NS        , 0x01, ROUTE_6                     ) => (0x1F01, 0x01),

        //ROUTE_7
        (ROUTE_7_GATE               , 0x02, ROUTE_7                     ) => (0x0013, 0x02),
        (ROUTE_7_GATE               , 0x03, ROUTE_7                     ) => (0x0013, 0x02),
        (ROUTE_7_GATE               , 0x00, ROUTE_7                     ) => (0x0013, 0x00),
        (ROUTE_7_GATE               , 0x01, ROUTE_7                     ) => (0x0013, 0x00),
        (PATH_ENTRANCE_ROUTE_7      , 0x00, ROUTE_7                     ) => (0x2101, 0x01),

        //ROUTE_7_GATE
        (LAST_MAP                   , 0x03, ROUTE_7                     ) => (0x0A03, 0x08),
        (LAST_MAP                   , 0x00, ROUTE_7                     ) => (0x1903, 0x01),
        (LAST_MAP                   , 0x01, ROUTE_7                     ) => (0x1903, 0x01),

        //PATH_ENTRANCE_ROUTE_7
        (LAST_MAP                   , 0x04, ROUTE_7                     ) => (0x1903, 0x00),
        (UNDERGROUND_PATH_WE        , 0x00, ROUTE_7                     ) => (0x2201, 0x01),

        //UNDERGROUND_PATH_WE
        (PATH_ENTRANCE_ROUTE_7      , 0x02, ROUTE_7                     ) => (0x2101, 0x03),
        (PATH_ENTRANCE_ROUTE_8      , 0x02, ROUTE_8                     ) => (0x2301, 0x03),

        //ROUTE_8
        (ROUTE_8_GATE               , 0x00, ROUTE_8                     ) => (0x0014, 0x01),
        (ROUTE_8_GATE               , 0x01, ROUTE_8                     ) => (0x0014, 0x01),
        (ROUTE_8_GATE               , 0x02, ROUTE_8                     ) => (0x0014, 0x02),
        (ROUTE_8_GATE               , 0x03, ROUTE_8                     ) => (0x0014, 0x02),
        (PATH_ENTRANCE_ROUTE_8      , 0x00, ROUTE_8                     ) => (0x2301, 0x01),

        //ROUTE_8_GATE
        (LAST_MAP                   , 0x00, ROUTE_8                     ) => (0x0A03, 0x0A),
        (LAST_MAP                   , 0x01, ROUTE_8                     ) => (0x0A03, 0x0A),
        (LAST_MAP                   , 0x02, ROUTE_8                     ) => (0x1A03, 0x01),
        (LAST_MAP                   , 0x03, ROUTE_8                     ) => (0x1A03, 0x01),

        //PATH_ENTRANCE_ROUTE_8
        (LAST_MAP                   , 0x04, ROUTE_8                     ) => (0x1A03, 0x00),
        (UNDERGROUND_PATH_WE        , 0x01, ROUTE_8                     ) => (0x2201, 0x00),

        //ROUTE_10
        (ROCK_TUNNEL_POKECENTER     , 0x00, ROUTE_10                    ) => (0x0015, 0x01),
        (ROCK_TUNNEL_1              , 0x00, ROUTE_10                    ) => (0x5101, 0x00),
        (ROCK_TUNNEL_1              , 0x02, ROUTE_10                    ) => (0x5101, 0x05),
        (POWER_PLANT                , 0x00, ROUTE_10                    ) => (0x5F01, 0x01),

        //ROCK_TUNNEL_POKECENTER
        (LAST_MAP                   , 0x00, ROUTE_10                    ) => (0x1C03, 0x03),

        //ROCK_TUNNEL_1
        (LAST_MAP                   , 0x01, ROUTE_10                    ) => (0x1C03, 0x00),
        (LAST_MAP                   , 0x02, ROUTE_10                    ) => (0x1C03, 0x01),
        (ROCK_TUNNEL_2              , 0x00, ROUTE_10                    ) => (0x5201, 0x00),
        (ROCK_TUNNEL_2              , 0x01, ROUTE_10                    ) => (0x5201, 0x01),
        (ROCK_TUNNEL_2              , 0x02, ROUTE_10                    ) => (0x5201, 0x02),
        (ROCK_TUNNEL_2              , 0x03, ROUTE_10                    ) => (0x5201, 0x03),
        
        //ROCK_TUNNEL_2
        (ROCK_TUNNEL_1              , 0x04, ROUTE_10                    ) => (0x5101, 0x01),
        (ROCK_TUNNEL_1              , 0x05, ROUTE_10                    ) => (0x5101, 0x02),
        (ROCK_TUNNEL_1              , 0x06, ROUTE_10                    ) => (0x5101, 0x03),
        (ROCK_TUNNEL_1              , 0x07, ROUTE_10                    ) => (0x5101, 0x04),

        //POWER_PLANT
        (LAST_MAP                   , 0x03, ROUTE_10                    ) => (0x1C03, 0x01),

        //ROUTE_11
        (ROUTE_11_GATE_1F           , 0x00, ROUTE_11                    ) => (0x0016, 0x00),
        (ROUTE_11_GATE_1F           , 0x01, ROUTE_11                    ) => (0x0016, 0x00),
        (ROUTE_11_GATE_1F           , 0x02, ROUTE_11                    ) => (0x0016, 0x02),
        (ROUTE_11_GATE_1F           , 0x03, ROUTE_11                    ) => (0x0016, 0x02),
        (DIGLETTS_CAVE_ENTRANCE     , 0x00, ROUTE_11                    ) => (0x2601, 0x00),

        //ROUTE_11_GATE_1F
        (LAST_MAP                   , 0x00, ROUTE_11                    ) => (0x1D03, 0x01),
        (LAST_MAP                   , 0x01, ROUTE_11                    ) => (0x1D03, 0x01),
        (LAST_MAP                   , 0x02, ROUTE_11                    ) => (0x1D03, 0x02),
        (LAST_MAP                   , 0x03, ROUTE_11                    ) => (0x1D03, 0x02),
        (ROUTE_11_GATE_2F           , 0x00, ROUTE_11                    ) => (0x0116, 0x00),

        //DIGLETTS_CAVE_ENTRANCE
        (LAST_MAP                   , 0x04, ROUTE_11                    ) => (0x1D03, 0x00),
        (DIGLETTS_CAVE              , 0x01, ROUTE_11                    ) => (0x2501, 0x01),

        //ROUTE_11_GATE_2F
        (ROUTE_11_GATE_1F           , 0x04, ROUTE_11                    ) => (0x0016, 0x04),
        
        //DIGLETTS_CAVE
        (DIGLETTS_CAVE_EXIT         , 0x02, ROUTE_11                    ) => (0x2401, 0x00),
        (DIGLETTS_CAVE_ENTRANCE     , 0x02, ROUTE_11                    ) => (0x2601, 0x01),

        //ROUTE_12
        (ROUTE_12_GATE_1F           , 0x00, ROUTE_12                    ) => (0x0017, 0x00),
        (ROUTE_12_GATE_1F           , 0x01, ROUTE_12                    ) => (0x0017, 0x00),
        (ROUTE_12_GATE_1F           , 0x02, ROUTE_12                    ) => (0x0017, 0x02),
        (ROUTE_12_HOUSE             , 0x00, ROUTE_12                    ) => (0x0217, 0x01),

        //ROUTE_12_GATE_1F
        (LAST_MAP                   , 0x00, ROUTE_12                    ) => (0x1E03, 0x01),
        (LAST_MAP                   , 0x01, ROUTE_12                    ) => (0x1E03, 0x01),
        (LAST_MAP                   , 0x02, ROUTE_12                    ) => (0x1E03, 0x03),
        (ROUTE_12_GATE_2F           , 0x00, ROUTE_12                    ) => (0x0117, 0x00),
        
        //ROUTE_12_HOUSE
        (LAST_MAP                   , 0x03, ROUTE_12                    ) => (0x1E03, 0x01),
        
        //ROUTE_12_GATE_2F
        (ROUTE_12_GATE_1F           , 0x04, ROUTE_12                    ) => (0x0017, 0x04),

        //ROUTE_15
        (ROUTE_15_GATE_1F           , 0x00, ROUTE_15                    ) => (0x0018, 0x00),
        (ROUTE_15_GATE_1F           , 0x01, ROUTE_15                    ) => (0x0018, 0x00),
        (ROUTE_15_GATE_1F           , 0x02, ROUTE_15                    ) => (0x0018, 0x02),
        (ROUTE_15_GATE_1F           , 0x03, ROUTE_15                    ) => (0x0018, 0x02),

        //ROUTE_15_GATE_1F
        (LAST_MAP                   , 0x00, ROUTE_15                    ) => (0x2103, 0x00),
        (LAST_MAP                   , 0x01, ROUTE_15                    ) => (0x2103, 0x00),
        (LAST_MAP                   , 0x02, ROUTE_15                    ) => (0x2103, 0x01),
        (LAST_MAP                   , 0x03, ROUTE_15                    ) => (0x2103, 0x01),
        (ROUTE_15_GATE_2F           , 0x00, ROUTE_15                    ) => (0x0118, 0x00),

        //ROUTE_15_GATE_2F
        (ROUTE_15_GATE_1F           , 0x04, ROUTE_15                    ) => (0x0018, 0x04),

        //ROUTE_16
        (ROUTE_16_GATE_1F           , 0x00, ROUTE_16                    ) => (0x0119, 0x02),
        (ROUTE_16_GATE_1F           , 0x01, ROUTE_16                    ) => (0x0119, 0x02),
        (ROUTE_16_GATE_1F           , 0x02, ROUTE_16                    ) => (0x0119, 0x03),
        (ROUTE_16_GATE_1F           , 0x03, ROUTE_16                    ) => (0x0119, 0x03),
        (ROUTE_16_GATE_1F           , 0x04, ROUTE_16                    ) => (0x0119, 0x00),
        (ROUTE_16_GATE_1F           , 0x05, ROUTE_16                    ) => (0x0119, 0x00),
        (ROUTE_16_GATE_1F           , 0x06, ROUTE_16                    ) => (0x0119, 0x01),
        (ROUTE_16_GATE_1F           , 0x07, ROUTE_16                    ) => (0x0119, 0x01),
        (ROUTE_16_HOUSE             , 0x00, ROUTE_16                    ) => (0x0019, 0x01),

        //ROUTE_16_GATE_1F
        (LAST_MAP                   , 0x00, ROUTE_16                    ) => (0x2203, 0x03),
        (LAST_MAP                   , 0x01, ROUTE_16                    ) => (0x2203, 0x03),
        (LAST_MAP                   , 0x02, ROUTE_16                    ) => (0x2203, 0x04),
        (LAST_MAP                   , 0x04, ROUTE_16                    ) => (0x2203, 0x01),
        (LAST_MAP                   , 0x05, ROUTE_16                    ) => (0x2203, 0x01),
        (LAST_MAP                   , 0x06, ROUTE_16                    ) => (0x2203, 0x02),
        (LAST_MAP                   , 0x07, ROUTE_16                    ) => (0x2203, 0x02),
        (ROUTE_16_GATE_2F           , 0x00, ROUTE_16                    ) => (0x0219, 0x00),

        //ROUTE_16_GATE_2F
        (ROUTE_16_GATE_1F           , 0x08, ROUTE_16                    ) => (0x0119, 0x04),

        //ROUTE_16_HOUSE
        (LAST_MAP                   , 0x08, ROUTE_16                    ) => (0x2203, 0x00),

        //ROUTE_18
        (ROUTE_18_GATE_1F           , 0x00, ROUTE_18                    ) => (0x001A, 0x00),
        (ROUTE_18_GATE_1F           , 0x01, ROUTE_18                    ) => (0x001A, 0x00),
        (ROUTE_18_GATE_1F           , 0x02, ROUTE_18                    ) => (0x001A, 0x01),
        (ROUTE_18_GATE_1F           , 0x03, ROUTE_18                    ) => (0x001A, 0x01),

        //ROUTE_18_GATE_1F
        (LAST_MAP                   , 0x00, ROUTE_18                    ) => (0x2403, 0x00),
        (LAST_MAP                   , 0x01, ROUTE_18                    ) => (0x2403, 0x00),
        (LAST_MAP                   , 0x02, ROUTE_18                    ) => (0x2403, 0x01),
        (LAST_MAP                   , 0x03, ROUTE_18                    ) => (0x2403, 0x01),
        (ROUTE_18_GATE_2F           , 0x00, ROUTE_18                    ) => (0x011A, 0x00),

        //ROUTE_18_GATE_2F
        (ROUTE_18_GATE_1F           , 0x04, ROUTE_18                    ) => (0x001A, 0x02),

        //ROUTE_20
        (SEAFOAM_ISLANDS_1          , 0x00, ROUTE_20                    ) => (0x5301, 0x03),
        (SEAFOAM_ISLANDS_1          , 0x02, ROUTE_20                    ) => (0x5301, 0x04),

        //SEAFOAM_ISLANDS_1
        (LAST_MAP                   , 0x00, ROUTE_20                    ) => (0x2603, 0x00),
        (LAST_MAP                   , 0x01, ROUTE_20                    ) => (0x2603, 0x01),
        (SEAFOAM_ISLANDS_2          , 0x01, ROUTE_20                    ) => (0x5401, 0x00),
        (SEAFOAM_ISLANDS_2          , 0x06, ROUTE_20                    ) => (0x5401, 0x01),
        (SEAFOAM_ISLANDS_2          , 0x04, ROUTE_20                    ) => (0x5401, 0x02),
        
        //SEAFOAM_ISLANDS_2
        (SEAFOAM_ISLANDS_3          , 0x00, ROUTE_20                    ) => (0x5501, 0x03),
        (SEAFOAM_ISLANDS_1          , 0x04, ROUTE_20                    ) => (0x5301, 0x00),
        (SEAFOAM_ISLANDS_3          , 0x02, ROUTE_20                    ) => (0x5501, 0x04),
        (SEAFOAM_ISLANDS_3          , 0x03, ROUTE_20                    ) => (0x5501, 0x05),
        (SEAFOAM_ISLANDS_1          , 0x06, ROUTE_20                    ) => (0x5301, 0x02),
        (SEAFOAM_ISLANDS_3          , 0x05, ROUTE_20                    ) => (0x5501, 0x06),
        (SEAFOAM_ISLANDS_1          , 0x05, ROUTE_20                    ) => (0x5301, 0x01),

        //SEAFOAM_ISLANDS_3
        (SEAFOAM_ISLANDS_2          , 0x00, ROUTE_20                    ) => (0x5401, 0x03),
        (SEAFOAM_ISLANDS_4          , 0x00, ROUTE_20                    ) => (0x5601, 0x00),
        (SEAFOAM_ISLANDS_2          , 0x02, ROUTE_20                    ) => (0x5401, 0x04),
        (SEAFOAM_ISLANDS_2          , 0x03, ROUTE_20                    ) => (0x5401, 0x05),
        (SEAFOAM_ISLANDS_4          , 0x03, ROUTE_20                    ) => (0x5601, 0x01),
        (SEAFOAM_ISLANDS_2          , 0x05, ROUTE_20                    ) => (0x5401, 0x06),
        (SEAFOAM_ISLANDS_4          , 0x04, ROUTE_20                    ) => (0x5601, 0x02),

        //SEAFOAM_ISLANDS_4
        (SEAFOAM_ISLANDS_3          , 0x01, ROUTE_20                    ) => (0x5501, 0x00),
        (SEAFOAM_ISLANDS_5          , 0x02, ROUTE_20                    ) => (0x5701, 0x00),
        (SEAFOAM_ISLANDS_5          , 0x03, ROUTE_20                    ) => (0x5701, 0x01),
        (SEAFOAM_ISLANDS_3          , 0x04, ROUTE_20                    ) => (0x5501, 0x01),
        (SEAFOAM_ISLANDS_3          , 0x06, ROUTE_20                    ) => (0x5501, 0x02),

        //SEAFOAM_ISLANDS_5
        (SEAFOAM_ISLANDS_4          , 0x01, ROUTE_20                    ) => (0x5601, 0x03),
        (SEAFOAM_ISLANDS_4          , 0x02, ROUTE_20                    ) => (0x5601, 0x04),

        //ROUTE_22
        (ROUTE_22_GATE              , 0x00, ROUTE_22                    ) => (0x001C, 0x02),

        //ROUTE_22_GATE
        (LAST_MAP                   , 0x00, ROUTE_22                    ) => (0x2903, 0x00),
        (LAST_MAP                   , 0x01, ROUTE_23                    ) => (0x2A03, 0x02),

        //ROUTE_23
        (ROUTE_22_GATE              , 0x02, ROUTE_23                    ) => (0x001C, 0x00),
        (ROUTE_22_GATE              , 0x03, ROUTE_23                    ) => (0x001C, 0x00),
        (VICTORY_ROAD_1             , 0x00, ROUTE_23                    ) => (0x2701, 0x01),
        (VICTORY_ROAD_2             , 0x01, ROUTE_23                    ) => (0x2801, 0x06),

        //VICTORY_ROAD_1
        (LAST_MAP                   , 0x02, ROUTE_23                    ) => (0x2A03, 0x00),
        (VICTORY_ROAD_2             , 0x00, ROUTE_23                    ) => (0x2801, 0x00),

        //VICTORY_ROAD_2
        (VICTORY_ROAD_1             , 0x02, ROUTE_23                    ) => (0x2701, 0x00),
        (LAST_MAP                   , 0x03, ROUTE_23                    ) => (0x2A03, 0x01),
        (VICTORY_ROAD_3             , 0x00, ROUTE_23                    ) => (0x2901, 0x01),
        (VICTORY_ROAD_3             , 0x02, ROUTE_23                    ) => (0x2901, 0x03),
        (VICTORY_ROAD_3             , 0x01, ROUTE_23                    ) => (0x2901, 0x02),
        (VICTORY_ROAD_3             , 0x03, ROUTE_23                    ) => (0x2901, 0x00),

        //VICTORY_ROAD_3
        (VICTORY_ROAD_2             , 0x03, ROUTE_23                    ) => (0x2801, 0x02),
        (VICTORY_ROAD_2             , 0x05, ROUTE_23                    ) => (0x2801, 0x03),
        (VICTORY_ROAD_2             , 0x04, ROUTE_23                    ) => (0x2801, 0x04),
        (VICTORY_ROAD_2             , 0x06, ROUTE_23                    ) => (0x2801, 0x01),

        //ROUTE_25
        (BILLS_HOUSE                , 0x01, ROUTE_25                    ) => (0x001E, 0x01),      

        //BILLS_HOUSE
        (LAST_MAP                   , 0x00, ROUTE_25                    ) => (0x2C03, 0x00)
    )
}