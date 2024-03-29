LoadTilesetHeader:
	call GetPredefRegisters
	push hl
	ld d, 0
	ld a, [wCurMapTileset]
	add a
	add a
	ld b, a
	add a
	add b ; a = tileset * 12
	jr nc, .noCarry
	inc d
.noCarry
	ld e, a
	ld hl, Tilesets
	add hl, de
	ld de, wTilesetBank
	ld c, $b
.copyTilesetHeaderLoop
	ld a, [hli]
	ld [de], a
	inc de
	dec c
	jr nz, .copyTilesetHeaderLoop
	ld a, [hl]
	ld [hTilesetType], a
	xor a
	ld [hMovingBGTilesCounter1], a
	pop hl
	ld a, [wd736]
	bit 3, a
	jr nz, .skip_same_tileset_check
	ld a, [wCurMapTileset]
	push hl
	push de
	ld hl, DungeonTilesets
	ld de, $1
	call IsInArray
	pop de
	pop hl
	jr c, .skip_same_tileset_check
	ld a, [wCurMapTileset]
	ld b, a
	ld a, [hPreviousTileset]
	cp b
	ret z
.skip_same_tileset_check
	ld a, [wDestinationWarpID]
	cp $ff
	ret z
	call LoadDestinationWarpPosition
	ld a, [wYCoord]
	and $1
	ld [wYBlockCoord], a
	ld a, [wXCoord]
	and $1
	ld [wXBlockCoord], a
	ret

INCLUDE "data/dungeon_tilesets.asm"

INCLUDE "data/tileset_headers.asm"
