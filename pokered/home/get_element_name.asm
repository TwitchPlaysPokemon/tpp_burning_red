GetMonName::
	push hl
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, BANK(MonsterNames)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ld a, [wd11e]
	dec a
	ld hl, MonsterNames
	ld c, 10
	ld b, 0
	call AddNTimes
	ld de, wcd6d
	push de
	ld bc, 10
	call CopyData
	ld hl, wcd6d + 10
	ld [hl], "@"
	pop de
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	pop hl
	ret

GetItemName::
; given an item ID at [wd11e], store the name of the item into a string
;     starting at wcd6d
	push hl
	push bc
	ld a, [wd11e]
	cp HM_01 ; is this a TM/HM?
	jr nc, .Machine

	ld [wd0b5], a
	ld a, ITEM_NAME
	ld [wNameListType], a
	ld a, BANK(ItemNames)
	ld [wPredefBank], a
	call GetName
	jr .Finish

.Machine
	call GetMachineName
.Finish
	ld de, wcd6d ; pointer to where item name is stored in RAM
	pop bc
	pop hl
	ret

GetMachineName::
; copies the name of the TM/HM in [wd11e] to wcd6d
	push hl
	push de
	push bc
	ld a, [wd11e]
	push af
	cp TM_01 ; is this a TM? [not HM]
	jr nc, .WriteTM
; if HM, then write "HM" and add 5 to the item ID, so we can reuse the
; TM printing code
	add 5
	ld [wd11e], a
	ld hl, HiddenPrefix ; points to "HM"
	ld bc, 2
	jr .WriteMachinePrefix
.WriteTM
	ld hl, TechnicalPrefix ; points to "TM"
	ld bc, 2
.WriteMachinePrefix
	ld de, wcd6d
	call CopyData

; now get the machine number and convert it to text
	ld a, [wd11e]
	sub TM_01 - 1
	ld b, "0"
.FirstDigit
	sub 10
	jr c, .SecondDigit
	inc b
	jr .FirstDigit
.SecondDigit
	add 10
	push af
	ld a, b
	ld [de], a
	inc de
	pop af
	ld b, "0"
	add b
	ld [de], a
	inc de
	ld a, "@"
	ld [de], a
	pop af
	ld [wd11e], a
	pop bc
	pop de
	pop hl
	ret

TechnicalPrefix::
	db "TM"
HiddenPrefix::
	db "HM"

NamePointers::
	dw MonsterNames
	dw MoveNames
	dw UnusedNames
	dw ItemNames
	dw wPartyMonOT ; player's OT names list
	dw wEnemyMonOT ; enemy's OT names list
	dw TrainerNames

GetName::
; arguments:
; [wd0b5] = which name
; [wNameListType] = which list
; [wPredefBank] = bank of list
;
; returns pointer to name in de
	ld a, [wd0b5]
	ld [wd11e], a

	; TM names are separate from item names.
	; BUG: This applies to all names instead of just items.
	cp HM_01
	jp nc, GetMachineName

	ld a, [H_LOADEDROMBANK]
	push af
	push hl
	push bc
	push de
	ld a, [wNameListType]    ;List3759_entrySelector
	dec a
	jr nz, .otherEntries
	;1 = MON_NAMES
	call GetMonName
	ld hl, NAME_LENGTH
	add hl, de
	ld e, l
	ld d, h
	jr .gotPtr
.otherEntries
	;2-7 = OTHER ENTRIES
	ld a, [wPredefBank]
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ld a, [wNameListType]    ;VariousNames' entryID
	dec a
	add a
	ld d, 0
	ld e, a
	jr nc, .skip
	inc d
.skip
	ld hl, NamePointers
	add hl, de
	ld a, [hli]
	ld [$ff96], a
	ld a, [hl]
	ld [$ff95], a
	ld a, [$ff95]
	ld h, a
	ld a, [$ff96]
	ld l, a
	ld a, [wd0b5]
	ld b, a
	ld c, 0
.nextName
	ld d, h
	ld e, l
.nextChar
	ld a, [hli]
	cp "@"
	jr nz, .nextChar
	inc c           ;entry counter
	ld a, b          ;wanted entry
	cp c
	jr nz, .nextName
	ld h, d
	ld l, e
	ld de, wcd6d
	ld bc, $0014
	call CopyData
.gotPtr
	ld a, e
	ld [wUnusedCF8D], a
	ld a, d
	ld [wUnusedCF8D + 1], a
	pop de
	pop bc
	pop hl
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret
