Route15Mons:
	db $0F
	IF _RED
		db 24,ODDISH
		db 26,DITTO
		db 23,PIDGEY
		db 26,VENONAT
		db 22,ODDISH
		db 28,VENONAT
		db 26,ODDISH
		db 30,GLOOM
		db 28,PIDGEOTTO
		db 30,PIDGEOTTO
	ENDC
	IF _BLUE
		db 24,BELLSPROUT
		db 26,DITTO
		db 23,PIDGEY
		db 26,VENONAT
		db 22,BELLSPROUT
		db 28,VENONAT
		db 26,BELLSPROUT
		db 30,WEEPINBELL
		db 28,PIDGEOTTO
		db 30,PIDGEOTTO
	ENDC
	db $00
