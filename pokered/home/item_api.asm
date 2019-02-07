ItemAPI::
	push hl
	push af
	ld hl, wItemAPICommand
.call_loop
	ld [hl], a
	ld a, 3
.wait_loop
	cp [hl]
	jr c, .wait_loop ;busy wait so we can do subframe API calls
	jr nz, .done

	; timeout
	call DelayFrame
	call DelayFrame ;2 frames hoping that the callee will update its state in the skipped frame...
	pop af
	push af
	jr .call_loop

.done
	; carry if null (2), zero if false (0), nz/nc if true (1)
	ld a, [hl]
	cp 2
	scf
	jr z, .return
	and a
.return
	pop hl
	ld a, h
	pop hl
	ret

UpdateCurrentItemPage::
	; intentionally returns wCurrentItemPage in hl
	ld hl, wCurrentItemPage
	cp [hl]
	ret z
	ld [hl], a
	xor a
	ld [wListScrollOffset], a
	ld [wCurrentMenuItem], a
	ld [wBagSavedMenuItem], a
	ld [wSavedListScrollOffset], a
	ret
