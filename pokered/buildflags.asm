; _BUILD_FLAGS values:
; bit 0: ROM version (0: Red, 1: Blue)
; bit 1: player gender (0: boy, 1: girl)
; bit 2: item API enabled (0: no, 1: yes)

IF !DEF(_BUILD_FLAGS)
_BUILD_FLAGS EQU 0
ENDC

_RED EQU (_BUILD_FLAGS & 1) == 0
_BLUE EQU (_BUILD_FLAGS & 1) == 1
_GREEN EQU 0 ;we don't build Green

_GIRL EQU (_BUILD_FLAGS & 2) == 2

_ITEMAPI EQU (_BUILD_FLAGS & 4) == 4
