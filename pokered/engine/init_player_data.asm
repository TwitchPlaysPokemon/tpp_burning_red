InitPlayerData:
InitPlayerData2:

	call Random
	ld a, [hRandomSub]
	ld [wPlayerID], a

	call Random
	ld a, [hRandomAdd]
	ld [wPlayerID + 1], a

	ld a, $ff
	ld [wUnusedD71B], a

	ld hl, wPartyCount
	call InitializeEmptyList
	ld hl, wNumInBox
	call InitializeEmptyList
IF _ITEMAPI
.initialize_player_items
	ld a, ITEMAPI_INITIALIZE_ITEM_LISTS
	call ItemAPI
	; keep going until it returns true or null
	jr c, .items_initialized
	jr z, .initialize_player_items
.items_initialized
	xor a
	ld [wCurrentItemPage], a
	ld [wCurrentPCItemPage], a
ELSE
	ld hl, wNumBagItems
	call InitializeEmptyList
	ld hl, wNumBoxItems
	call InitializeEmptyList
ENDC

START_MONEY EQU $3000
	ld hl, wPlayerMoney + 1
	ld a, START_MONEY / $100
	ld [hld], a
	xor a
	ld [hli], a
	inc hl
	ld [hl], a

	ld [wMonDataLocation], a

	ld hl, wObtainedBadges
	ld [hli], a

	ld [hl], a

	ld hl, wPlayerCoins
	ld [hli], a
	ld [hl], a

	ld hl, wGameProgressFlags
	ld bc, wGameProgressFlagsEnd - wGameProgressFlags
	call FillMemory ; clear all game progress flags

	jp InitializeMissableObjectsFlags

InitializeEmptyList:
	xor a ; count
	ld [hli], a
	dec a ; terminator
	ld [hl], a
	ret
