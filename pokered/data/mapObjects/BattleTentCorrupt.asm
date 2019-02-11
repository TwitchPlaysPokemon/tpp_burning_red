BattleTentCorrupt_Object:
	db $e ; border block

	db 2 ; warps
	warp 2, 19, 0, -1
	warp 3, 19, 0, -1

	db 0 ; signs

	db 0 ; objects

	; warp-to
	warp_to 2, 19, BATTLE_TENT_CORRUPT_WIDTH
	warp_to 3, 19, BATTLE_TENT_CORRUPT_WIDTH
