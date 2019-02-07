PlayerPC:
	ld hl, wd730
	set 6, [hl]
	ld a, ITEM_NAME
	ld [wNameListType], a
	call SaveScreenTilesToBuffer1
	xor a
	ld [wBagSavedMenuItem], a
	ld [wParentMenuItem], a
	ld a, [wFlags_0xcd60]
	bit 3, a ; accessing player's PC through another PC?
	jr nz, PlayerPCMenu
; accessing it directly
	ld a, SFX_TURN_ON_PC
	call PlaySound
	ld hl, TurnedOnPC2Text
	call PrintText

PlayerPCMenu:
	ld a, [wParentMenuItem]
	ld [wCurrentMenuItem], a
	ld hl, wFlags_0xcd60
	set 5, [hl]
	call LoadScreenTilesFromBuffer2
	coord hl, 0, 0
	lb bc, 8, 14
	call TextBoxBorder
	call UpdateSprites
	coord hl, 2, 2
	ld de, PlayersPCMenuEntries
	call PlaceString
	ld hl, wTopMenuItemY
	ld a, 2
	ld [hli], a ; wTopMenuItemY
	dec a
	ld [hli], a ; wTopMenuItemX
	inc hl
	inc hl
	ld a, 3
	ld [hli], a ; wMaxMenuItem
	ld a, A_BUTTON | B_BUTTON
	ld [hli], a ; wMenuWatchedKeys
	xor a
	ld [hl], a
	ld hl, wListScrollOffset
	ld [hli], a ; wListScrollOffset
	ld [hl], a ; wMenuWatchMovingOutOfBounds
	ld [wPlayerMonNumber], a
	ld hl, WhatDoYouWantText
	call PrintText
	call HandleMenuInput
	bit 1, a
	jp nz, ExitPlayerPC
	call PlaceUnfilledArrowMenuCursor
	ld a, [wCurrentMenuItem]
	ld [wParentMenuItem], a
	and a
	jp z, PlayerPCWithdraw
	dec a
	jp z, PlayerPCDeposit
	dec a
	jp z, PlayerPCToss

ExitPlayerPC:
	ld a, [wFlags_0xcd60]
	bit 3, a ; accessing player's PC through another PC?
	jr nz, .next
; accessing it directly
	ld a, SFX_TURN_OFF_PC
	call PlaySound
	call WaitForSoundToFinish
.next
	ld hl, wFlags_0xcd60
	res 5, [hl]
	call LoadScreenTilesFromBuffer2
	xor a
	ld [wListScrollOffset], a
	ld [wBagSavedMenuItem], a
	ld hl, wd730
	res 6, [hl]
	xor a
	ld [wDoNotWaitForButtonPressAfterDisplayingText], a
	ret

PlayerPCDeposit:
	xor a
	ld [wCurrentMenuItem], a
	ld [wListScrollOffset], a
IF _ITEMAPI
	ld [wCurrentItemPage], a
	ld a, ITEMAPI_IS_BAG_EMPTY
	call ItemAPI
	jr c, .loop
	jr z, .loop
ELSE
	ld a, [wNumBagItems]
	and a
	jr nz, .loop
ENDC
	ld hl, NothingToDepositText
	call PrintText
	jp PlayerPCMenu
.loop
	ld hl, WhatToDepositText
	call PrintText
	ld a, LOW(wNumItems)
	ld [wListPointer], a
	ld a, HIGH(wNumItems)
	ld [wListPointer + 1], a
	xor a
	ld [wPrintItemPrices], a
IF _ITEMAPI
	ld [wCurrentItemList], a
ENDC
	ld a, ITEMLISTMENU
	ld [wListMenuID], a
	call DisplayListMenuID
	jp c, PlayerPCMenu
	call IsKeyItem
	ld a, 1
	ld [wItemQuantity], a
	ld a, [wIsKeyItem]
	and a
	jr nz, .next
; if it's not a key item, there can be more than one of the item
	ld hl, DepositHowManyText
	call PrintText
	call DisplayChooseQuantityMenu
	cp $ff
	jr z, .loop
.next
IF _ITEMAPI
	ld hl, wItemAPIBuffer
	ld a, [wCurrentItemPage]
	ld [hli], a
	ld a, [wWhichPokemon]
	ld [hli], a
	ld a, [wItemQuantity]
	ld [hl], a
	ld a, ITEMAPI_DEPOSIT
	call ItemAPI
	jr c, .loop
	ld hl, NoRoomToStoreText
	jr z, .done
ELSE
	ld hl, wNumBoxItems
	call AddItemToInventory
	ld hl, NoRoomToStoreText
	jr nc, .done
	ld hl, wNumBagItems
	call RemoveItemFromInventory
ENDC
	call WaitForSoundToFinish
	ld a, SFX_WITHDRAW_DEPOSIT
	call PlaySound
	call WaitForSoundToFinish
	ld hl, ItemWasStoredText
.done
	call PrintText
	jr .loop

PlayerPCWithdraw:
	xor a
	ld [wCurrentMenuItem], a
	ld [wListScrollOffset], a
IF _ITEMAPI
	ld [wCurrentPCItemPage], a
	ld a, ITEMAPI_IS_PC_EMPTY
	call ItemAPI
	jr c, .loop
	jr z, .loop
ELSE
	ld a, [wNumBoxItems]
	and a
	jr nz, .loop
ENDC
	ld hl, NothingStoredText
	call PrintText
	jp PlayerPCMenu
.loop
	ld hl, WhatToWithdrawText
	call PrintText
	ld a, LOW(wNumItems)
	ld [wListPointer], a
	ld a, HIGH(wNumItems)
	ld [wListPointer + 1], a
	xor a
	ld [wPrintItemPrices], a
IF _ITEMAPI
	inc a
	ld [wCurrentItemList], a
ENDC
	ld a, ITEMLISTMENU
	ld [wListMenuID], a
	call DisplayListMenuID
	jp c, PlayerPCMenu
	call IsKeyItem
	ld a, 1
	ld [wItemQuantity], a
	ld a, [wIsKeyItem]
	and a
	jr nz, .next
; if it's not a key item, there can be more than one of the item
	ld hl, WithdrawHowManyText
	call PrintText
	call DisplayChooseQuantityMenu
	cp $ff
	jr z, .loop
.next
IF _ITEMAPI
	ld hl, wItemAPIBuffer
	ld a, [wCurrentPCItemPage]
	ld [hli], a
	ld a, [wWhichPokemon]
	ld [hli], a
	ld a, [wItemQuantity]
	ld [hl], a
	ld a, ITEMAPI_WITHDRAW
	call ItemAPI
	jr c, .loop
	ld hl, CantCarryMoreText
	jr z, .done
ELSE
	ld hl, wNumBagItems
	call AddItemToInventory
	ld hl, CantCarryMoreText
	jr nc, .done
	ld hl, wNumBoxItems
	call RemoveItemFromInventory
ENDC
	call WaitForSoundToFinish
	ld a, SFX_WITHDRAW_DEPOSIT
	call PlaySound
	call WaitForSoundToFinish
	ld hl, WithdrewItemText
.done
	call PrintText
	jr .loop

PlayerPCToss:
	xor a
	ld [wCurrentMenuItem], a
	ld [wListScrollOffset], a
IF _ITEMAPI
	ld [wCurrentPCItemPage], a
	ld a, ITEMAPI_IS_PC_EMPTY
	call ItemAPI
	jr c, .loop
	jr z, .loop
ELSE
	ld a, [wNumItems]
	and a
	jr nz, .loop
ENDC
	ld hl, NothingStoredText
	call PrintText
	jp PlayerPCMenu
.loop
	ld hl, WhatToTossText
	call PrintText
	ld a, LOW(wNumItems)
	ld [wListPointer], a
	ld a, HIGH(wNumItems)
	ld [wListPointer + 1], a
	xor a
	ld [wPrintItemPrices], a
IF _ITEMAPI
	inc a
	ld [wCurrentItemList], a
ENDC
	ld a, ITEMLISTMENU
	ld [wListMenuID], a
	push hl
	call DisplayListMenuID
	pop hl
	jp c, PlayerPCMenu
	push hl
	call IsKeyItem
	pop hl
	ld a, 1
	ld [wItemQuantity], a
	ld a, [wIsKeyItem]
	and a
	jr nz, .next
	ld a, [wcf91]
	call IsItemHM
	jr c, .next
; if it's not a key item, there can be more than one of the item
	push hl
	ld hl, TossHowManyText
	call PrintText
	call DisplayChooseQuantityMenu
	pop hl
	cp $ff
	jr z, .loop
.next
IF _ITEMAPI
	ld hl, wCurrentPCItemPage
ENDC
	call TossItem ; disallows tossing key items
	jr .loop

PlayersPCMenuEntries:
	db   "WITHDRAW ITEM"
	next "DEPOSIT ITEM"
	next "TOSS ITEM"
	next "LOG OFF@"

TurnedOnPC2Text:
	TX_FAR _TurnedOnPC2Text
	db "@"

WhatDoYouWantText:
	TX_FAR _WhatDoYouWantText
	db "@"

WhatToDepositText:
	TX_FAR _WhatToDepositText
	db "@"

DepositHowManyText:
	TX_FAR _DepositHowManyText
	db "@"

ItemWasStoredText:
	TX_FAR _ItemWasStoredText
	db "@"

NothingToDepositText:
	TX_FAR _NothingToDepositText
	db "@"

NoRoomToStoreText:
	TX_FAR _NoRoomToStoreText
	db "@"

WhatToWithdrawText:
	TX_FAR _WhatToWithdrawText
	db "@"

WithdrawHowManyText:
	TX_FAR _WithdrawHowManyText
	db "@"

WithdrewItemText:
	TX_FAR _WithdrewItemText
	db "@"

NothingStoredText:
	TX_FAR _NothingStoredText
	db "@"

CantCarryMoreText:
	TX_FAR _CantCarryMoreText
	db "@"

WhatToTossText:
	TX_FAR _WhatToTossText
	db "@"

TossHowManyText:
	TX_FAR _TossHowManyText
	db "@"
