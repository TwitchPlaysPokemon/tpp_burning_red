TextScriptEndingChar::
	db "@"
TextScriptEnd::
	ld hl, TextScriptEndingChar
	ret

ExclamationText::
	TX_FAR _ExclamationText
	db "@"

GroundRoseText::
	TX_FAR _GroundRoseText
	db "@"

BoulderText::
	TX_FAR _BoulderText
	db "@"

MartSignText::
	TX_FAR _MartSignText
	db "@"

PokeCenterSignText::
	TX_FAR _PokeCenterSignText
	db "@"

PickUpItemText::
	TX_ASM
	predef PickUpItem
	jp TextScriptEnd

PokemonFaintedText::
	TX_FAR _PokemonFaintedText
	db "@"

PlayerBlackedOutText::
	TX_FAR _PlayerBlackedOutText
	db "@"

RepelWoreOffText::
	TX_FAR _RepelWoreOffText
	db "@"

PokemartGreetingText::
	TX_FAR _PokemartGreetingText
	db "@"
