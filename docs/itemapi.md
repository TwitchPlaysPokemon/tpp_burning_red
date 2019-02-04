# Item API documentation

This page documents the API used by the modified Pokémon Red ROM in order to execute inventory-related actions and/or
retrieve data about the inventory state.

**Note that the ROM that uses the API no longer tracks its own inventory data.** This means that the ROM fully depends
on the API in order to perform all inventory-related tasks. The API will have to keep track of the inventory and will
be informed when the player gains or loses items, deposits them, withdraws them, swaps them, etc.

## Basics

Communication with the API uses three locations in memory:

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

(WIP)
