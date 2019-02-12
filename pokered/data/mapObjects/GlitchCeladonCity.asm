GlitchCeladonCity_Object:
	db $f ; border block

	db 13 ; warps
	warp 10, 27, 0, CELADON_MART_1F
	warp 10, 13, 2, CELADON_MART_1F
	warp 24, 9, 0, CELADON_MANSION_1F
	warp 24, 3, 2, CELADON_MANSION_1F
	warp 25, 3, 2, CELADON_MANSION_1F
	warp 41, 9, 0, CELADON_POKECENTER
	warp 12, 27, 0, CELADON_GYM
	warp 28, 19, 0, GAME_CORNER
	warp 39, 19, 0, CELADON_MART_5F ; beta warp! no longer used
	warp 33, 19, 0, GAME_CORNER_PRIZE_ROOM
	warp 31, 27, 0, CELADON_DINER
	warp 35, 27, 0, CELADON_CHIEF_HOUSE
	warp 43, 27, 0, CELADON_HOTEL

	db 0 ; signs

	db 0 ; objects

	; warp-to
	warp_to 10, 27, CELADON_CITY_WIDTH ; CELADON_MART_1F
	warp_to 10, 13, CELADON_CITY_WIDTH ; CELADON_MART_1F
	warp_to 24, 9, CELADON_CITY_WIDTH ; CELADON_MANSION_1F
	warp_to 24, 3, CELADON_CITY_WIDTH ; CELADON_MANSION_1F
	warp_to 25, 3, CELADON_CITY_WIDTH ; CELADON_MANSION_1F
	warp_to 41, 9, CELADON_CITY_WIDTH ; CELADON_POKECENTER
	warp_to 12, 27, CELADON_CITY_WIDTH ; CELADON_GYM
	warp_to 28, 19, CELADON_CITY_WIDTH ; GAME_CORNER
	warp_to 39, 19, CELADON_CITY_WIDTH ; CELADON_MART_5F
	warp_to 33, 19, CELADON_CITY_WIDTH ; GAME_CORNER_PRIZE_ROOM
	warp_to 31, 27, CELADON_CITY_WIDTH ; CELADON_DINER
	warp_to 35, 27, CELADON_CITY_WIDTH ; CELADON_CHIEF_HOUSE
	warp_to 43, 27, CELADON_CITY_WIDTH ; CELADON_HOTEL