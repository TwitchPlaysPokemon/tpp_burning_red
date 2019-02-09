LoadFlippedFrontSpriteByMonIndex::
	ld a, 1
	ld [wSpriteFlipped], a

LoadFrontSpriteByMonIndex::
	push hl
	ld a, [wd11e]
	push af
	ld a, [wcf91]
	ld [wd11e], a
	predef IndexToPokedex
	ld hl, wd11e
	ld a, [hl]
	pop bc
	ld [hl], b
	and a
	pop hl
	jr z, .invalidDexNumber ; dex #0 invalid
	cp NUM_POKEMON + 1
	jr c, .validDexNumber   ; dex >#151 invalid
.invalidDexNumber
	ld a, RHYDON ; $1
	ld [wcf91], a
	ret
.validDexNumber
	push hl
	ld de, vFrontPic
	call LoadMonFrontSprite
	pop hl
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, Bank(CopyUncompressedPicToHL)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	xor a
	ld [hStartTileID], a
	call CopyUncompressedPicToHL
	xor a
	ld [wSpriteFlipped], a
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

; uncompresses the front or back sprite of the specified mon
; assumes the corresponding mon header is already loaded
; hl contains offset to sprite pointer ($b for front or $d for back)
UncompressMonSprite::
	ld bc, wMonHeader
	add hl, bc
	ld a, [hli]
	ld [wSpriteInputPtr], a    ; fetch sprite input pointer
	ld a, [hl]
	ld [wSpriteInputPtr+1], a
; define (by index number) the bank that a pokemon's image is in
; index = Mew, bank 1
; index = Kabutops fossil, bank $B
; index = Phancero, bank $E
; index < $1F, bank 9
; $1F ≤ index < $4A, bank $A
; $4A ≤ index < $74, bank $B
; $74 ≤ index < $99, bank $C
; $99 ≤ index,       bank $D
	ld a, [wcf91] ; XXX name for this ram location
	ld b, a
	cp MEW
	ld a, BANK(MewPicFront)
	jr z, .GotBank
	ld a, b
	cp FOSSIL_KABUTOPS
	ld a, BANK(FossilKabutopsPic)
	jr z, .GotBank
	ld a, b
	cp PHANCERO
	ld a, BANK(PhanceroPicFront)
	jr z, .GotBank
	ld a, b
	cp TANGELA + 1
	ld a, BANK(TangelaPicFront)
	jr c, .GotBank
	ld a, b
	cp MOLTRES + 1
	ld a, BANK(MoltresPicFront)
	jr c, .GotBank
	ld a, b
	cp BEEDRILL + 2
	ld a, BANK(BeedrillPicFront)
	jr c, .GotBank
	ld a, b
	cp STARMIE + 1
	ld a, BANK(StarmiePicFront)
	jr c, .GotBank
	ld a, BANK(VictreebelPicFront)
.GotBank
	jp UncompressSpriteData

; de: destination location
LoadMonFrontSprite::
	push de
	ld hl, wMonHFrontSprite - wMonHeader
	call UncompressMonSprite
	ld hl, wMonHSpriteDim
	ld a, [hli]
	ld c, a
	pop de
	; fall through

; postprocesses uncompressed sprite chunks to a 2bpp sprite and loads it into video ram
; calculates alignment parameters to place both sprite chunks in the center of the 7*7 tile sprite buffers
; de: destination location
; a,c:  sprite dimensions (in tiles of 8x8 each)
LoadUncompressedSpriteData::
	push de
	and $f
	ld [H_SPRITEWIDTH], a ; each byte contains 8 pixels (in 1bpp), so tiles=bytes for width
	ld b, a
	ld a, $7
	sub b      ; 7-w
	inc a      ; 8-w
	srl a      ; (8-w)/2     ; horizontal center (in tiles, rounded up)
	ld b, a
	add a
	add a
	add a
	sub b      ; 7*((8-w)/2) ; skip for horizontal center (in tiles)
	ld [H_SPRITEOFFSET], a
	ld a, c
	swap a
	and $f
	ld b, a
	add a
	add a
	add a     ; 8*tiles is height in bytes
	ld [H_SPRITEHEIGHT], a
	ld a, $7
	sub b      ; 7-h         ; skip for vertical center (in tiles, relative to current column)
	ld b, a
	ld a, [H_SPRITEOFFSET]
	add b     ; 7*((8-w)/2) + 7-h ; combined overall offset (in tiles)
	add a
	add a
	add a     ; 8*(7*((8-w)/2) + 7-h) ; combined overall offset (in bytes)
	ld [H_SPRITEOFFSET], a
	xor a
	ld [$4000], a
	ld hl, sSpriteBuffer0
	call ZeroSpriteBuffer   ; zero buffer 0
	ld de, sSpriteBuffer1
	ld hl, sSpriteBuffer0
	call AlignSpriteDataCentered    ; copy and align buffer 1 to 0 (containing the MSB of the 2bpp sprite)
	ld hl, sSpriteBuffer1
	call ZeroSpriteBuffer   ; zero buffer 1
	ld de, sSpriteBuffer2
	ld hl, sSpriteBuffer1
	call AlignSpriteDataCentered    ; copy and align buffer 2 to 1 (containing the LSB of the 2bpp sprite)
	pop de
	jp InterlaceMergeSpriteBuffers

; copies and aligns the sprite data properly inside the sprite buffer
; sprite buffers are 7*7 tiles in size, the loaded sprite is centered within this area
AlignSpriteDataCentered::
	ld a, [H_SPRITEOFFSET]
	ld b, $0
	ld c, a
	add hl, bc
	ld a, [H_SPRITEWIDTH]
.columnLoop
	push af
	push hl
	ld a, [H_SPRITEHEIGHT]
	ld c, a
.columnInnerLoop
	ld a, [de]
	inc de
	ld [hli], a
	dec c
	jr nz, .columnInnerLoop
	pop hl
	ld bc, 7*8    ; 7 tiles
	add hl, bc    ; advance one full column
	pop af
	dec a
	jr nz, .columnLoop
	ret

; fills the sprite buffer (pointed to in hl) with zeros
ZeroSpriteBuffer::
	ld bc, SPRITEBUFFERSIZE
.nextByteLoop
	xor a
	ld [hli], a
	dec bc
	ld a, b
	or c
	jr nz, .nextByteLoop
	ret

; combines the (7*7 tiles, 1bpp) sprite chunks in buffer 0 and 1 into a 2bpp sprite located in buffer 1 through 2
; in the resulting sprite, the rows of the two source sprites are interlaced
; de: output address
InterlaceMergeSpriteBuffers::
	xor a
	ld [$4000], a
	push de
	ld hl, sSpriteBuffer2 + (SPRITEBUFFERSIZE - 1) ; destination: end of buffer 2
	ld de, sSpriteBuffer1 + (SPRITEBUFFERSIZE - 1) ; source 2: end of buffer 1
	ld bc, sSpriteBuffer0 + (SPRITEBUFFERSIZE - 1) ; source 1: end of buffer 0
	ld a, SPRITEBUFFERSIZE/2 ; $c4
	ld [H_SPRITEINTERLACECOUNTER], a
.interlaceLoop
	ld a, [de]
	dec de
	ld [hld], a   ; write byte of source 2
	ld a, [bc]
	dec bc
	ld [hld], a   ; write byte of source 1
	ld a, [de]
	dec de
	ld [hld], a   ; write byte of source 2
	ld a, [bc]
	dec bc
	ld [hld], a   ; write byte of source 1
	ld a, [H_SPRITEINTERLACECOUNTER]
	dec a
	ld [H_SPRITEINTERLACECOUNTER], a
	jr nz, .interlaceLoop
	ld a, [wSpriteFlipped]
	and a
	jr z, .notFlipped
	ld bc, 2*SPRITEBUFFERSIZE
	ld hl, sSpriteBuffer1
.swapLoop
	swap [hl]    ; if flipped swap nybbles in all bytes
	inc hl
	dec bc
	ld a, b
	or c
	jr nz, .swapLoop
.notFlipped
	pop hl
	ld de, sSpriteBuffer1
	ld c, (2*SPRITEBUFFERSIZE)/16 ; $31, number of 16 byte chunks to be copied
	ld a, [H_LOADEDROMBANK]
	ld b, a
	jp CopyVideoData

UpdateSprites::
	ld a, [wUpdateSpritesEnabled]
	dec a
	ret nz
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, Bank(_UpdateSprites)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call _UpdateSprites
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	ret

ResetPlayerSpriteData::
	ld hl, wSpriteStateData1
	call ResetPlayerSpriteData_ClearSpriteData
	ld hl, wSpriteStateData2
	call ResetPlayerSpriteData_ClearSpriteData
	ld a, $1
	ld [wSpriteStateData1], a
	ld [wSpriteStateData2 + $0e], a
	ld hl, wSpriteStateData1 + 4
	ld [hl], $3c     ; set Y screen pos
	inc hl
	inc hl
	ld [hl], $40     ; set X screen pos
	ret

; overwrites sprite data with zeroes
ResetPlayerSpriteData_ClearSpriteData::
	ld bc, $10
	xor a
	jp FillMemory

GetSpritePosition1::
	ld hl, _GetSpritePosition1
	jr SpritePositionBankswitch

GetSpritePosition2::
	ld hl, _GetSpritePosition2
	jr SpritePositionBankswitch

SetSpritePosition1::
	ld hl, _SetSpritePosition1
	jr SpritePositionBankswitch

SetSpritePosition2::
	ld hl, _SetSpritePosition2
SpritePositionBankswitch::
	ld b, BANK(_GetSpritePosition1) ; BANK(_GetSpritePosition2), BANK(_SetSpritePosition1), BANK(_SetSpritePosition2)
	jp Bankswitch ; indirect jump to one of the four functions

SetSpriteFacingDirectionAndDelay::
	call SetSpriteFacingDirection
	ld c, 6
	rst DelayFrames
	ret

SetSpriteFacingDirection::
	ld a, $9
	ld [H_SPRITEDATAOFFSET], a
	call GetPointerWithinSpriteStateData1
	ld a, [hSpriteFacingDirection]
	ld [hl], a
	ret

SetSpriteImageIndexAfterSettingFacingDirection::
	ld de, -7
	add hl, de
	ld [hl], a
	ret

GetPointerWithinSpriteStateData1::
	ld h, $c1
	jr _GetPointerWithinSpriteStateData

GetPointerWithinSpriteStateData2::
	ld h, $c2

_GetPointerWithinSpriteStateData:
	ld a, [H_SPRITEDATAOFFSET]
	ld b, a
	ld a, [H_SPRITEINDEX]
	swap a
	add b
	ld l, a
	ret

; sets movement byte 1 for sprite [H_SPRITEINDEX] to $FE and byte 2 to [hSpriteMovementByte2]
SetSpriteMovementBytesToFE::
	push hl
	call GetSpriteMovementByte1Pointer
	ld [hl], $fe
	call GetSpriteMovementByte2Pointer
	ld a, [hSpriteMovementByte2]
	ld [hl], a
	pop hl
	ret

; sets both movement bytes for sprite [H_SPRITEINDEX] to $FF
SetSpriteMovementBytesToFF::
	push hl
	call GetSpriteMovementByte1Pointer
	ld [hl], $FF
	call GetSpriteMovementByte2Pointer
	ld [hl], $FF ; prevent person from walking?
	pop hl
	ret

; returns the sprite movement byte 1 pointer for sprite [H_SPRITEINDEX] in hl
GetSpriteMovementByte1Pointer::
	ld h, $C2
	ld a, [H_SPRITEINDEX]
	swap a
	add 6
	ld l, a
	ret

; returns the sprite movement byte 2 pointer for sprite [H_SPRITEINDEX] in hl
GetSpriteMovementByte2Pointer::
	push de
	ld hl, wMapSpriteData
	ld a, [H_SPRITEINDEX]
	dec a
	add a
	ld d, 0
	ld e, a
	add hl, de
	pop de
	ret

; Copy the current map's sprites' tile patterns to VRAM again after they have
; been overwritten by other tile patterns.
ReloadMapSpriteTilePatterns::
	ld hl, wFontLoaded
	ld a, [hl]
	push af
	res 0, [hl]
	push hl
	xor a
	ld [wSpriteSetID], a
	call DisableLCD
	callba InitMapSprites
	call EnableLCD
	pop hl
	pop af
	ld [hl], a
	call LoadPlayerSpriteGraphics
	call LoadFontTilePatterns
	jp UpdateSprites

MoveSprite::
; move the sprite [H_SPRITEINDEX] with the movement pointed to by de
; actually only copies the movement data to wNPCMovementDirections for later
	call SetSpriteMovementBytesToFF
MoveSprite_::
	push hl
	push bc
	call GetSpriteMovementByte1Pointer
	xor a
	ld [hl], a
	ld hl, wNPCMovementDirections
	ld c, 0

.loop
	ld a, [de]
	ld [hli], a
	inc de
	inc c
	cp $FF ; have we reached the end of the movement data?
	jr nz, .loop

	ld a, c
	ld [wNPCNumScriptedSteps], a ; number of steps taken

	pop bc
	ld hl, wd730
	set 0, [hl]
	pop hl
	xor a
	ld [wOverrideSimulatedJoypadStatesMask], a
	ld [wSimulatedJoypadStatesEnd], a
	dec a
	ld [wJoyIgnore], a
	ld [wWastedByteCD3A], a
	ret
