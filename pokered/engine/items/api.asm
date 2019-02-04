LoadItemListFromAPI::
	ld a, [wCurrentItemList]
	cp 2
	ret nc
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
	ret nz
	xor a
	ld hl, wNumItems
	ld [hli], a
	ld [hl], -1
.no_name
	ld a, "@"
	ld [wItemAPIBuffer], a
	ret

LoadCurrentItemPageLimits::
	ld a, [wCurrentItemList]
	cp 2
	ret nc
	push hl
	push bc
	ld a, ITEMAPI_GET_PAGE_LIMITS
	call ItemAPI
	ld a, 1
	jr c, .got_limit
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
	ld a, c
	ld [wCurrentItemPageLimit], a
	pop bc
	pop hl
	ret

PrintItemPageName::
	ld a, "@"
	ld [wItemAPIBuffer + 12], a
	ld de, wItemAPIBuffer
	coord hl, 6, 2
	ld c, 12
.name_loop
	ld a, [de]
	cp "@"
	jr z, .name_done
	inc de
	ld [hli], a
	dec c
	jr nz, .name_loop
.name_done
	ld a, c
	and a
	ret z
	ld a, "─"
.dash_loop
	ld [hli], a
	dec c
	jr nz, .dash_loop
	ret

GetFilteredItemList::
	; in: de: desired items
	ld hl, wItemAPIBuffer
	push de
	push hl
.copy_loop
	ld a, [de]
	ld [hli], a
	inc de
	and a
	jr nz, .copy_loop
	ld a, ITEMAPI_GET_ITEM_QUANTITIES
	call ItemAPI
	pop hl
	pop de
	ld bc, wFilteredBagItemsCount
	push af
	xor a
	ld [bc], a
	inc bc
	dec a
	ld [bc], a
	pop af
	ret c
	ret z
.check_loop
	ld a, [de]
	and a
	jr z, .done
	ld a, [hli]
	and a
	jr z, .next
	ld a, [de]
	ld [bc], a
	inc bc
	push hl
	ld hl, wFilteredBagItemsCount
	inc [hl]
	pop hl
.next
	inc de
	jr .check_loop

.done
	dec a
	ld [bc], a
	ret
