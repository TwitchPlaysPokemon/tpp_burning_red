GlitchCeladonCity_h:
	db OVERWORLD ; tileset
	db GLITCH_CELADON_CITY_HEIGHT, GLITCH_CELADON_CITY_WIDTH ; dimensions (y, x)
	dw GlitchCeladonCity_Blocks ; blocks
	dw GlitchCeladonCity_TextPointers ; texts
	dw GlitchCeladonCity_Script ; scripts
	db WEST | EAST ; connections
	WEST_MAP_CONNECTION GLITCH_CELADON_CITY, GLITCH_CELADON_CITY, 7, 0, GlitchCeladonCity_Blocks
	EAST_MAP_CONNECTION GLITCH_CELADON_CITY, GLITCH_CELADON_CITY, -7, 0, GlitchCeladonCity_Blocks
	dw GlitchCeladonCity_Object ; objects
