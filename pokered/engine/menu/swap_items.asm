HandleItemListSwapping:
	ld a, [wListMenuID]
	cp ITEMLISTMENU
	jr nz, .jump_to_loop ; only rearrange item list menus
	ld a, [wCurrentItemList]
	cp 2
	jr nc, .jump_to_loop ; only rearrange your own bag or PC, not arbitrary lists
	push hl
	ld hl, wListPointer
	ld a, [hli]
	ld h, [hl]
	ld l, a
	inc hl ; hl = beginning of list entries
	ld a, [wCurrentMenuItem]
	ld b, a
	ld a, [wListScrollOffset]
	add b
	add a
	ld c, a
	ld b, 0
	add hl, bc ; hl = address of currently selected item entry
	ld a, [hl]
	pop hl
	inc a
	jr z, .jump_to_loop ; ignore attempts to swap the Cancel menu item
	ld a, [wMenuItemToSwap] ; ID of item chosen for swapping (counts from 1)
	and a ; has the first item to swap already been chosen?
	jr nz, .swapItems
; if not, set the currently selected item as the first item
	ld a, [wCurrentMenuItem]
	inc a
	ld b, a
	ld a, [wListScrollOffset] ; index of top (visible) menu item within the list
	add b
	ld [wMenuItemToSwap], a
	call GetCurrentPageNumber
	ld [wMenuItemPageToSwap], a
	ld c, 20
	call DelayFrames
.jump_to_loop
	callba LoadItemListFromAPI
	jp DisplayListMenuIDLoop

.swapItems
	ld a, [wCurrentMenuItem]
	inc a
	ld b, a
	ld a, [wListScrollOffset]
	add b
	ld b, a
	call GetCurrentPageNumber
	ld c, a
	ld a, [wMenuItemToSwap]
	cp b ; is the currently selected item the same as the first item to swap?
	ld a, [wMenuItemPageToSwap]
	jr nz, .do_swap
	cp c
	jr z, .jump_to_loop ; ignore attempts to swap an item with itself
.do_swap
	ld hl, wItemAPIBuffer
	ld [hli], a
	ld a, [wMenuItemToSwap]
	dec a
	ld [hli], a
	ld a, c
	ld [hli], a
	dec b
	ld [hl], b
	ld a, [wCurrentItemList]
	and a
	ld a, ITEMAPI_SWAP_ITEMS
	jr z, .got_swap_call
	ld a, ITEMAPI_SWAP_PC_ITEMS
.got_swap_call
	call ItemAPI
	jr c, .done
	jr z, .nope
	xor a
	ld [wListScrollOffset], a
	ld [wCurrentMenuItem], a
.done
	xor a
	ld [wMenuItemToSwap], a
	jr .jump_to_loop

.nope
	ld a, SFX_DENIED
	call PlaySound
	call WaitForSoundToFinish
	jr .jump_to_loop
