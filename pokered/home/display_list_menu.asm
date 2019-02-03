; INPUT:
; [wListMenuID] = list menu ID
; [wListPointer] = address of the list (2 bytes)
DisplayListMenuID::
	xor a
	ld [H_AUTOBGTRANSFERENABLED], a ; disable auto-transfer
	ld a, 1
	ld [hJoy7], a ; joypad state update flag
	ld a, [wBattleType]
	and a ; is it the Old Man battle?
	jr nz, .specialBattleType
	ld a, $01 ; hardcoded bank
	jr .bankswitch
.specialBattleType ; Old Man battle
	ld a, BANK(DisplayBattleMenu)
.bankswitch
	call BankswitchHome
	ld hl, wd730
	set 6, [hl] ; turn off letter printing delay
	xor a
	ld [wMenuItemToSwap], a ; 0 means no item is currently being swapped
	ld [wListCount], a
	ld a, [wListPointer]
	ld l, a
	ld a, [wListPointer + 1]
	ld h, a ; hl = address of the list
	ld a, [wListMenuID]
	cp ITEMLISTMENU
	push af
	call z, LoadCurrentItemPageLimits
	pop af
	call z, LoadItemListFromAPI
	ld a, [hl] ; the first byte is the number of entries in the list
	ld [wListCount], a
	ld a, LIST_MENU_BOX
	ld [wTextBoxID], a
	call DisplayTextBoxID ; draw the menu text box
	call UpdateSprites ; disable sprites behind the text box
; the code up to .skipMovingSprites appears to be useless
	coord hl, 4, 2 ; coordinates of upper left corner of menu text box
	lb de, 9, 14 ; height and width of menu text box
	ld a, [wListMenuID]
	and a ; is it a PC pokemon list?
	jr nz, .skipMovingSprites
	call UpdateSprites
.skipMovingSprites
	ld a, 1 ; max menu item ID is 1 if the list has less than 2 entries
	ld [wMenuWatchMovingOutOfBounds], a
	ld a, [wListCount]
	cp 2 ; does the list have less than 2 entries?
	jr c, .setMenuVariables
	ld a, 2 ; max menu item ID is 2 if the list has at least 2 entries
.setMenuVariables
	ld [wMaxMenuItem], a
	ld a, 4
	ld [wTopMenuItemY], a
	ld a, 5
	ld [wTopMenuItemX], a
	ld a, [wListMenuID]
	cp ITEMLISTMENU
	jr nz, .regular_watched_keys
	ld a, [wCurrentItemList]
	cp 2
	jr nc, .regular_watched_keys
	ld a, A_BUTTON | B_BUTTON | SELECT | D_LEFT | D_RIGHT
	jr .got_watched_keys
.regular_watched_keys
	ld a, A_BUTTON | B_BUTTON | SELECT
.got_watched_keys
	ld [wMenuWatchedKeys], a
	ld c, 10
	call DelayFrames

DisplayListMenuIDLoop::
	xor a
	ld [H_AUTOBGTRANSFERENABLED], a ; disable transfer
	call PrintListMenuEntries
	ld a, 1
	ld [H_AUTOBGTRANSFERENABLED], a ; enable transfer
	call Delay3
	ld a, [wBattleType]
	and a ; is it the Old Man battle?
	jr z, .notOldManBattle
.oldManBattle
	ld a, "▶"
	Coorda 5, 4 ; place menu cursor in front of first menu entry
	ld c, 80
	call DelayFrames
	xor a
	ld [wCurrentMenuItem], a
	coord hl, 5, 4
	ld a, l
	ld [wMenuCursorLocation], a
	ld a, h
	ld [wMenuCursorLocation + 1], a
	jr .buttonAPressed
.notOldManBattle
	call LoadGBPal
	call HandleMenuInput
	push af
	call PlaceMenuCursor
	pop af
	bit BIT_A_BUTTON, a
	jp z, .checkOtherKeys
.buttonAPressed
	ld a, [wCurrentMenuItem]
	call PlaceUnfilledArrowMenuCursor

; pointless because both values are overwritten before they are read
	ld a, $01
	ld [wMenuExitMethod], a
	ld [wChosenMenuItem], a

	xor a
	ld [wMenuWatchMovingOutOfBounds], a
	ld a, [wCurrentMenuItem]
	ld c, a
	ld a, [wListScrollOffset]
	add c
	ld c, a
	ld a, [wListCount]
	and a ; is the list empty?
	jp z, ExitListMenu ; if so, exit the menu
	dec a
	cp c ; did the player select Cancel?
	jp c, ExitListMenu ; if so, exit the menu
	ld a, c
	ld [wWhichPokemon], a
	ld a, [wListMenuID]
	cp ITEMLISTMENU
	jr nz, .skipMultiplying
; if it's an item menu
	sla c ; item entries are 2 bytes long, so multiply by 2
.skipMultiplying
	ld a, [wListPointer]
	ld l, a
	ld a, [wListPointer + 1]
	ld h, a
	inc hl ; hl = beginning of list entries
	ld b, 0
	add hl, bc
	ld a, [hl]
	ld [wcf91], a
	ld a, [wListMenuID]
	and a ; is it a PC pokemon list?
	jr z, .pokemonList
	push hl
	call GetItemPrice
	pop hl
	ld a, [wListMenuID]
	cp ITEMLISTMENU
	jr nz, .skipGettingQuantity
; if it's an item menu
	inc hl
	ld a, [hl] ; a = item quantity
	ld [wMaxItemQuantity], a
.skipGettingQuantity
	ld a, [wcf91]
	ld [wd0b5], a
	ld a, BANK(ItemNames)
	ld [wPredefBank], a
	call GetName
	jr .storeChosenEntry
.pokemonList
	ld hl, wPartyCount
	ld a, [wListPointer]
	cp l ; is it a list of party pokemon or box pokemon?
	ld hl, wPartyMonNicks
	jr z, .getPokemonName
	ld hl, wBoxMonNicks ; box pokemon names
.getPokemonName
	ld a, [wWhichPokemon]
	call GetPartyMonName
.storeChosenEntry ; store the menu entry that the player chose and return
	ld de, wcd6d
	call CopyStringToCF4B ; copy name to wcf4b
	ld a, CHOSE_MENU_ITEM
	ld [wMenuExitMethod], a
	ld a, [wCurrentMenuItem]
	ld [wChosenMenuItem], a
	xor a
	ld [hJoy7], a ; joypad state update flag
	ld hl, wd730
	res 6, [hl] ; turn on letter printing delay
	jp BankswitchBack
.checkOtherKeys ; check B, SELECT and directional keys
	bit BIT_B_BUTTON, a
	jp nz, ExitListMenu ; if so, exit the menu
	bit BIT_SELECT, a
	jp nz, HandleItemListSwapping ; if so, allow the player to swap menu entries
	ld b, a
	bit BIT_D_UP, b
	ld hl, wListScrollOffset
	jr nz, .upPressed
	bit BIT_D_DOWN, b
	jr z, .check_left_right
.downPressed
	ld a, [hl]
	add 3
	ld b, a
	ld a, [wListCount]
	cp b ; will going down scroll past the Cancel button?
	jr c, .jump_to_loop
	inc [hl] ; if not, go down
	jr .jump_to_loop
.upPressed
	ld a, [hl]
	and a
	jr z, .jump_to_loop
	dec [hl]
.jump_to_loop
	jp DisplayListMenuIDLoop

.check_left_right
	ld a, [wListMenuID]
	cp ITEMLISTMENU
	jr nz, .jump_to_loop
	ld a, [wCurrentItemPageLimit]
	ld c, a
	cp 2
	jr c, .jump_to_loop
	ld a, [wCurrentItemList]
	cp 2
	jr nc, .jump_to_loop
	and a
	ld hl, wCurrentItemPage
	jr z, .got_page_pointer
	inc hl
.got_page_pointer
	bit BIT_D_LEFT, b
	jr nz, .left
	; right
	ld a, [hl]
	inc a
	cp c
	jr c, .got_new_page
	xor a
	jr .got_new_page
.left
	ld a, [hl]
	and a
	jr nz, .got_previous_page
	ld a, c
.got_previous_page
	dec a
.got_new_page
	ld [hl], a
	ld c, a
	call LoadItemListFromAPI
	xor a
	ld [wListScrollOffset], a
	ld [wCurrentMenuItem], a
	ld [wBagSavedMenuItem], a
	ld [wSavedListScrollOffset], a
	ld a, [wNumItems]
	ld [wListCount], a
	cp 2
	jr c, .got_max_menu_item
	ld a, 2
.got_max_menu_item
	ld [wMaxMenuItem], a
	jr .jump_to_loop

LoadItemListFromAPI::
	ld a, [wCurrentItemList]
	cp 2
	ret nc
	push hl
	and a
	ld a, ITEMAPI_GET_PAGE
	ld hl, wCurrentItemPage
	jr z, .got_pointer
	ld a, ITEMAPI_GET_PC_PAGE
	inc hl
.got_pointer
	add a, [hl]
	call ItemAPI
	jr c, .no_name
	jr nz, .done
	xor a
	ld hl, wNumItems
	ld [hli], a
	ld [hl], -1
.no_name
	ld a, "@"
	ld [wItemAPIBuffer], a
.done
	pop hl
	ret

LoadCurrentItemPageLimits::
	ld a, [wCurrentItemList]
	cp 2
	ret nc
	push hl
	push bc
	ld a, ITEMAPI_GET_PAGE_LIMITS
	call ItemAPI
	ld a, 0
	jr c, .got_limit
	ld a, 1
	jr z, .got_limit
	ld a, [wCurrentItemList]
	ld b, a
	and a
	ld hl, wItemAPIBuffer
	ld a, [hli]
	jr z, .got_limit
	ld a, [hl]
.got_limit
	ld c, a
	ld a, b
	and a
	ld hl, wCurrentItemPage
	jr z, .got_page_pointer
	inc hl
.got_page_pointer
	ld a, [hl]
	cp c
	jr c, .page_OK
	xor a
	ld [hl], a
.page_OK
	ld [wCurrentItemPageLimit], a
	pop bc
	pop hl
	ret
