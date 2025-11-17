## `key_down(key)`
Returns `true` if the specified key is currently pressed down.

**Properties:**

- `key` (String): The key to check. See the list of valid keys below.

**Returns:** `Boolean` - `true` if the key is currently pressed down, `false` otherwise.
!!! example
    ```
    update {
        if key_down("a") {
            print("The A key is being held down.")
        }
    }
    ```

## `key_pressed(key)`
Returns `true` if the specified key was pressed down in the current frame.

**Properties:**

- `key` (String): The key to check. See the list of valid keys below.

**Returns:** `Boolean` - `true` if the key was pressed down in the current frame, `false` otherwise.
!!! example
    ```
    update {
        if key_pressed("space") {
            print("Jump")
        }
    }
    ```

## `key_released(key)`
Returns `true` if the specified key was released in the current frame.

**Properties:**

- `key` (String): The key to check. See the list of valid keys below.

**Returns:** `Boolean` - `true` if the key was released in the current frame, `false` otherwise.
!!! example
    ```
    update {
        if key_pressed("space") {
            print("Charging...")
        }
        if key_released("space") {
            print("Swoosh!")
        }
    }
    ```

## `last_key()`
Returns the last key that was pressed down. If no key was pressed down, it returns `null`.

**Properties:** none

**Returns:** `String` - The last key that was pressed down, or `null` if no key was pressed.

## `combination_pressed(keys...)`
Returns `true` if the specified combination of keys was pressed down in order. Fires only once when the combination is pressed, not every frame.

**Properties:**

- `key1` (String): The first key in the combination.
- `key2` (String): The second key in the combination.
- `keyN` (String): Nth key in the combination.

**Returns:** `Boolean` - `true` if the combination was pressed down in order, `false` otherwise.

## `mouse_button_down(button)`
Returns `true` if the specified mouse button is currently pressed down.

**Properties:**

- `button` (String): The mouse button to check. See the list of valid mouse buttons below.

**Returns:** `Boolean` - `true` if the mouse button is currently pressed down, `false` otherwise.

## `mouse_button_pressed(button)`
Returns `true` if the specified mouse button was pressed down in the current frame.

**Properties:**

- `button` (String): The mouse button to check. See the list of valid mouse buttons below.

**Returns:** `Boolean` - `true` if the mouse button was pressed down in the current frame, `false` otherwise.

## `mouse_button_released(button)`
Returns `true` if the specified mouse button was released in the current frame.

**Properties:**

- `button` (String): The mouse button to check. See the list of valid mouse buttons below.

**Returns:** `Boolean` - `true` if the mouse button was released in the current frame, `false` otherwise.

## `sprite_clicked()`
Returns `true` if the sprite was clicked in the current frame.

**Properties:** none

**Returns:** `Boolean` - `true` if the sprite was clicked in the current frame, `false` otherwise.
!!! example
    ```
    setup {
        score = 0
    }
    update {
        if sprite_clicked() {
            score += 1
            print("Score: " + score)
        }
    }
    ```

## `is_backdrop(backdrop)`
Returns `true` if the current backdrop is the specified backdrop. The backdrop is specified by its index in the stage's backdrop list, starting from 0.

**Properties:**

- `backdrop` (Number): The index of the backdrop to check.

**Returns:** `Boolean` - `true` if the current backdrop is the specified backdrop, `false` otherwise.
!!! example
    ```
    update {
        if is_backdrop(2) {
            print("The current backdrop is the third one.")
        }
    }
    ```

## `broadcast_id_of(broadcast)`
Returns the ID of the specified broadcast.

**Properties:**

- `broadcast` (String): The name of the broadcast to get the ID of.

**Returns:** `Number` - The ID of the specified broadcast.
!!! example
    ```
    setup {
        id = broadcast_id_of("start_game")
        if id in property_of("player", "completed_broadcasts") {
            print("Player has completed processing the start_game broadcast.")
        }
    }
    ```

## `broadcast(broadcast)`
Broadcasts the specified message to all sprites. The broadcast message is specified by its name.

**Properties:**

- `broadcast` (String): The name of the broadcast message to send.

**Returns:** `null`
!!! note
    The sprite executing the broadcast function will also be able to receive the broadcast message.

## Keys

The keys that can be used in the `key_down`, `key_pressed`, and `key_released` functions are:

- `"a" - "z"`: The letters A to Z.
- `"0" - "9"`: The numbers 0 to 9.
- ``"`"``: The backtick / grave accent key.
- `"-"`: The minus key.
- `"="`: The equals key.
- `"("`: The left parenthesis key.
- `")"`: The right parenthesis key.
- `"\"`: The backslash key.
- `"/"`: The forward slash key.
- `";"`: The semicolon key.
- `"'"`: The single quote key.
- `","`: The comma key.
- `"."`: The period key.
- `"lctrl"`: The left control key.
- `"rctrl"`: The right control key.
- `"lshift"`: The left shift key.
- `"rshift"`: The right shift key.
- `"lalt"`: The left alt key.
- `"ralt"`: The right alt key.
- `"lsuper"`: The left super key (Windows key / Command key).
- `"rsuper"`: The right super key (Windows key / Command key) (who even uses this?).
- `"tab"`: The tab key.
- `"del"`: The delete key.
- `"back"`: The backspace key.
- `"left"`: The left arrow key.
- `"right"`: The right arrow key.
- `"up"`: The up arrow key.
- `"down"`: The down arrow key.
- `"space"`: The space key.
- `"enter"`: The enter key.
- `"esc"`: The escape key.

## Mouse buttons

The mouse buttons that can be used in the `mouse_button_down`, `mouse_button_pressed`, and `mouse_button_released` functions are:

- `"left"`: The left mouse button.
- `"middle"`: The middle mouse button (usually the scroll wheel).
- `"right"`: The right mouse button.
