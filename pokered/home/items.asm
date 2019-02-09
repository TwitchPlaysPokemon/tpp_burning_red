LoadItemList::
	ld a, 1
	ld [wUpdateSpritesEnabled], a
	ld a, h
	ld [wItemListPointer], a
	ld a, l
	ld [wItemListPointer + 1], a
	ld de, wItemList
.loop
	ld a, [hli]
	ld [de], a
	inc de
	cp $ff
	jr nz, .loop
	ret

; function to remove an item (in varying quantities) from the player's bag or PC box
; INPUT:
; (no item API) hl = address of inventory (either wNumBagItems or wNumBoxItems)
; (item API)    hl = address of page number: wCurrentItemPage, wCurrentPCItemPage (specific pages)
; [wWhichPokemon] = index (within the inventory) of the item to remove
; [wItemQuantity] = quantity to remove
RemoveItemFromInventory::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(RemoveItemFromInventory_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call RemoveItemFromInventory_
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; function to add an item (in varying quantities) to the player's bag or PC box
; INPUT:
; (no item API) hl = address of inventory (either wNumBagItems or wNumBoxItems)
; (item API)    hl = address of page number: wCurrentItemPage, wCurrentPCItemPage (specific pages),
;                                            LOW(wCurrentItemPage), LOW(wCurrentPCItemPage) (any page)
; [wcf91] = item ID
; [wItemQuantity] = item quantity
; sets carry flag if successful, unsets carry flag if unsuccessful
AddItemToInventory::
	push bc
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(AddItemToInventory_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call AddItemToInventory_
	pop bc
	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	pop bc
	ret

; sets carry if item is HM, clears carry if item is not HM
; Input: a = item ID
IsItemHM::
	cp HM_01
	jr c, .notHM
	cp TM_01
	ret
.notHM
	and a
	ret

; uses an item
; UseItem is used with dummy items to perform certain other functions as well
; INPUT:
; [wcf91] = item ID
; OUTPUT:
; [wActionResultOrTookBattleTurn] = success
; 00: unsuccessful
; 01: successful
; 02: not able to be used right now, no extra menu displayed (only certain items use this)
UseItem::
	jpba UseItem_

; confirms the item toss and then tosses the item
; INPUT:
; (no item API) hl = address of inventory (either wNumBagItems or wNumBoxItems)
; (item API)    hl = address of page number (wCurrentItemPage or wCurrentPCItemPage)
; [wcf91] = item ID
; [wWhichPokemon] = index of item within inventory
; [wItemQuantity] = quantity to toss
; OUTPUT:
; clears carry flag if the item is tossed, sets carry flag if not
TossItem::
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(TossItem_)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call TossItem_
	pop de
	ld a, d
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; checks if an item is a key item
; INPUT:
; [wcf91] = item ID
; OUTPUT:
; [wIsKeyItem] = result
; 00: item is not key item
; 01: item is key item
IsKeyItem::
	push hl
	push de
	push bc
	callba IsKeyItem_
	pop bc
	pop de
	pop hl
	ret

IsItemInBag::
; given an item_id in b
; set zero flag if item isn't in player's bag
; else reset zero flag
; related to Pokémon Tower and ghosts
	predef GetQuantityOfItemInBag
	ld a, b
	and a
	ret

GiveItem::
; Give player quantity c of item b,
; and copy the item's name to wcf4b.
; Return carry on success.
	ld a, b
	ld [wd11e], a
	ld [wcf91], a
	ld a, c
	ld [wItemQuantity], a
IF _ITEMAPI
	ld hl, LOW(wCurrentItemPage)
ELSE
	ld hl, wNumBagItems
ENDC
	call AddItemToInventory
	ret nc
	call GetItemName
	call CopyStringToCF4B
	scf
	ret

GetItemPrice::
; Stores item's price as BCD at hItemPrice (3 bytes)
; Input: [wcf91] = item id
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [wListMenuID]
	cp MOVESLISTMENU
	ld a, BANK(ItemPrices)
	jr nz, .ok
	ld a, $f ; hardcoded Bank
.ok
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ld hl, wItemPrices
	ld a, [hli]
	ld h, [hl]
	ld l, a
	ld a, [wcf91] ; a contains item id
	cp HM_01
	jr nc, .getTMPrice
	ld bc, $3
.loop
	add hl, bc
	dec a
	jr nz, .loop
	dec hl
	ld a, [hld]
	ld [hItemPrice + 2], a
	ld a, [hld]
	ld [hItemPrice + 1], a
	ld a, [hl]
	ld [hItemPrice], a
	jr .done
.getTMPrice
	ld a, Bank(GetMachinePrice)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call GetMachinePrice
.done
	ld de, hItemPrice
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret
