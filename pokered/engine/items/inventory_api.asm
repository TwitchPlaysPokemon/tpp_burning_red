; function to add an item (in varying quantities) to the player's bag or PC box
; INPUT:
; hl = address of page number: wCurrentItemPage, wCurrentPCItemPage (specific pages), LOW(wCurrentItemPage), LOW(wCurrentPCItemPage) (any page)
; [wcf91] = item ID
; [wItemQuantity] = item quantity
; sets carry flag if successful, unsets carry flag if unsuccessful
AddItemToInventory_:
	push af
	push hl
	push bc
	push de
	push hl
	ld a, h
	and a
	ld a, -1
	jr z, .got_page
	ld a, [hl]
.got_page
	ld hl, wItemAPIBuffer + 2
	ld [hld], a
	ld e, a
	ld a, [wItemQuantity]
	ld [hld], a
	ld d, a
	ld a, [wcf91]
	ld [hl], a
	ld c, a
	pop hl
	ld a, l
	cp LOW(wCurrentPCItemPage)
	ld a, ITEMAPI_CAN_GET_ITEM
	jr nz, .got_kind
	ld a, ITEMAPI_CAN_GET_PC_ITEM
.got_kind
	ld b, a
	call ItemAPI
	jr c, .okay
	jr z, .done
.okay
	ld hl, wItemAPIBuffer + 2
	ld a, e
	ld [hld], a
	ld a, d
	ld [hld], a
	ld a, c
	ld [hld], a
	ld a, b
	inc a
	call ItemAPI
	jr z, .done
	scf
.done
	pop de
	pop bc
	pop hl
	pop af
	ret

; function to remove an item (in varying quantities) from the player's bag or PC box
; INPUT:
; hl = address of page number: wCurrentItemPage, wCurrentPCItemPage (specific pages)
; [wWhichPokemon] = index (within the inventory) of the item to remove
; [wItemQuantity] = quantity to remove
RemoveItemFromInventory_:
	push hl
	ld a, [hl]
	ld hl, wItemAPIBuffer + 2
	ld [hld], a
	ld a, [wItemQuantity]
	ld [hld], a
	ld a, [wWhichPokemon]
	ld [hl], a
	pop hl
	push hl
	ld a, l
	cp LOW(wCurrentPCItemPage)
	ld a, ITEMAPI_REMOVE_ITEM
	jr nz, .got_kind
	ld a, ITEMAPI_REMOVE_ITEM_FROM_PC
.got_kind
	call ItemAPI
	pop hl
	ret nc
	xor a
	ld [wListScrollOffset], a
	ld [wCurrentMenuItem], a
	ld [wBagSavedMenuItem], a
	ld [wSavedListScrollOffset], a
	ld a, [wItemAPIBuffer]
	ld [hl], a
	ld a, [wItemAPIBuffer + 1]
	ld [wListCount], a
	cp 2
	jr c, .got_new_limit
	ld a, 2
.got_new_limit
	ld [wMaxMenuItem], a
	ret
