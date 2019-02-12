SECTION "rst 00", ROM0
	rst $38
	nop ;padding
	nop

GBPalWhiteOutWithDelay3::
	call GBPalWhiteOut

Delay3::
; The bg map is updated each frame in thirds.
; Wait three frames to let the bg map fully update.
	ld c, 3

SECTION "rst 08", ROM0
RST_DelayFrames::
; wait c frames
	call DelayFrame
	dec c
	jr nz, RST_DelayFrames
	ret

GenericDummyFunction::
	ret

SECTION "rst 10", ROM0
	rst $38

SECTION "rst 18", ROM0
	rst $38

SECTION "rst 20", ROM0
	rst $38

SECTION "rst 28", ROM0
	rst $38

SECTION "rst 30", ROM0
	rst $38

SECTION "rst 38", ROM0
	rst $38

; Hardware interrupts
SECTION "vblank", ROM0
	jp VBlank

SECTION "hblank", ROM0
	rst $38

EnableLCD::
	ld a, [rLCDC]
	set rLCDC_ENABLE, a
	ld [rLCDC], a
	ret

SECTION "timer",  ROM0
	reti

SECTION "serial", ROM0
	jp Serial

SECTION "joypad", ROM0
	reti

SECTION "Home", ROM0

DisableLCD::
	xor a
	ld [rIF], a
	ld a, [rIE]
	ld b, a
	res 0, a
	ld [rIE], a

.wait
	ld a, [rLY]
	cp LY_VBLANK
	jr nz, .wait

	ld a, [rLCDC]
	and $ff ^ rLCDC_ENABLE_MASK
	ld [rLCDC], a
	ld a, b
	ld [rIE], a
	ret

ClearSprites::
	xor a
	ld hl, wOAMBuffer
	ld b, 40 * 4
.loop
	ld [hli], a
	dec b
	jr nz, .loop
	ret

HideSprites::
	ld a, 160
	ld hl, wOAMBuffer
	ld de, 4
	ld b, 40
.loop
	ld [hl], a
	add hl, de
	dec b
	jr nz, .loop
	ret

INCLUDE "home/copy_high.asm"

IF _ITEMAPI
INCLUDE "home/item_api.asm"
ENDC

SECTION "Entry", ROM0

	nop
	jp Start

SECTION "Header", ROM0
	; The header is generated by rgbfix.
	; The space here is allocated to prevent code from being overwritten.

	rept $150 - $104
		db 0
	endr

SECTION "Main", ROM0

INCLUDE "home/audio.asm"
INCLUDE "home/bcd.asm"
INCLUDE "home/choose_quantity_menu.asm"
INCLUDE "home/collision_data.asm"
INCLUDE "home/copy.asm"
INCLUDE "home/fade.asm"
INCLUDE "home/get_element_name.asm"
INCLUDE "home/init.asm"
INCLUDE "home/item_filter_lists.asm"
INCLUDE "home/items.asm"
INCLUDE "home/joypad.asm"
INCLUDE "home/map_header_pointers.asm"
INCLUDE "home/mart_inventories.asm"
INCLUDE "home/math.asm"
INCLUDE "home/menu.asm"
INCLUDE "home/overworld.asm"
INCLUDE "home/party_menu.asm"
INCLUDE "home/pic.asm"
INCLUDE "home/predef.asm"
INCLUDE "home/serial.asm"
INCLUDE "home/sprites.asm"
INCLUDE "home/start_menu.asm"
INCLUDE "home/stats.asm"
INCLUDE "home/text.asm"
INCLUDE "home/text_data.asm"
INCLUDE "home/text_predefs.asm"
INCLUDE "home/trainer.asm"
INCLUDE "home/trainer_types.asm"
INCLUDE "home/vblank.asm"
INCLUDE "home/vcopy.asm"

IF _ITEMAPI
INCLUDE "home/display_list_menu_api.asm"
ELSE
INCLUDE "home/display_list_menu.asm"
ENDC

Start::
	cp GBC
	jr z, .gbc
	xor a
	jr .ok
.gbc
	ld a, 0
.ok
	ld [wGBC], a
	jp Init

CheckForUserInterruption::
; Return carry if Up+Select+B, Start or A are pressed in c frames.
; Used only in the intro and title screen.
	call DelayFrame

	push bc
	call JoypadLowSensitivity
	pop bc

	ld a, [hJoyHeld]
	cp D_UP + SELECT + B_BUTTON
	jr z, .input

	ld a, [hJoy5]
	and START | A_BUTTON
	jr nz, .input

	dec c
	jr nz, CheckForUserInterruption

	and a
	ret

.input
	scf
	ret

; function to load position data for destination warp when switching maps
; INPUT:
; a = ID of destination warp within destination map
LoadDestinationWarpPosition::
	ld b, a
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [wPredefParentBank]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ld a, b
	add a
	add a
	ld c, a
	ld b, 0
	add hl, bc
	ld bc, 4
	ld de, wCurrentTileBlockMapViewPointer
	call CopyData
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

DrawHPBar::
; Draw an HP bar d tiles long, and fill it to e pixels.
; If c is nonzero, show at least a sliver regardless.
; The right end of the bar changes with [wHPBarType].

	push hl
	push de
	push bc

	; Left
	ld a, $71 ; "HP:"
	ld [hli], a
	ld a, $62
	ld [hli], a

	push hl

	; Middle
	ld a, $63 ; empty
.draw
	ld [hli], a
	dec d
	jr nz, .draw

	; Right
	ld a, [wHPBarType]
	dec a
	ld a, $6d ; status screen and battle
	jr z, .ok
	dec a ; pokemon menu
.ok
	ld [hl], a

	pop hl

	ld a, e
	and a
	jr nz, .fill

	; If c is nonzero, draw a pixel anyway.
	ld a, c
	and a
	jr z, .done
	ld e, 1

.fill
	ld a, e
	sub 8
	jr c, .partial
	ld e, a
	ld a, $6b ; full
	ld [hli], a
	ld a, e
	and a
	jr z, .done
	jr .fill

.partial
	; Fill remaining pixels at the end if necessary.
	ld a, $63 ; empty
	add e
	ld [hl], a
.done
	pop bc
	pop de
	pop hl
	ret

; loads pokemon data from one of multiple sources to wLoadedMon
; loads base stats to wMonHeader
; INPUT:
; [wWhichPokemon] = index of pokemon within party/box
; [wMonDataLocation] = source
; 00: player's party
; 01: enemy's party
; 02: current box
; 03: daycare
; OUTPUT:
; [wcf91] = pokemon ID
; wLoadedMon = base address of pokemon data
; wMonHeader = base address of base stats
LoadMonData::
	jpab LoadMonData_

ApplyPhanceroCryModifier::
	push hl
	ld hl, wd72c
	bit 2, [hl]
	pop hl
	ret z
	inc d
	ret

PlayCry::
; Play monster a's cry.
	cp PHANCERO
	jr nz, .notPhancero
	ld hl, wd72c
	set 2, [hl]
.notPhancero
	call GetCryData
	call PlaySound
	call WaitForSoundToFinish
	ld hl, wd72c
	res 2, [hl]
	ret

GetCryData::
; Load cry data for monster a.
	dec a
	ld c, a
	ld b, 0
	ld hl, CryData
	add hl, bc
	add hl, bc
	add hl, bc

	ld a, BANK(CryData)
	call BankswitchHome
	ld a, [hli]
	ld b, a ; cry id
	ld a, [hli]
	ld [wFrequencyModifier], a
	ld a, [hl]
	ld [wTempoModifier], a
	call BankswitchBack

	; Cry headers have 3 channels,
	; and start from index $14,
	; so add 3 times the cry id.
	ld a, b
	ld c, $14
	rlca ; * 2
	add b
	add c
	ret

; prints a pokemon's status condition
; INPUT:
; de = address of status condition
; hl = destination address
PrintStatusCondition::
	push de
	dec de
	dec de ; de = address of current HP
	ld a, [de]
	ld b, a
	dec de
	ld a, [de]
	or b ; is the pokemon's HP zero?
	pop de
	jr nz, PrintStatusConditionNotFainted
; if the pokemon's HP is 0, print "FNT"
	ld a, "F"
	ld [hli], a
	ld a, "N"
	ld [hli], a
	ld [hl], "T"
	and a
	ret

PrintStatusConditionNotFainted:
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(PrintStatusAilment)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call PrintStatusAilment ; print status condition
	pop bc
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; function to print pokemon level, leaving off the ":L" if the level is at least 100
; INPUT:
; hl = destination address
; [wLoadedMonLevel] = level
PrintLevel::
	ld a, $6e ; ":L" tile ID
	ld [hli], a
	ld c, 2 ; number of digits
	ld a, [wLoadedMonLevel] ; level
	cp 100
	jr c, PrintLevelCommon
; if level at least 100, write over the ":L" tile
	dec hl
	inc c ; increment number of digits to 3
	jr PrintLevelCommon

; prints the level without leaving off ":L" regardless of level
; INPUT:
; hl = destination address
; [wLoadedMonLevel] = level
PrintLevelFull::
	ld a, $6e ; ":L" tile ID
	ld [hli], a
	ld c, 3 ; number of digits
	ld a, [wLoadedMonLevel] ; level

PrintLevelCommon::
	ld [wd11e], a
	ld de, wd11e
	ld b, LEFT_ALIGN | 1 ; 1 byte
	jp PrintNumber

; copies the base stat data of a pokemon to wMonHeader
; INPUT:
; [wd0b5] = pokemon ID
GetMonHeader::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(BaseStats)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	push bc
	push de
	push hl
	ld a, [wd11e]
	push af
	ld a, [wd0b5]
	ld [wd11e], a
	ld de, FossilKabutopsPic
	ld b, $66 ; size of Kabutops fossil and Ghost sprites
	cp FOSSIL_KABUTOPS ; Kabutops fossil
	jr z, .specialID
	ld de, GhostPic
	cp MON_GHOST ; Ghost
	jr z, .specialID
	ld de, FossilAerodactylPic
	ld b, $77 ; size of Aerodactyl fossil sprite
	cp FOSSIL_AERODACTYL ; Aerodactyl fossil
	jr z, .specialID
	cp MEW
	jr z, .mew
	predef IndexToPokedex   ; convert pokemon ID in [wd11e] to pokedex number
	ld a, [wd11e]
	dec a
	ld bc, MonBaseStatsEnd - MonBaseStats
	ld hl, BaseStats
	call AddNTimes
	ld de, wMonHeader
	ld bc, MonBaseStatsEnd - MonBaseStats
	call CopyData
	jr .done
.specialID
	ld hl, wMonHSpriteDim
	ld [hl], b ; write sprite dimensions
	inc hl
	ld [hl], e ; write front sprite pointer
	inc hl
	ld [hl], d
	jr .done
.mew
	ld hl, MewBaseStats
	ld de, wMonHeader
	ld bc, MonBaseStatsEnd - MonBaseStats
	ld a, BANK(MewBaseStats)
	call FarCopyData
.done
	ld a, [wd0b5]
	ld [wMonHIndex], a
	pop af
	ld [wd11e], a
	pop hl
	pop de
	pop bc
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; copy party pokemon's name to wcd6d
GetPartyMonName2::
	ld a, [wWhichPokemon] ; index within party
	ld hl, wPartyMonNicks

; this is called more often
GetPartyMonName::
	push hl
	push bc
	call SkipFixedLengthTextEntries ; add NAME_LENGTH to hl, a times
	ld de, wcd6d
	push de
	ld bc, NAME_LENGTH
	call CopyData
	pop de
	pop bc
	pop hl
	ret

; this function is used to display sign messages, sprite dialog, etc.
; INPUT: [hSpriteIndexOrTextID] = sprite ID or text ID
DisplayTextID::
	ld a, [H_LOADEDROMBANK]
	push af
	callba DisplayTextIDInit ; initialization
	ld hl, wTextPredefFlag
	bit 0, [hl]
	res 0, [hl]
	jr nz, .skipSwitchToMapBank
	ld a, [wCurMap]
	call SwitchToMapRomBank
.skipSwitchToMapBank
	ld a, 30 ; half a second
	ld [H_FRAMECOUNTER], a ; used as joypad poll timer
	ld hl, wMapTextPtr
	ld a, [hli]
	ld h, [hl]
	ld l, a ; hl = map text pointer
	ld d, $00
	ld a, [hSpriteIndexOrTextID] ; text ID
	ld [wSpriteIndex], a
	and a
	jp z, DisplayStartMenu
	cp TEXT_SAFARI_GAME_OVER
	jp z, DisplaySafariGameOverText
	cp TEXT_MON_FAINTED
	jp z, DisplayPokemonFaintedText
	cp TEXT_BLACKED_OUT
	jp z, DisplayPlayerBlackedOutText
	cp TEXT_REPEL_WORE_OFF
	jp z, DisplayRepelWoreOffText
	ld a, [wNumSprites]
	ld e, a
	ld a, [hSpriteIndexOrTextID] ; sprite ID
	cp e
	jr z, .spriteHandling
	jr nc, .skipSpriteHandling
.spriteHandling
; get the text ID of the sprite
	push hl
	push de
	push bc
	callba UpdateSpriteFacingOffsetAndDelayMovement ; update the graphics of the sprite the player is talking to (to face the right direction)
	pop bc
	pop de
	ld hl, wMapSpriteData ; NPC text entries
	ld a, [hSpriteIndexOrTextID]
	dec a
	add a
	add l
	ld l, a
	jr nc, .noCarry
	inc h
.noCarry
	inc hl
	ld a, [hl] ; a = text ID of the sprite
	pop hl
.skipSpriteHandling
; look up the address of the text in the map's text entries
	dec a
	ld e, a
	sla e
	add hl, de
	ld a, [hli]
	ld h, [hl]
	ld l, a ; hl = address of the text
	ld a, [hl] ; a = first byte of text
; check first byte of text for special cases
	cp $fe   ; Pokemart NPC
	jp z, DisplayPokemartDialogue
	cp $ff   ; Pokemon Center NPC
	jp z, DisplayPokemonCenterDialogue
	cp $fc   ; Item Storage PC
	jp z, FuncTX_ItemStoragePC
	cp $fd   ; Bill's PC
	jp z, FuncTX_BillsPC
	cp $f9   ; Pokemon Center PC
	jp z, FuncTX_PokemonCenterPC
	cp $f5   ; Vending Machine
	jr nz, .notVendingMachine
	callba VendingMachineMenu ; jump banks to vending machine routine
	jr AfterDisplayingTextID
.notVendingMachine
	cp $f7   ; prize menu
	jp z, FuncTX_GameCornerPrizeMenu
	cp $f6   ; cable connection NPC in Pokemon Center
	jr nz, .notSpecialCase
	callab CableClubNPC
	jr AfterDisplayingTextID
.notSpecialCase
	call PrintText_NoCreatingTextBox ; display the text
	ld a, [wDoNotWaitForButtonPressAfterDisplayingText]
	and a
	jr nz, HoldTextDisplayOpen

AfterDisplayingTextID::
	ld a, [wEnteringCableClub]
	and a
	jr nz, HoldTextDisplayOpen
	call WaitForTextScrollButtonPress ; wait for a button press after displaying all the text

; loop to hold the dialogue box open as long as the player keeps holding down the A button
HoldTextDisplayOpen::
	call Joypad
	ld a, [hJoyHeld]
	bit 0, a ; is the A button being pressed?
	jr nz, HoldTextDisplayOpen

CloseTextDisplay::
	ld a, [wCurMap]
	call SwitchToMapRomBank
	ld a, $90
	ld [hWY], a ; move the window off the screen
	call DelayFrame
	call LoadGBPal
	xor a
	ld [H_AUTOBGTRANSFERENABLED], a ; disable continuous WRAM to VRAM transfer each V-blank
; loop to make sprites face the directions they originally faced before the dialogue
	ld hl, wSpriteStateData2 + $19
	ld c, $0f
	ld de, $0010
.restoreSpriteFacingDirectionLoop
	ld a, [hl]
	dec h
	ld [hl], a
	inc h
	add hl, de
	dec c
	jr nz, .restoreSpriteFacingDirectionLoop
	ld a, BANK(InitMapSprites)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call InitMapSprites ; reload sprite tile pattern data (since it was partially overwritten by text tile patterns)
	ld hl, wFontLoaded
	res 0, [hl]
	ld a, [wd732]
	bit 3, a ; used fly warp
	call z, LoadPlayerSpriteGraphics
	call LoadCurrentMapView
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	jp UpdateSprites

DisplayPokemartDialogue::
	push hl
	ld hl, PokemartGreetingText
	call PrintText
	pop hl
	inc hl
	call LoadItemList
	ld a, PRICEDITEMLISTMENU
	ld [wListMenuID], a
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, Bank(DisplayPokemartDialogue_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call DisplayPokemartDialogue_
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	jp AfterDisplayingTextID

DisplayPokemonCenterDialogue::
; zeroing these doesn't appear to serve any purpose
	xor a
	ld [$ff8b], a
	ld [$ff8c], a
	ld [$ff8d], a

	inc hl
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, Bank(DisplayPokemonCenterDialogue_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call DisplayPokemonCenterDialogue_
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	jp AfterDisplayingTextID

DisplaySafariGameOverText::
	callab PrintSafariGameOverText
	jp AfterDisplayingTextID

DisplayPokemonFaintedText::
	ld hl, PokemonFaintedText
	call PrintText
	jp AfterDisplayingTextID

DisplayPlayerBlackedOutText::
	ld hl, PlayerBlackedOutText
	call PrintText
	ld a, [wd732]
	res 5, a ; reset forced to use bike bit
	ld [wd732], a
	jp HoldTextDisplayOpen

DisplayRepelWoreOffText::
	ld hl, RepelWoreOffText
	call PrintText
	jp AfterDisplayingTextID

; function to count how many bits are set in a string of bytes
; INPUT:
; hl = address of string of bytes
; b = length of string of bytes
; OUTPUT:
; [wNumSetBits] = number of set bits
CountSetBits::
	ld c, 0
.loop
	ld a, [hli]
	ld e, a
	ld d, 8
.innerLoop ; count how many bits are set in the current byte
	srl e
	ld a, 0
	adc c
	ld c, a
	dec d
	jr nz, .innerLoop
	dec b
	jr nz, .loop
	ld a, c
	ld [wNumSetBits], a
	ret

; subtracts the amount the player paid from their money
; sets carry flag if there is enough money and unsets carry flag if not
SubtractAmountPaidFromMoney::
	jpba SubtractAmountPaidFromMoney_

; adds the amount the player sold to their money
AddAmountSoldToMoney::
	ld de, wPlayerMoney + 2
	ld hl, $ffa1 ; total price of items
	ld c, 3 ; length of money in bytes
	predef AddBCDPredef ; add total price to money
	ld a, MONEY_BOX
	ld [wTextBoxID], a
	call DisplayTextBoxID ; redraw money text box
	ld a, SFX_PURCHASE
	call PlaySoundWaitForCurrent
	jp WaitForSoundToFinish

ExitListMenu::
	ld a, [wCurrentMenuItem]
	ld [wChosenMenuItem], a
	ld a, CANCELLED_MENU
	ld [wMenuExitMethod], a
	ld [wMenuWatchMovingOutOfBounds], a
	xor a
	ld [hJoy7], a
	ld hl, wd730
	res 6, [hl]
	call BankswitchBack
	xor a
	ld [wMenuItemToSwap], a ; 0 means no item is currently being swapped
	scf
	ret

; sets carry if move is an HM, clears carry if move is not an HM
; Input: a = move ID
IsMoveHM::
	ld hl, HMMoves
	ld de, 1
	jp IsInArray

HMMoves::
	db CUT,FLY,SURF,STRENGTH,FLASH
	db $ff ; terminator

GetMoveName::
	push hl
	ld a, MOVE_NAME
	ld [wNameListType], a
	ld a, [wd11e]
	ld [wd0b5], a
	ld a, BANK(MoveNames)
	ld [wPredefBank], a
	call GetName
	ld de, wcd6d ; pointer to where move name is stored in RAM
	pop hl
	ret

; reloads text box tile patterns, current map view, and tileset tile patterns
ReloadMapData::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [wCurMap]
	call SwitchToMapRomBank
	call DisableLCD
	call LoadTextBoxTilePatterns
	call LoadCurrentMapView
	call LoadTilesetTilePatternData
	call EnableLCD
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; reloads tileset tile patterns
ReloadTilesetTilePatterns::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [wCurMap]
	call SwitchToMapRomBank
	call DisableLCD
	call LoadTilesetTilePatternData
	call EnableLCD
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; shows the town map and lets the player choose a destination to fly to
ChooseFlyDestination::
	ld hl, wd72e
	res 4, [hl]
	jpba LoadTownMap_Fly

; causes the text box to close without waiting for a button press after displaying text
DisableWaitingAfterTextDisplay::
	ld a, $01
	ld [wDoNotWaitForButtonPressAfterDisplayingText], a
	ret

; function to draw various text boxes
; INPUT:
; [wTextBoxID] = text box ID
; b, c = y, x cursor position (TWO_OPTION_MENU only)
DisplayTextBoxID::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(DisplayTextBoxID_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call DisplayTextBoxID_
	pop bc
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; not zero if an NPC movement script is running, the player character is
; automatically stepping down from a door, or joypad states are being simulated
IsPlayerCharacterBeingControlledByGame::
	ld a, [wNPCMovementScriptPointerTableNum]
	and a
	ret nz
	ld a, [wd736]
	bit 1, a ; currently stepping down from door bit
	ret nz
	ld a, [wd730]
	and $80
	ret

RunNPCMovementScript::
	ld hl, wd736
	bit 0, [hl]
	res 0, [hl]
	jr nz, .playerStepOutFromDoor
	ld a, [wNPCMovementScriptPointerTableNum]
	and a
	ret z
	dec a
	add a
	ld d, 0
	ld e, a
	ld hl, .NPCMovementScriptPointerTables
	add hl, de
	ld a, [hli]
	ld h, [hl]
	ld l, a
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [wNPCMovementScriptBank]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ld a, [wNPCMovementScriptFunctionNum]
	call CallFunctionInTable
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

.NPCMovementScriptPointerTables
	dw PalletMovementScriptPointerTable
	dw PewterMuseumGuyMovementScriptPointerTable
	dw PewterGymGuyMovementScriptPointerTable
.playerStepOutFromDoor
	jpba PlayerStepOutFromDoor

EndNPCMovementScript::
	jpba _EndNPCMovementScript

; stores hl in [wTrainerHeaderPtr]
StoreTrainerHeaderPointer::
	ld a, h
	ld [wTrainerHeaderPtr], a
	ld a, l
	ld [wTrainerHeaderPtr+1], a
	ret

; executes the current map script from the function pointer array provided in hl.
; a: map script index to execute (unless overridden by [wd733] bit 4)
ExecuteCurMapScriptInTable::
	push af
	push de
	call StoreTrainerHeaderPointer
	pop hl
	pop af
	push hl
	ld hl, wFlags_D733
	bit 4, [hl]
	res 4, [hl]
	jr z, .useProvidedIndex   ; test if map script index was overridden manually
	ld a, [wCurMapScript]
.useProvidedIndex
	pop hl
	ld [wCurMapScript], a
	call CallFunctionInTable
	ld a, [wCurMapScript]
	ret

LoadGymLeaderAndCityName::
	push de
	ld de, wGymCityName
	ld bc, $11
	call CopyData   ; load city name
	pop hl
	ld de, wGymLeaderName
	ld bc, NAME_LENGTH
	jp CopyData     ; load gym leader name

; sets opponent type and mon set/lvl based on the engaging trainer data
InitBattleEnemyParameters::
	ld a, [wEngagedTrainerClass]
	ld [wCurOpponent], a
	ld [wEnemyMonOrTrainerClass], a
	cp 200
	ld a, [wEngagedTrainerSet]
	jr c, .noTrainer
	ld [wTrainerNo], a
	ret
.noTrainer
	ld [wCurEnemyLVL], a
	ret

; checks if the player's coordinates match an arrow movement tile's coordinates
; and if so, decodes the RLE movement data
; b = player Y
; c = player X
DecodeArrowMovementRLE::
	ld a, [hli]
	cp $ff
	ret z ; no match in the list
	cp b
	jr nz, .nextArrowMovementTileEntry1
	ld a, [hli]
	cp c
	jr nz, .nextArrowMovementTileEntry2
	ld a, [hli]
	ld d, [hl]
	ld e, a
	ld hl, wSimulatedJoypadStatesEnd
	call DecodeRLEList
	dec a
	ld [wSimulatedJoypadStatesIndex], a
	ret
.nextArrowMovementTileEntry1
	inc hl
.nextArrowMovementTileEntry2
	inc hl
	inc hl
	jr DecodeArrowMovementRLE

FuncTX_ItemStoragePC::
	call SaveScreenTilesToBuffer2
	ld b, BANK(PlayerPC)
	ld hl, PlayerPC
	jr bankswitchAndContinue

FuncTX_BillsPC::
	call SaveScreenTilesToBuffer2
	ld b, BANK(BillsPC_)
	ld hl, BillsPC_
	jr bankswitchAndContinue

FuncTX_GameCornerPrizeMenu::
; XXX find a better name for this function
; special_F7
	ld b, BANK(CeladonPrizeMenu)
	ld hl, CeladonPrizeMenu
bankswitchAndContinue::
	call Bankswitch
	jp HoldTextDisplayOpen        ; continue to main text-engine function

FuncTX_PokemonCenterPC::
	ld b, BANK(ActivatePC)
	ld hl, ActivatePC
	jr bankswitchAndContinue

StartSimulatingJoypadStates::
	xor a
	ld [wOverrideSimulatedJoypadStatesMask], a
	ld [wSpriteStateData2 + $06], a ; player's sprite movement byte 1
	ld hl, wd730
	set 7, [hl]
	ret

DisplayPokedex::
	ld [wd11e], a
	jpba _DisplayPokedex

; tests if the player's coordinates are in a specified array
; INPUT:
; hl = address of array
; OUTPUT:
; [wCoordIndex] = if there is match, the matching array index
; sets carry if the coordinates are in the array, clears carry if not
ArePlayerCoordsInArray::
	ld a, [wYCoord]
	ld b, a
	ld a, [wXCoord]
	ld c, a
	; fallthrough

CheckCoords::
	xor a
	ld [wCoordIndex], a
.loop
	ld a, [hli]
	cp $ff ; reached terminator?
	jr z, .notInArray
	push hl
	ld hl, wCoordIndex
	inc [hl]
	pop hl
.compareYCoord
	cp b
	jr z, .compareXCoord
	inc hl
	jr .loop
.compareXCoord
	ld a, [hli]
	cp c
	jr nz, .loop
.inArray
	scf
	ret
.notInArray
	and a
	ret

; tests if a boulder's coordinates are in a specified array
; INPUT:
; hl = address of array
; [H_SPRITEINDEX] = index of boulder sprite
; OUTPUT:
; [wCoordIndex] = if there is match, the matching array index
; sets carry if the coordinates are in the array, clears carry if not
CheckBoulderCoords::
	push hl
	ld hl, wSpriteStateData2 + $04
	ld a, [H_SPRITEINDEX]
	swap a
	ld d, $0
	ld e, a
	add hl, de
	ld a, [hli]
	sub $4 ; because sprite coordinates are offset by 4
	ld b, a
	ld a, [hl]
	sub $4 ; because sprite coordinates are offset by 4
	ld c, a
	pop hl
	jp CheckCoords

; decodes a $ff-terminated RLEncoded list
; each entry is a pair of bytes <byte value> <repetitions>
; the final $ff will be replicated in the output list and a contains the number of bytes written
; de: input list
; hl: output list
DecodeRLEList::
	xor a
	ld [wRLEByteCount], a     ; count written bytes here
.listLoop
	ld a, [de]
	cp $ff
	jr z, .endOfList
	ld [hRLEByteValue], a ; store byte value to be written
	inc de
	ld a, [de]
	ld b, $0
	ld c, a                      ; number of bytes to be written
	ld a, [wRLEByteCount]
	add c
	ld [wRLEByteCount], a     ; update total number of written bytes
	ld a, [hRLEByteValue]
	call FillMemory              ; write a c-times to output
	inc de
	jr .listLoop
.endOfList
	ld a, $ff
	ld [hl], a                   ; write final $ff
	ld a, [wRLEByteCount]
	inc a                        ; include sentinel in counting
	ret

HasEnoughMoney::
; Check if the player has at least as much
; money as the 3-byte BCD value at hMoney.
	ld de, wPlayerMoney
	ld hl, hMoney
	ld c, 3
	jp StringCmp

HasEnoughCoins::
; Check if the player has at least as many
; coins as the 2-byte BCD value at hCoins.
	ld de, wPlayerCoins
	ld hl, hCoins
	ld c, 2
	jp StringCmp

BankswitchHome::
; switches to bank # in a
; Only use this when in the home bank!
	ld [wBankswitchHomeTemp], a
	ld a, [H_LOADEDROMBANK]
	ld [wBankswitchHomeSavedROMBank], a
	ld a, [wBankswitchHomeTemp]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

BankswitchBack::
; returns from BankswitchHome
	ld a, [wBankswitchHomeSavedROMBank]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

Bankswitch::
; self-contained bankswitch, use this when not in the home bank
; switches to the bank in b
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ld bc, .Return
	push bc
	jp hl
.Return
	pop bc
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; displays yes/no choice
; yes -> set carry
YesNoChoice::
	call SaveScreenTilesToBuffer1
	call InitYesNoTextBoxParameters
	jr DisplayYesNoChoice

InitYesNoTextBoxParameters::
	xor a ; YES_NO_MENU
	ld [wTwoOptionMenuID], a
	coord hl, 14, 7
	ld bc, $80f
	ret

YesNoChoicePokeCenter::
	call SaveScreenTilesToBuffer1
	ld a, HEAL_CANCEL_MENU
	ld [wTwoOptionMenuID], a
	coord hl, 11, 6
	lb bc, 8, 12
	jr DisplayYesNoChoice

DisplayYesNoChoice::
	ld a, TWO_OPTION_MENU
	ld [wTextBoxID], a
	call DisplayTextBoxID
	jp LoadScreenTilesFromBuffer1

LoadFontTilePatterns::
	ld a, [rLCDC]
	bit 7, a ; is the LCD enabled?
	jr nz, .on
.off
	ld hl, FontGraphics
	ld de, vFont
	ld bc, FontGraphicsEnd - FontGraphics
	ld a, BANK(FontGraphics)
	jp FarCopyDataDouble ; if LCD is off, transfer all at once
.on
	ld de, FontGraphics
	ld hl, vFont
	lb bc, BANK(FontGraphics), (FontGraphicsEnd - FontGraphics) / $8
	jp CopyVideoDataDouble ; if LCD is on, transfer during V-blank

LoadTextBoxTilePatterns::
	ld a, [rLCDC]
	bit 7, a ; is the LCD enabled?
	jr nz, .on
.off
	ld hl, TextBoxGraphics
	ld de, vChars2 + $600
	ld bc, TextBoxGraphicsEnd - TextBoxGraphics
	ld a, BANK(TextBoxGraphics)
	jp FarCopyData2 ; if LCD is off, transfer all at once
.on
	ld de, TextBoxGraphics
	ld hl, vChars2 + $600
	lb bc, BANK(TextBoxGraphics), (TextBoxGraphicsEnd - TextBoxGraphics) / $10
	jp CopyVideoData ; if LCD is on, transfer during V-blank

LoadHpBarAndStatusTilePatterns::
	ld a, [rLCDC]
	bit 7, a ; is the LCD enabled?
	jr nz, .on
.off
	ld hl, HpBarAndStatusGraphics
	ld de, vChars2 + $620
	ld bc, HpBarAndStatusGraphicsEnd - HpBarAndStatusGraphics
	ld a, BANK(HpBarAndStatusGraphics)
	jp FarCopyData2 ; if LCD is off, transfer all at once
.on
	ld de, HpBarAndStatusGraphics
	ld hl, vChars2 + $620
	lb bc, BANK(HpBarAndStatusGraphics), (HpBarAndStatusGraphicsEnd - HpBarAndStatusGraphics) / $10
	jp CopyVideoData ; if LCD is on, transfer during V-blank

UncompressSpriteFromDE::
; Decompress pic at a:de.
	ld hl, wSpriteInputPtr
	ld [hl], e
	inc hl
	ld [hl], d
	jp UncompressSpriteData

SaveScreenTilesToBuffer2::
	coord hl, 0, 0
	ld de, wTileMapBackup2
	ld bc, SCREEN_WIDTH * SCREEN_HEIGHT
	call CopyData
	ret

LoadScreenTilesFromBuffer2::
	call LoadScreenTilesFromBuffer2DisableBGTransfer
	ld a, 1
	ld [H_AUTOBGTRANSFERENABLED], a
	ret

; loads screen tiles stored in wTileMapBackup2 but leaves H_AUTOBGTRANSFERENABLED disabled
LoadScreenTilesFromBuffer2DisableBGTransfer::
	xor a
	ld [H_AUTOBGTRANSFERENABLED], a
	ld hl, wTileMapBackup2
	coord de, 0, 0
	ld bc, SCREEN_WIDTH * SCREEN_HEIGHT
	call CopyData
	ret

SaveScreenTilesToBuffer1::
	coord hl, 0, 0
	ld de, wTileMapBackup
	ld bc, SCREEN_WIDTH * SCREEN_HEIGHT
	jp CopyData

LoadScreenTilesFromBuffer1::
	xor a
	ld [H_AUTOBGTRANSFERENABLED], a
	ld hl, wTileMapBackup
	coord de, 0, 0
	ld bc, SCREEN_WIDTH * SCREEN_HEIGHT
	call CopyData
	ld a, 1
	ld [H_AUTOBGTRANSFERENABLED], a
	ret

; this function is used when lower button sensitivity is wanted (e.g. menus)
; OUTPUT: [hJoy5] = pressed buttons in usual format
; there are two flags that control its functionality, [hJoy6] and [hJoy7]
; there are essentially three modes of operation
; 1. Get newly pressed buttons only
;    ([hJoy7] == 0, [hJoy6] == any)
;    Just copies [hJoyPressed] to [hJoy5].
; 2. Get currently pressed buttons at low sample rate with delay
;    ([hJoy7] == 1, [hJoy6] != 0)
;    If the user holds down buttons for more than half a second,
;    report buttons as being pressed up to 12 times per second thereafter.
;    If the user holds down buttons for less than half a second,
;    report only one button press.
; 3. Same as 2, but report no buttons as pressed if A or B is held down.
;    ([hJoy7] == 1, [hJoy6] == 0)
JoypadLowSensitivity::
	call Joypad
	ld a, [hJoy7] ; flag
	and a ; get all currently pressed buttons or only newly pressed buttons?
	ld a, [hJoyPressed] ; newly pressed buttons
	jr z, .storeButtonState
	ld a, [hJoyHeld] ; all currently pressed buttons
.storeButtonState
	ld [hJoy5], a
	ld a, [hJoyPressed] ; newly pressed buttons
	and a ; have any buttons been newly pressed since last check?
	jr z, .noNewlyPressedButtons
.newlyPressedButtons
	ld a, 30 ; half a second delay
	ld [H_FRAMECOUNTER], a
	ret
.noNewlyPressedButtons
	ld a, [H_FRAMECOUNTER]
	and a ; is the delay over?
	jr z, .delayOver
.delayNotOver
	xor a
	ld [hJoy5], a ; report no buttons as pressed
	ret
.delayOver
; if [hJoy6] = 0 and A or B is pressed, report no buttons as pressed
	ld a, [hJoyHeld]
	and A_BUTTON | B_BUTTON
	jr z, .setShortDelay
	ld a, [hJoy6] ; flag
	and a
	jr nz, .setShortDelay
	xor a
	ld [hJoy5], a
.setShortDelay
	ld a, 5 ; 1/12 of a second delay
	ld [H_FRAMECOUNTER], a
	ret

WaitForTextScrollButtonPress::
	ld a, [H_DOWNARROWBLINKCNT1]
	push af
	ld a, [H_DOWNARROWBLINKCNT2]
	push af
	xor a
	ld [H_DOWNARROWBLINKCNT1], a
	ld a, $6
	ld [H_DOWNARROWBLINKCNT2], a
.loop
	push hl
	ld a, [wTownMapSpriteBlinkingEnabled]
	and a
	jr z, .skipAnimation
	call TownMapSpriteBlinkingAnimation
.skipAnimation
	coord hl, 18, 16
	call HandleDownArrowBlinkTiming
	pop hl
	call JoypadLowSensitivity
	predef CableClub_Run
	ld a, [hJoy5]
	and A_BUTTON | B_BUTTON
	jr z, .loop
	pop af
	ld [H_DOWNARROWBLINKCNT2], a
	pop af
	ld [H_DOWNARROWBLINKCNT1], a
	ret

; (unless in link battle) waits for A or B being pressed and outputs the scrolling sound effect
ManualTextScroll::
	ld a, [wLinkState]
	cp LINK_STATE_BATTLING
	jr z, .inLinkBattle
	call WaitForTextScrollButtonPress
	ld a, SFX_PRESS_AB
	jp PlaySound
.inLinkBattle
	ld c, 65
	rst DelayFrames
	ret

; This function is used to wait a short period after printing a letter to the
; screen unless the player presses the A/B button or the delay is turned off
; through the [wd730] or [wLetterPrintingDelayFlags] flags.
PrintLetterDelay::
	ld a, [wd730]
	bit 6, a
	ret nz
	ld a, [wLetterPrintingDelayFlags]
	bit 1, a
	ret z
	push hl
	push de
	push bc
	ld a, [wLetterPrintingDelayFlags]
	bit 0, a
	jr z, .waitOneFrame
	ld a, [wOptions]
	and $f
	ld [H_FRAMECOUNTER], a
	jr .checkButtons
.waitOneFrame
	ld a, 1
	ld [H_FRAMECOUNTER], a
.checkButtons
	call Joypad
	ld a, [hJoyHeld]
.checkAButton
	bit 0, a ; is the A button pressed?
	jr z, .checkBButton
	jr .endWait
.checkBButton
	bit 1, a ; is the B button pressed?
	jr z, .buttonsNotPressed
.endWait
	call DelayFrame
	jr .done
.buttonsNotPressed ; if neither A nor B is pressed
	ld a, [H_FRAMECOUNTER]
	and a
	jr nz, .checkButtons
.done
	pop bc
	pop de
	pop hl
	ret

; Function to remove a pokemon from the party or the current box.
; wWhichPokemon determines the pokemon.
; [wRemoveMonFromBox] == 0 specifies the party.
; [wRemoveMonFromBox] != 0 specifies the current box.
RemovePokemon::
	jpab _RemovePokemon

AddPartyMon::
	push hl
	push de
	push bc
	callba _AddPartyMon
	pop bc
	pop de
	pop hl
	ret

AddEnemyMonToPlayerParty::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(_AddEnemyMonToPlayerParty)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call _AddEnemyMonToPlayerParty
	pop bc
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

MoveMon::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(_MoveMon)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call _MoveMon
	pop bc
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; skips a text entries, each of size NAME_LENGTH (like trainer name, OT name, rival name, ...)
; hl: base pointer, will be incremented by NAME_LENGTH * a
SkipFixedLengthTextEntries::
	and a
	ret z
	ld bc, NAME_LENGTH
.skipLoop
	add hl, bc
	dec a
	jr nz, .skipLoop
	ret

; INPUT:
; a = oam block index (each block is 4 oam entries)
; b = Y coordinate of upper left corner of sprite
; c = X coordinate of upper left corner of sprite
; de = base address of 4 tile number and attribute pairs
WriteOAMBlock::
	ld h, wOAMBuffer / $100
	swap a ; multiply by 16
	ld l, a
	call .writeOneEntry ; upper left
	push bc
	ld a, 8
	add c
	ld c, a
	call .writeOneEntry ; upper right
	pop bc
	ld a, 8
	add b
	ld b, a
	call .writeOneEntry ; lower left
	ld a, 8
	add c
	ld c, a
	                      ; lower right
.writeOneEntry
	ld [hl], b ; Y coordinate
	inc hl
	ld [hl], c ; X coordinate
	inc hl
	ld a, [de] ; tile number
	inc de
	ld [hli], a
	ld a, [de] ; attribute
	inc de
	ld [hli], a
	ret

; This toggles a blinking down arrow at hl on and off after a delay has passed.
; This is often called even when no blinking is occurring.
; The reason is that most functions that call this initialize H_DOWNARROWBLINKCNT1 to 0.
; The effect is that if the tile at hl is initialized with a down arrow,
; this function will toggle that down arrow on and off, but if the tile isn't
; initialized with a down arrow, this function does nothing.
; That allows this to be called without worrying about if a down arrow should
; be blinking.
HandleDownArrowBlinkTiming::
	ld a, [hl]
	ld b, a
	ld a, "▼"
	cp b
	jr nz, .downArrowOff
.downArrowOn
	ld a, [H_DOWNARROWBLINKCNT1]
	dec a
	ld [H_DOWNARROWBLINKCNT1], a
	ret nz
	ld a, [H_DOWNARROWBLINKCNT2]
	dec a
	ld [H_DOWNARROWBLINKCNT2], a
	ret nz
	ld a, " "
	ld [hl], a
	ld a, $ff
	ld [H_DOWNARROWBLINKCNT1], a
	ld a, $06
	ld [H_DOWNARROWBLINKCNT2], a
	ret
.downArrowOff
	ld a, [H_DOWNARROWBLINKCNT1]
	and a
	ret z
	dec a
	ld [H_DOWNARROWBLINKCNT1], a
	ret nz
	dec a
	ld [H_DOWNARROWBLINKCNT1], a
	ld a, [H_DOWNARROWBLINKCNT2]
	dec a
	ld [H_DOWNARROWBLINKCNT2], a
	ret nz
	ld a, $06
	ld [H_DOWNARROWBLINKCNT2], a
	ld a, "▼"
	ld [hl], a
	ret

; The following code either enables or disables the automatic drawing of
; text boxes by DisplayTextID. Both functions cause DisplayTextID to wait
; for a button press after displaying text (unless [wEnteringCableClub] is set).

EnableAutoTextBoxDrawing::
	xor a
	jr AutoTextBoxDrawingCommon

DisableAutoTextBoxDrawing::
	ld a, $01

AutoTextBoxDrawingCommon::
	ld [wAutoTextBoxDrawingControl], a
	xor a
	ld [wDoNotWaitForButtonPressAfterDisplayingText], a ; make DisplayTextID wait for button press
	ret

PrintText::
; Print text hl at (1, 14).
	push hl
	ld a, MESSAGE_BOX
	ld [wTextBoxID], a
	call DisplayTextBoxID
	call UpdateSprites
	call Delay3
	pop hl
PrintText_NoCreatingTextBox::
	coord bc, 1, 14
	jp TextCommandProcessor

PrintNumber::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(PrintNumber_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call PrintNumber_
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

CallFunctionInTable::
; Call function a in jumptable hl.
; de is not preserved.
	push hl
	push de
	push bc
	add a
	ld d, 0
	ld e, a
	add hl, de
	ld a, [hli]
	ld h, [hl]
	ld l, a
	ld de, .returnAddress
	push de
	jp hl
.returnAddress
	pop bc
	pop de
	pop hl
	ret

IsInArray::
; Search an array at hl for the value in a.
; Entry size is de bytes.
; Return count b and carry if found.
	ld b, 0

IsInRestOfArray::
	ld c, a
.loop
	ld a, [hl]
	cp -1
	jr z, .notfound
	cp c
	jr z, .found
	inc b
	add hl, de
	jr .loop

.notfound
	and a
	ret

.found
	scf
	ret

RestoreScreenTilesAndReloadTilePatterns::
	call ClearSprites
	ld a, $1
	ld [wUpdateSpritesEnabled], a
	call ReloadMapSpriteTilePatterns
	call LoadScreenTilesFromBuffer2
	call LoadTextBoxTilePatterns
	call RunDefaultPaletteCommand
	jp Delay3

GBPalNormal::
; Reset BGP and OBP0.
	ld a, %11100100 ; 3210
	ld [rBGP], a
	ld a, %11010000 ; 3100
	ld [rOBP0], a
	ret

GBPalWhiteOut::
; White out all palettes.
	xor a
	ld [rBGP], a
	ld [rOBP0], a
	ld [rOBP1], a
	ret

RunDefaultPaletteCommand::
	ld b, $ff
RunPaletteCommand::
	ld a, [wOnSGB]
	and a
	ret z
	predef_jump _RunPaletteCommand

GetHealthBarColor::
; Return at hl the palette of
; an HP bar e pixels long.
	ld a, e
	cp 27
	ld d, 0 ; green
	jr nc, .gotColor
	cp 10
	inc d ; yellow
	jr nc, .gotColor
	inc d ; red
.gotColor
	ld [hl], d
	ret

GivePokemon::
; Give the player monster b at level c.
	ld a, b
	ld [wcf91], a
	ld a, c
	ld [wCurEnemyLVL], a
	xor a ; PLAYER_PARTY_DATA
	ld [wMonDataLocation], a
	jpba _GivePokemon

Random::
; Return a random number in a.
; For battles, use BattleRandom.
	push hl
	push de
	push bc
	callba Random_
	ld a, [hRandomAdd]
	pop bc
	pop de
	pop hl
	ret

UpdateCinnabarGymGateTileBlocks::
	jpba UpdateCinnabarGymGateTileBlocks_

CheckForHiddenObjectOrBookshelfOrCardKeyDoor::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [hJoyHeld]
	bit 0, a ; A button
	jr z, .nothingFound
; A button is pressed
	ld a, Bank(CheckForHiddenObject)
	ld [MBC1RomBank], a
	ld [H_LOADEDROMBANK], a
	call CheckForHiddenObject
	ld a, [$ffee]
	and a
	jr nz, .hiddenObjectNotFound
	ld a, [wHiddenObjectFunctionRomBank]
	ld [MBC1RomBank], a
	ld [H_LOADEDROMBANK], a
	ld de, .returnAddress
	push de
	jp hl
.returnAddress
	xor a
	jr .done
.hiddenObjectNotFound
	callba PrintBookshelfText
	ld a, [$ffdb]
	and a
	jr z, .done
.nothingFound
	ld a, $ff
.done
	ld [$ffeb], a
	pop af
	ld [MBC1RomBank], a
	ld [H_LOADEDROMBANK], a
	ret

PrintPredefTextID::
	ld [hSpriteIndexOrTextID], a
	ld hl, TextPredefs
	call SetMapTextPointer
	ld hl, wTextPredefFlag
	set 0, [hl]
	call DisplayTextID

RestoreMapTextPointer::
	ld hl, wMapTextPtr
	ld a, [$ffec]
	ld [hli], a
	ld a, [$ffec + 1]
	ld [hl], a
	ret

SetMapTextPointer::
	ld a, [wMapTextPtr]
	ld [$ffec], a
	ld a, [wMapTextPtr + 1]
	ld [$ffec + 1], a
	ld a, l
	ld [wMapTextPtr], a
	ld a, h
	ld [wMapTextPtr + 1], a
	ret

PrintTwoDigitNumber:
	; from a to hl
	push bc
	ld c, -1
.loop
	inc c
	sub 10
	jr nc, .loop
	inc hl
	; takes advantage that -10 ($f6) is the codepoint for "0"
	ld [hld], a
	ld a, c
	pop bc
	and a
	jr nz, .got_tens
	ld a, " " - "0"
.got_tens
	add a, "0"
	ld [hl], a
	ret
