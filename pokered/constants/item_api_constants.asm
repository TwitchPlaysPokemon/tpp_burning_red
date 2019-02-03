; Unlisted constants (since they refer to responses):
; $00 = false
; $01 = true
; $02 = null
; $03 = error / timeout

const_value = $30
	const ITEMAPI_CAN_GET_ITEM ;(item, quantity, page)
	const ITEMAPI_ADD_ITEM ;(item, quantity, page)
	const ITEMAPI_HAS_ITEM ;(item, quantity, page) - returns page, index, quantity
	const ITEMAPI_REMOVE_ITEM ;(index, quantity, page) - null indicates stack is empty and returns active page and number of items in page

	; completely equivalent to the bag functions, but for the PC
	const ITEMAPI_CAN_GET_PC_ITEM ;(item, quantity, page)
	const ITEMAPI_ADD_ITEM_TO_PC ;(item, quantity, page)
	const ITEMAPI_HAS_ITEM_IN_PC ;(item, quantity, page)
	const ITEMAPI_REMOVE_ITEM_FROM_PC ;(index, quantity, page)

	const ITEMAPI_CAN_DEPOSIT ;(item, quantity, pack page, PC page)
	const ITEMAPI_DEPOSIT ;(item, quantity, pack page, PC page)
	const ITEMAPI_CAN_WITHDRAW ;(item, quantity, pack page, PC page)
	const ITEMAPI_WITHDRAW ;(item, quantity, pack page, PC page)

	const ITEMAPI_SWAP_ITEMS ;(page, first stack, second stack)
	const ITEMAPI_SWAP_PC_ITEMS ;(page, first stack, second stack)

	const ITEMAPI_USE_ITEM ;(item ID)

	const ITEMAPI_GET_PAGE_LIMITS ;(void)

ITEMAPI_GET_PC_PAGE EQU $40
ITEMAPI_GET_PAGE EQU $80
