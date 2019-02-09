DisplayPartyMenu::
	ld a, [hTilesetType]
	push af
	xor a
	ld [hTilesetType], a
	call GBPalWhiteOutWithDelay3
	call ClearSprites
	call PartyMenuInit
	call DrawPartyMenu
	jp HandlePartyMenuInput

GoBackToPartyMenu::
	ld a, [hTilesetType]
	push af
	xor a
	ld [hTilesetType], a
	call PartyMenuInit
	call RedrawPartyMenu
	jp HandlePartyMenuInput

PartyMenuInit::
	ld a, 1 ; hardcoded bank
	call BankswitchHome
	call LoadHpBarAndStatusTilePatterns
	ld hl, wd730
	set 6, [hl] ; turn off letter printing delay
	xor a ; PLAYER_PARTY_DATA
	ld [wMonDataLocation], a
	ld [wMenuWatchMovingOutOfBounds], a
	ld hl, wTopMenuItemY
	inc a
	ld [hli], a ; top menu item Y
	xor a
	ld [hli], a ; top menu item X
	ld a, [wPartyAndBillsPCSavedMenuItem]
	push af
	ld [hli], a ; current menu item ID
	inc hl
	ld a, [wPartyCount]
	and a ; are there more than 0 pokemon in the party?
	jr z, .storeMaxMenuItemID
	dec a
; if party is not empty, the max menu item ID is ([wPartyCount] - 1)
; otherwise, it is 0
.storeMaxMenuItemID
	ld [hli], a ; max menu item ID
	ld a, [wForcePlayerToChooseMon]
	and a
	ld a, A_BUTTON | B_BUTTON
	jr z, .next
	xor a
	ld [wForcePlayerToChooseMon], a
	inc a ; a = A_BUTTON
.next
	ld [hli], a ; menu watched keys
	pop af
	ld [hl], a ; old menu item ID
	ret

HandlePartyMenuInput::
	ld a, 1
	ld [wMenuWrappingEnabled], a
	ld a, $40
	ld [wPartyMenuAnimMonEnabled], a
	call HandleMenuInput_
	call PlaceUnfilledArrowMenuCursor
	ld b, a
	xor a
	ld [wPartyMenuAnimMonEnabled], a
	ld a, [wCurrentMenuItem]
	ld [wPartyAndBillsPCSavedMenuItem], a
	ld hl, wd730
	res 6, [hl] ; turn on letter printing delay
	ld a, [wMenuItemToSwap]
	and a
	jp nz, .swappingPokemon
	pop af
	ld [hTilesetType], a
	bit 1, b
	jr nz, .noPokemonChosen
	ld a, [wPartyCount]
	and a
	jr z, .noPokemonChosen
	ld a, [wCurrentMenuItem]
	ld [wWhichPokemon], a
	ld hl, wPartySpecies
	ld b, 0
	ld c, a
	add hl, bc
	ld a, [hl]
	ld [wcf91], a
	ld [wBattleMonSpecies2], a
	call BankswitchBack
	and a
	ret
.noPokemonChosen
	call BankswitchBack
	scf
	ret
.swappingPokemon
	bit 1, b ; was the B button pressed?
	jr z, .handleSwap ; if not, handle swapping the pokemon
.cancelSwap ; if the B button was pressed
	callba ErasePartyMenuCursors
	xor a
	ld [wMenuItemToSwap], a
	ld [wPartyMenuTypeOrMessageID], a
	call RedrawPartyMenu
	jr HandlePartyMenuInput
.handleSwap
	ld a, [wCurrentMenuItem]
	ld [wWhichPokemon], a
	callba SwitchPartyMon
	jr HandlePartyMenuInput

DrawPartyMenu::
	ld hl, DrawPartyMenu_
	jr DrawPartyMenuCommon

RedrawPartyMenu::
	ld hl, RedrawPartyMenu_

DrawPartyMenuCommon::
	ld b, BANK(RedrawPartyMenu_)
	jp Bankswitch
