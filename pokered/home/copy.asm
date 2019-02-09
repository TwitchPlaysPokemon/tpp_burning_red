FarCopyData2::
; Identical to FarCopyData, but uses hROMBankTemp
; as temp space instead of wBuffer.
	ld [hROMBankTemp], a
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [hROMBankTemp]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call CopyData
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

FarCopyData3::
; Copy bc bytes from a:de to hl.
	ld [hROMBankTemp], a
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [hROMBankTemp]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	push hl
	push de
	push de
	ld d, h
	ld e, l
	pop hl
	call CopyData
	pop de
	pop hl
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

FarCopyDataDouble::
; Expand bc bytes of 1bpp image data
; from a:hl to 2bpp data at de.
	ld [hROMBankTemp], a
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, [hROMBankTemp]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
.loop
	ld a, [hli]
	ld [de], a
	inc de
	ld [de], a
	inc de
	dec bc
	ld a, c
	or b
	jr nz, .loop
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

CopyVideoData::
; Wait for the next VBlank, then copy c 2bpp
; tiles from b:de to hl, 8 tiles at a time.
; This takes c/8 frames.

	ld a, [H_AUTOBGTRANSFERENABLED]
	push af
	xor a ; disable auto-transfer while copying
	ld [H_AUTOBGTRANSFERENABLED], a

	ld a, [H_LOADEDROMBANK]
	ld [hROMBankTemp], a

	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a

	ld a, e
	ld [H_VBCOPYSRC], a
	ld a, d
	ld [H_VBCOPYSRC + 1], a

	ld a, l
	ld [H_VBCOPYDEST], a
	ld a, h
	ld [H_VBCOPYDEST + 1], a

.loop
	ld a, c
	cp 8
	jr nc, .keepgoing

.done
	ld [H_VBCOPYSIZE], a
	call DelayFrame
	ld a, [hROMBankTemp]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	pop af
	ld [H_AUTOBGTRANSFERENABLED], a
	ret

.keepgoing
	ld a, 8
	ld [H_VBCOPYSIZE], a
	call DelayFrame
	ld a, c
	sub 8
	ld c, a
	jr .loop

CopyVideoDataDouble::
; Wait for the next VBlank, then copy c 1bpp
; tiles from b:de to hl, 8 tiles at a time.
; This takes c/8 frames.
	ld a, [H_AUTOBGTRANSFERENABLED]
	push af
	xor a ; disable auto-transfer while copying
	ld [H_AUTOBGTRANSFERENABLED], a
	ld a, [H_LOADEDROMBANK]
	ld [hROMBankTemp], a

	ld a, b
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a

	ld a, e
	ld [H_VBCOPYDOUBLESRC], a
	ld a, d
	ld [H_VBCOPYDOUBLESRC + 1], a

	ld a, l
	ld [H_VBCOPYDOUBLEDEST], a
	ld a, h
	ld [H_VBCOPYDOUBLEDEST + 1], a

.loop
	ld a, c
	cp 8
	jr nc, .keepgoing

.done
	ld [H_VBCOPYDOUBLESIZE], a
	call DelayFrame
	ld a, [hROMBankTemp]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	pop af
	ld [H_AUTOBGTRANSFERENABLED], a
	ret

.keepgoing
	ld a, 8
	ld [H_VBCOPYDOUBLESIZE], a
	call DelayFrame
	ld a, c
	sub 8
	ld c, a
	jr .loop

ClearScreenArea::
; Clear tilemap area cxb at hl.
	ld a, " " ; blank tile
	ld de, 20 ; screen width
.y
	push hl
	push bc
.x
	ld [hli], a
	dec c
	jr nz, .x
	pop bc
	pop hl
	add hl, de
	dec b
	jr nz, .y
	ret

CopyScreenTileBufferToVRAM::
; Copy wTileMap to the BG Map starting at b * $100.
; This is done in thirds of 6 rows, so it takes 3 frames.

	ld c, 6

	ld hl, $600 * 0
	coord de, 0, 6 * 0
	call .setup
	call DelayFrame

	ld hl, $600 * 1
	coord de, 0, 6 * 1
	call .setup
	call DelayFrame

	ld hl, $600 * 2
	coord de, 0, 6 * 2
	call .setup
	jp DelayFrame

.setup
	ld a, d
	ld [H_VBCOPYBGSRC+1], a
	call GetRowColAddressBgMap
	ld a, l
	ld [H_VBCOPYBGDEST], a
	ld a, h
	ld [H_VBCOPYBGDEST+1], a
	ld a, c
	ld [H_VBCOPYBGNUMROWS], a
	ld a, e
	ld [H_VBCOPYBGSRC], a
	ret

ClearScreen::
; Clear wTileMap, then wait
; for the bg map to update.
	ld bc, 20 * 18
	inc b
	coord hl, 0, 0
	ld a, " "
.loop
	ld [hli], a
	dec c
	jr nz, .loop
	dec b
	jr nz, .loop
	jp Delay3

FillMemory::
; Fill bc bytes at hl with a.
	push de
	ld d, a
.loop
	ld a, d
	ld [hli], a
	dec bc
	ld a, b
	or c
	jr nz, .loop
	pop de
	ret

; copies a string from [de] to [wcf4b]
CopyStringToCF4B::
	ld hl, wcf4b
	; fall through

; copies a string from [de] to [hl]
CopyString::
	ld a, [de]
	inc de
	ld [hli], a
	cp "@"
	jr nz, CopyString
	ret

; Copies [hl, bc) to [de, bc - hl).
; In other words, the source data is from hl up to but not including bc,
; and the destination is de.
CopyDataUntil::
	ld a, [hli]
	ld [de], a
	inc de
	ld a, h
	cp b
	jr nz, CopyDataUntil
	ld a, l
	cp c
	jr nz, CopyDataUntil
	ret

; Compare strings, c bytes in length, at de and hl.
; Often used to compare big endian numbers in battle calculations.
StringCmp::
	ld a, [de]
	cp [hl]
	ret nz
	inc de
	inc hl
	dec c
	jr nz, StringCmp
	ret
