## `hide()`
Hides the sprite. The sprite will not be visible.

**Properties:** none

**Returns:** `null`

## `show()`
Shows the sprite. The sprite will be visible.

**Properties:** none

**Returns:** `null`

## `say(string)` / `say(string, time)`
Makes the sprite say the specified string. The string is displayed above the sprite.

=== "`say(string)`"
    The sprite will say the string indefinitely until another `say` or `think` block is called.

    **Properties:**
    
    - `string` (String): The string to say indefinitely until another `say` or `think` block is called.
    
    **Returns:** `null`

=== "`say(string, time)`"
    The sprite will say the string for the specified time in seconds and will also pause the script for that duration.

    **Properties:**

    - `string` (String): The string to say.
    - `time` (Number): The time in seconds to say the string.

    **Returns:** `null`

## `think(string)` / `think(string, time)`
Makes the sprite think the specified string. The string is displayed above the sprite. The text is 30% transparent.

=== "`think(string)`"
    The sprite will think the string indefinitely until another `say` or `think` block is called.

    **Properties:**
    
    - `string` (String): The string to think indefinitely until another `say` or `think` block is called.
    
    **Returns:** `null`

=== "`think(string, time)`"
    The sprite will think the string for the specified time in seconds and will also pause the script for that duration.

    **Properties:**

    - `string` (String): The string to think.
    - `time` (Number): The time in seconds to think the string.

    **Returns:** `null`

## `switch_costume(costume)`
Switches the sprite's costume to the specified costume. The costume is specified by its index in the sprite's costume list, starting from 0.

**Properties:**

- `costume` (Number): The index of the costume to switch to.

**Returns:** `null`

## `next_costume()`
Switches the sprite to the next costume in the sprite's costume list. If the sprite is already on the last costume, it wraps around to the first costume.

**Properties:** none

**Returns:** `null`

## `previous_costume()`
Switches the sprite to the previous costume in the sprite's costume list. If the sprite is already on the first costume, it wraps around to the last costume.

**Properties:** none

**Returns:** `null`

## `switch_backdrop(backdrop)`
Switches the backdrop to the specified backdrop. The backdrop is specified by its index in the stage's backdrop list, starting from 0.

**Properties:**

- `backdrop` (Number): The index of the backdrop to switch to.

**Returns:** `null`

## `next_backdrop()`
Switches the stage to the next backdrop in the stage's backdrop list. If the stage is already on the last backdrop, it wraps around to the first backdrop.

**Properties:** none

**Returns:** `null`

## `previous_backdrop()`
Switches the stage to the previous backdrop in the stage's backdrop list. If the stage is already on the first backdrop, it wraps around to the last backdrop.

**Properties:** none

**Returns:** `null`

## `change_scale(increment)`
Changes the sprite's scale by the specified increment. The scale is a percentage of the original size, where 100% is the original size.

**Properties:**

- `increment` (Number): The amount to change the scale by.

**Returns:** `null`

## `set_scale(scale)`
Sets the sprite's scale to the specified value. The scale is a percentage of the original size

**Properties:**

- `scale` (Number): The scale to set the sprite to.

**Returns:** `null`
!!! example
    ```
    update {
        set_scale(sin(time()) * 50 + 100) // Scale oscillates between 50% and 150%
    }
    ```

## `change_effect(effect, increment)`
Changes the specified effect by the specified increment.

**Properties:**

- `effect` (String): The name of the effect to change. See the list of valid effects below.
- `increment` (Number): The amount to change the effect by (can be positive or negative).

**Returns:** `null`

## `set_effect(effect, value)`
Sets the specified effect to the specified value.

**Properties:**

- `effect` (String): The name of the effect to set. See the list of valid effects below.
- `value` (Number): The value to set the effect to.

**Returns:** `null`

## `clear_effects()`
Clears all effects applied to the sprite.

**Properties:** none

**Returns:** `null`

## `clear_effect(effect)`
Clears the specified effect applied to the sprite.

**Properties:**

- `effect` (String): The name of the effect to clear. See the list of valid effects below.

**Returns:** `null`

## `go_to_layer(layer)`
Moves the sprite to the specified layer.

**Properties:**

- `layer` (Number): The layer to move the sprite to. Higher numbers are in front of lower numbers.

**Returns:** `null`

## `go_by_layers("forward" | "backward", steps)`
Moves the sprite by the specified number of layers in the specified direction.

**Properties:**

- `direction` (String): The direction to move the sprite. Can be either `"forward"` or `"backward"`.
    - `"forward"`: Moves the sprite forward by the specified number of layers.
    - `"backward"`: Moves the sprite backward by the specified number of layers.
- `steps` (Number): The number of layers to move the sprite by.

**Returns:** `null`

## `costume()`
Returns the current costume of the sprite.

**Properties:** none

**Returns:** `Number` - The index of the current costume.

## `backdrop()`
Returns the current backdrop of the stage.

**Properties:** none

**Returns:** `Number` - The index of the current backdrop.

## `size()`
Returns the current size of the sprite.

**Properties:** none

**Returns:** `Number` - The current size of the sprite.

## `scale()`
Returns the current scale modifier of the sprite.

**Properties:** none

**Returns:** `Number` - The current scale modifier of the sprite.

## `bounds()`
Returns the bounds of the sprite as a list `[x, y, width, height]`, where `(x, y)` is the top-left corner of the sprite.

**Properties:** none

**Returns:** `List` - A list containing the bounds of the sprite: `[x, y, width, height]`.

## `layer()`
Returns the current layer of the sprite.

**Properties:** none

**Returns:** `Number` - The current layer of the sprite.

## `effect(effect)`
Returns the current value of the specified effect applied to the sprite. The effect can be one of the valid effects listed below.

**Properties:**

- `effect` (String): The name of the effect to get. See the list of valid effects below.

**Returns:** `Number` - The current value of the specified effect.

## Effects

All the valid effects that can be used are:

- `"brightness"`: Changes the brightness of the sprite.
- `"ghost"`: Changes the transparency of the sprite.
- `"hue"`: Changes the hue of the sprite.
- `"saturation"`: Changes the saturation of the sprite.
- `"sepia"`: Changes the sepia effect of the sprite.
- `"grayscale-averaged"`: Changes the grayscale effect of the sprite, using the average method.
- `"grayscale-weighted"`: Changes the grayscale effect of the sprite, using the weighted method.
- `"invert"`: Inverts the colors of the sprite.
- `"multiply"`: Multiplies the RGB excluding transparency of the sprite by the specified value.
- `"multiply-r"`: Multiplies the red channel of the sprite by the specified value.
- `"multiply-g"`: Multiplies the green channel of the sprite by the specified value.
- `"multiply-b"`: Multiplies the blue channel of the sprite by the specified value.
- `"multiply-a"`: Multiplies the transparency channel of the sprite by the specified value.
- `"add"`: Adds the specified value to the RGB excluding transparency of the sprite.
- `"add-r"`: Adds the specified value to the red channel of the sprite.
- `"add-g"`: Adds the specified value to the green channel of the sprite.
- `"add-b"`: Adds the specified value to the blue channel of the sprite.
- `"add-a"`: Adds the specified value to the transparency channel of the sprite.
