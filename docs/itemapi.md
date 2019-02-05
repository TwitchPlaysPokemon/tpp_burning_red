# Item API documentation

This page documents the API used by the modified Pokémon Red ROM in order to execute inventory-related actions and/or
retrieve data about the inventory state.

**Note that the ROM that uses the API no longer tracks its own inventory data.** This means that the ROM fully depends
on the API in order to perform all inventory-related tasks. The API will have to keep track of the inventory and will
be informed when the player gains or loses items, deposits them, withdraws them, swaps them, etc.

* [Basics](#basics)
* [Regular API calls](#regular-api-calls)
    * [`LOCK`](#lock)
    * [`UNLOCK`](#unlock)
    * [`INITIALIZE_ITEM_LISTS`](#initialize_item_lists)
    * [`CAN_GET_ITEM`](#can_get_item)
    * [`ADD_ITEM`](#add_item)
    * [`HAS_ITEM`](#has_item)
    * [`REMOVE_ITEM`](#remove_item)
    * [`CAN_GET_PC_ITEM`](#can_get_pc_item)
    * [`ADD_ITEM_TO_PC`](#add_item_to_pc)
    * [`HAS_ITEM_IN_PC`](#has_item_in_pc)
    * [`REMOVE_ITEM_FROM_PC`](#remove_item_from_pc)
    * [`DEPOSIT`](#deposit)
    * [`WITHDRAW`](#withdraw)
    * [`SWAP_ITEMS`](#swap_items)
    * [`SWAP_PC_ITEMS`](#swap_pc_items)
    * [`IS_BAG_EMPTY`](#is_bag_empty)
    * [`IS_PC_EMPTY`](#is_pc_empty)
    * [`GET_ITEM_QUANTITIES`](#get_item_quantities)
    * [`GET_PAGE_LIMITS`](#get_page_limits)
* [Inventory-loading API calls](#inventory-loading-api-calls)

## Basics

The modified ROM no longer tracks its own inventory. This means that the program that responds to API calls (hereafter
the "controller") will have to track all of the data that the game usually tracks on its own. This means tracking both
the contents of every item slot and their ordering (since the game expects the order of the item pack to not change
spontaneously while menus are open). Up to 64 pages of items and 32 pages of PC items are supported; each page can
have a name (up to 12 characters) and up to 60 stacks of items.

The game will issue API calls for any item-related functions that it might need to perform externally, such as getting
an item or loading the item list. Communication with the API uses three locations in memory:

* `wItemAPICommand`, which is a one-byte buffer that the game writes to make an API call, to which the API should
  write to return a result to the game;
* `wItemAPIBuffer`, a 16-byte buffer that comes right after `wItemAPICommand`, used for parameters and additional
  return values;
* `wNumItems`, which is the beginning of a 122-byte buffer used only for specific commands that load an item page.

When the game intends to make an API call, it will load the parameters into `wItemAPIBuffer` (if any), and then write
the API function ID to `wItemAPICommand`. All function IDs are greater than 3. When a value greater than 3 is written
to this address, the controller must respond to the API call; any additional result values must be written to
`wItemAPIBuffer`, and the result of the API call itself must be written to `wItemAPICommand`. (`wItemAPICommand` must
always be written last; the game will strictly follow this rule.)

The result of the call is a tri-state boolean, and it can be any of the following values:

* 0: **false**
* 1: **true**
* 2: **null**
* 3: timeout/delay - see below

The timeout value should be used when the controller cannot complete the call for any reason (e.g., some component was
disconnected); the game will wait two frames and repeat the call.  
The remaining three values have specific meanings for each call (that will be explained in the details for each one of
them), but they generally behave like a regular tri-state boolean value.

The API calls are divided into two groups: `$04` to `$3F` represent regular function calls, which carry their
parameters in `wItemAPIBuffer` and expect additional results in that buffer as well, and `$40` to `$FF` represent
inventory loads, which are functions that are used to load a full page of items and return values via `wItemAPIBuffer`
and the buffer in `wNumItems`.

The API must be locked when the game starts; a locked API does not respond to calls. In locked state, it must only
respond to a correctly formed `UNLOCK` call (i.e., with the correct parameters); once unlocked, it should behave
normally unless it is locked again via a `LOCK` call. This prevents both accidental calls due to the initial state of
RAM or due to loading a dirty save, and random calls due to attempting to interact with an unmodified ROM.

---

## Regular API calls

The following table lists all API calls. Note that the arguments and return values are passed via `wItemAPIBuffer`.

|Value|Name                    |Arguments                                    |Return values                          |
|:---:|:-----------------------|:--------------------------------------------|:--------------------------------------|
|`$04`|`LOCK`                  |_(void)_                                     |_(none)_                               |
|`$05`|`UNLOCK`                |_(unlocking key)_                            |_(none)_                               |
|`$06`|`INITIALIZE_ITEM_LISTS` |_(void)_                                     |_(none)_                               |
|`$10`|`CAN_GET_ITEM`          |item ID, quantity, page #                    |_(none)_                               |
|`$11`|`ADD_ITEM`              |item ID, quantity, page #                    |_(none)_                               |
|`$12`|`HAS_ITEM`              |item ID, quantity                            |page #, index #, quantity              |
|`$13`|`REMOVE_ITEM`           |index #, quantity, page #                    |page #, item count                     |
|`$14`|`CAN_GET_PC_ITEM`       |item ID, quantity, page #                    |_(none)_                               |
|`$15`|`ADD_ITEM_TO_PC`        |item ID, quantity, page #                    |_(none)_                               |
|`$16`|`HAS_ITEM_IN_PC`        |item ID, quantity                            |page #, index #, quantity              |
|`$17`|`REMOVE_ITEM_FROM_PC`   |index #, quantity, page #                    |page #, item count                     |
|`$18`|`DEPOSIT`               |page #, index #, quantity                    |_(none)_                               |
|`$19`|`WITHDRAW`              |page #, index #, quantity                    |_(none)_                               |
|`$1A`|`SWAP_ITEMS`            |page # 1, index # 1, page # 2, index # 2     |_(none)_                               |
|`$1B`|`SWAP_PC_ITEMS`         |page # 1, index # 1, page # 2, index # 2     |_(none)_                               |
|`$1C`|`IS_BAG_EMPTY`          |_(void)_                                     |_(none)_                               |
|`$1D`|`IS_PC_EMPTY`           |_(void)_                                     |_(none)_                               |
|`$1E`|`GET_ITEM_QUANTITIES`   |item ID, item ID, ..., `$00`                 |quantity, quantity, ...                |
|`$1F`|`GET_PAGE_LIMITS`       |_(void)_                                     |bag page count, PC page count          |

Note that in the following descriptions, the term "return values" refers to values returned by the API call in
`wItemAPIBuffer`. The actual boolean result (returned in `wItemAPICommand`) is referred to as "result".

With the exception of the unlocking key for `UNLOCK`, all arguments and return values are one byte long.

For functions that refer to specific locations within the inventory, page numbers and stack numbers both start at 0.
Page numbers may go up to 63 for the bag and 31 for the PC; stack numbers may go up to 59.

### `LOCK`

**Arguments:** none.

**Return values:** none.

**Effects:** called by the game when the API should be locked. A locked API must only respond to an `UNLOCK` command
with the correct key and to no other commands.

**Results:**

* **false:** could not lock the API due to an error.
* **true:** API locked successfully.
* **null:** no-op call; the API was already locked.

### `UNLOCK`

**Arguments:** unlocking key — the first 12 bytes of `wItemAPIBuffer` must equal the string `InitItemAPI@` (in the
game's encoding).

**Return values:** none.

**Effects:** called by the game when the API should be unlocked, such as when the game starts. A locked API must only
respond to an `UNLOCK` command with the correct key (and to no other commands); if the API is already unlocked when
this function is called, the controller should respond to any call.

**Results:**

* **false:** could not unlock the API due to an error or an incorrect unlocking key.
* **true:** API unlocked successfully.
* **null:** no-op call; the API was already unlocked.

### `INITIALIZE_ITEM_LISTS`

**Arguments:** none.

**Return values:** none.

**Effects:** empties out the entire pack and PC; called when a new game starts to initialize the item lists.

**Results:**

* **false:** error when initializing the lists.
* **true:** lists initialized successfully.
* **null:** no-op call; the lists were already initialized and empty.

### `CAN_GET_ITEM`

**Arguments:** item ID, quantity, page number.

**Return values:** none.

**Effects:** determines if the player has enough room in their bag to obtain a specific item. If the page number is
-1, the query is over the whole bag; otherwise, it refers to the specific page indicated in the argument.

**Results:**

* **false:** there is no room for the item.
* **true:** there is enough room for the item.
* **null:** obtaining the item will be a no-op (e.g., quantity is zero); this is considered a success.

### `ADD_ITEM`

**Arguments:** item ID, quantity, page number.

**Return values:** none.

**Effects:** adds an item to the bag. The page number indicates to which page it should be added; if it is -1, the
item should be added wherever there is room for it.

**Results:**

* **false:** failed to add the item to the bag.
* **true:** the item was added successfully.
* **null:** no-op call (e.g., quantity was zero); this is treated as a success.

### `HAS_ITEM`

**Arguments:** item ID, quantity.

**Return values:** page number, stack number, quantity. (Only returned when the result is true.)

**Effects:** determines whether the player has a given quantity of a certain item in the bag. If they do, it returns
the location of an item stack that has at least the requested quantity of the item.

**Results:**

* **false:** the player does not have a stack of the requested item with at least the requested quantity.
* **true:** a stack with at least the requested quantity of the requested item was found.
* **null:** nothing to search for; e.g., quantity is zero.

### `REMOVE_ITEM`

**Arguments:** stack number, quantity, page number.

**Return values:** new page number, page size. (Only returned when the result is null.)

**Effects:** removes items from a specific stack of items in the bag. Note that the stack is identified by its
position, not by item ID. This call returns true or null depending on whether items remain on the stack after removing
the requested quantity; if the call returns null, the game may need to relocate the item cursor in the bag after
removing the items. Therefore, when the call returns null, the new active page number and the number of item stacks in
that page are returned; the new active page may be the same as the requested one, or any other.

**Results:**

* **false:** failed to remove items from the stack.
* **true:** the items were removed from the stack; this did not empty the stack.
* **null:** the items were removed from the stack, removing the whole stack. The new active page number and its size
are returned in the return values.

### `CAN_GET_PC_ITEM`

This call is equivalent to `CAN_GET_ITEM`, but for the PC instead of the bag.

### `ADD_ITEM_TO_PC`

This call is equivalent to `ADD_ITEM`, but for the PC instead of the bag.

### `HAS_ITEM_IN_PC`

This call is equivalent to `HAS_ITEM`, but for the PC instead of the bag.

### `REMOVE_ITEM_FROM_PC`

This call is equivalent to `REMOVE_ITEM`, but for the PC instead of the bag.

### `DEPOSIT`

**Arguments:** bag page number, bag stack number, quantity.

**Return values:** none.

**Effects:** deposits items from the bag into the PC. The arguments indicate the item page from which the items will
come; the controller may choose the page where the items will be placed. Note that this function will be called
_instead_ of `REMOVE_ITEM` and `ADD_ITEM_TO_PC` when items are deposited, and thus should take care of updating the
respective inventories.

**Results:**

* **false:** could not deposit items (e.g., no room).
* **true:** items were deposited.
* **null:** no-op call (e.g., quantity was zero).

### `WITHDRAW`

**Arguments:** PC page number, PC stack number, quantity.

**Return values:** none.

**Effects:** withdraws items from the PC into the bag. The arguments indicate the item page from which the items will
come; the controller may choose the page where the items will be placed. Note that this function will be called
_instead_ of `REMOVE_ITEM_FROM_PC` and `ADD_ITEM` when items are withdrawn, and thus should take care of updating the
respective inventories.

**Results:**

* **false:** could not withdraw items (e.g., no room).
* **true:** items were withdrawn.
* **null:** no-op call (e.g., quantity was zero).

### `SWAP_ITEMS`

**Arguments:** source page number, source stack number, destination page number, destination stack number.

**Return values:** none.

**Effects:** attempts to swap two item stacks in the bag. The swap may occur within the same page or across pages; the
controller should expect both forms. The controller should decide whether the swap goes through or fails; the
controller may deny the swap at its own discretion (e.g., because the items swapped belong to incompatible pages). If
the swap succeeds, the controller may merge item stacks that contain the same item and add up to less than one hundred
units of it, just like the game usually does.

**Results:**

* **false:** the controller refused to swap the items.
* **true:** the item stacks were swapped.
* **null:** no-op call (e.g., attempted to swap a stack with itself).

### `SWAP_PC_ITEMS`

This call is equivalent to `SWAP_ITEMS`, but for the PC instead of the bag.

### `IS_BAG_EMPTY`

**Arguments:** none.

**Return values:** none.

**Effects:** checks whether the bag is empty.

**Results:**

* **false:** the bag has items in it.
* **true:** the bag is empty.
* **null:** invalid call or error; treated as not empty.

### `IS_PC_EMPTY`

This call is equivalent to `IS_BAG_EMPTY`, but for the PC instead of the bag.

### `GET_ITEM_QUANTITIES`

**Arguments:** item ID, item ID, ..., `$00`.

**Return values:** quantity, quantity, ... (Only returned when the result is true.)

**Effects:** checks the quantities in the bag of multiple items at once. Up to 15 item IDs may be given, followed by
a `$00` byte to terminate the list; values beyond the `$00` must be ignored. If the player does have any of the listed
items in the bag, the call returns the quantities of each item (in the order given in the call) in the bag; if the
player holds more than 255 of any item, the API returns 255 for that item instead.

**Results:**

* **false:** none of the requested items were found in the bag.
* **true:** at least one of the items was found; the quantities of each are returned in the return values.
* **null:** invalid call or empty list; treated as no items found.

### `GET_PAGE_LIMITS`

**Arguments:** none.

**Return values:** bag page count, PC page count. (Only returned when the result is true.)

**Effects:** gets the number of pages in both inventories, bag and PC. The bag page count must be between 1 and 64,
and the PC page count must be between 1 and 32.

**Results:**

* **false:** no paged inventories; they are both a single page.
* **true:** limits returned in the return values.
* **null:** invalid call or error; the game treats it as a false result.

---

## Inventory-loading API calls

(WIP)
