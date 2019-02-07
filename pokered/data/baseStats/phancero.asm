PhanceroBaseStats:
db DEX_PHANCERO ; pokedex id
db 178 ; base hp
db 137 ; base attack
db 57  ; base defense
db 128 ; base speed
db 65  ; base special
db GHOST  ; species type 1
db FLYING ; species type 2
db 3   ; catch rate
db 216 ; base exp yield
INCBIN "pic/bmon/phancero.pic",0,1 ; 77, sprite dimensions
dw PhanceroPicFront
dw PhanceroPicBack
; attacks known at lvl 0
db POUND
db 0
db 0
db 0
db 5 ; growth rate
; learnset
	tmlearn 3,6
	tmlearn 10
	tmlearn 18,19
	tmlearn 25,26,29,32
	tmlearn 0
	tmlearn 43,44,45
	tmlearn 49,50,52,56
db 0 ; padding
