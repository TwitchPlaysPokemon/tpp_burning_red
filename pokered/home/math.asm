; calculates the difference |a-b|, setting carry flag if a<b
CalcDifference::
	sub b
	ret nc
	cpl
	add $1
	scf
	ret

; divides [hDividend2] by [hDivisor2] and stores the quotient in [hQuotient2]
DivideBytes::
	push hl
	ld hl, hQuotient2
	xor a
	ld [hld], a
	ld a, [hld]
	and a
	jr z, .done
	ld a, [hli]
.loop
	sub [hl]
	jr c, .done
	inc hl
	inc [hl]
	dec hl
	jr .loop
.done
	pop hl
	ret

; function to do multiplication
; all values are big endian
; INPUT
; FF96-FF98 =  multiplicand
; FF99 = multiplier
; OUTPUT
; FF95-FF98 = product
Multiply::
	push hl
	push bc
	callab _Multiply
	pop bc
	pop hl
	ret

; function to do division
; all values are big endian
; INPUT
; FF95-FF98 = dividend
; FF99 = divisor
; b = number of bytes in the dividend (starting from FF95)
; OUTPUT
; FF95-FF98 = quotient
; FF99 = remainder
Divide::
	push hl
	push de
	push bc
	ld a, [H_LOADEDROMBANK]
	push af
	ld a, Bank(_Divide)
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	call _Divide
	pop af
	ld [H_LOADEDROMBANK], a
	ld [MBC1RomBank], a
	pop bc
	pop de
	pop hl
	ret

AddNTimes::
; add bc to hl a times
	and a
	ret z
.loop
	add hl, bc
	dec a
	jr nz, .loop
	ret
