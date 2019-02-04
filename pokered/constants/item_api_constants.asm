; Unlisted constants (since they refer to responses):
; $00 = false
; $01 = true
; $02 = null
; $03 = error / timeout

const_value = $04
	const ITEMAPI_LOCK ;(void)
	const ITEMAPI_UNLOCK ;(key) - key = "InitItemAPI@"

	const ITEMAPI_IS_BAG_EMPTY ;(void)

const_value = $10
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

	const ITEMAPI_SWAP_ITEMS ;(first page, first stack, second page, second stack) - false: cannot swap, true: swapped, null: no swap (e.g., same stacks)
	const ITEMAPI_SWAP_PC_ITEMS ;(first page, first stack, second page, second stack)

	const ITEMAPI_INITIALIZE_ITEM_LISTS ;(void)

	const ITEMAPI_GET_PAGE_LIMITS ;(void) - returns max page, max PC page

; return name in buffer, contents in wItems; false = no items, true = OK, null = items but w/o name
ITEMAPI_GET_PC_PAGE EQU $40
ITEMAPI_GET_PAGE EQU $80
