GetQuantityOfItemInBag:
; In: b = item ID
; Out: b = how many of that item are in the bag
	call GetPredefRegisters
	ld hl, wItemAPIBuffer
	ld a, b
	ld [hli], a
	ld a, 1
	ld [hli], a
	ld [hl], -1
	ld a, ITEMAPI_HAS_ITEM
	call ItemAPI
	jr c, .nope
	jr z, .nope
	ld a, [wItemAPIBuffer + 2]
	ld b, a
	ret

.nope
	ld b, 0
	ret
