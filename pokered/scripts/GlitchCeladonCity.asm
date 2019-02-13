GlitchCeladonCity_Script:
	call EnableAutoTextBoxDrawing
	ld hl, PhanceroTrainerHeader
	ld de, GlitchCeladonCity_ScriptPointers
	ld a, [wGlitchCeladonCityCurScript]
	call ExecuteCurMapScriptInTable
	ld [wGlitchCeladonCityCurScript], a
	ret
	
GlitchCeladonCity_ScriptPointers:
	dw CheckFightingMapTrainers
	dw DisplayEnemyTrainerTextAndStartBattle
	dw EndTrainerBattle


GlitchCeladonCity_TextPointers:
	dw PhanceroText

PhanceroTrainerHeader:
	dbEventFlagBit EVENT_8C2
	db ($0 << 4) ; trainer's view range
	dwEventFlagAddress EVENT_8C2
	dw PhanceroBattleText ; TextBeforeBattle
	dw PhanceroBattleText ; TextAfterBattle
	dw PhanceroBattleText ; TextEndBattle
	dw PhanceroBattleText ; TextEndBattle

	db $ff

PhanceroText:
	TX_ASM
	ld hl, PhanceroTrainerHeader
	call TalkToTrainer
	jp TextScriptEnd

PhanceroBattleText:
	TX_FAR _PhanceroBattleText
	TX_ASM
	ld a, PHANCERO
	call PlayCry
	call WaitForSoundToFinish
	jp TextScriptEnd
