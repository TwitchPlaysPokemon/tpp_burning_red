Route2Mons:
	db $19
	db 3,RATTATA
	db 3,PIDGEY
	db 4,PIDGEY
	db 4,RATTATA
	db 5,PIDGEY
	IF _RED
		db 3,WEEDLE
	ENDC
	IF _BLUE
		db 3,CATERPIE
	ENDC
	db 2,RATTATA
	db 5,RATTATA
	IF _RED
		db 4,WEEDLE
		db 5,WEEDLE
	ENDC
	IF _BLUE
		db 4,CATERPIE
		db 5,CATERPIE
	ENDC
	db $00
