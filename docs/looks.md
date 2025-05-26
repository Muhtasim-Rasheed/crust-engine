The looks functions are used to change the appearance of sprites and backdrops. They can be used to change costumes, backdrops, sizes, effects, and more. Here is a list of all the looks functions in Crust:

- `say(string, time)`: Makes the sprite say the specified string for the specified time in seconds. The string is displayed above the sprite.
- `say(string)`: Makes the sprite say the specified string indefinitely until another `say` or `think` block is called.
- `think(string, time)`: Makes the sprite think the specified string for the specified time in seconds. The string is displayed above the sprite. The text is 30% transparent.
- `think(string)`: Makes the sprite think the specified string indefinitely until another `say` or `think` block is called.
- `switch_costume(costume)`: Switches the sprite's costume to the specified costume. The costume is specified by its index in the sprite's costume list, starting from 0.
- `next_costume()`: Switches the sprite to the next costume in the sprite's costume list. If the sprite is already on the last costume, it wraps around to the first costume.
- `previous_costume()`: Switches the sprite to the previous costume in the sprite's costume list. If the sprite is already on the first costume, it wraps around to the last costume.
- `switch_backdrop(backdrop)`: Switches the backdrop to the specified backdrop. The backdrop is specified by its index in the stage's backdrop list, starting from 0.
- `next_backdrop()`: Switches the stage to the next backdrop in the stage's backdrop list. If the stage is already on the last backdrop, it wraps around to the first backdrop.
- `previous_backdrop()`: Switches the stage to the previous backdrop in the stage's backdrop list. If the stage is already on the first backdrop, it wraps around to the last backdrop.
- `change_scale(increment)`: Changes the sprite's scale by the specified increment. The scale is a percentage of the original size, where 100% is the original size.
- `set_scale(scale)`: Sets the sprite's scale to the specified value. The scale is a percentage of the original size, where 100% is the original size.
- `change_effect(effect, increment)`: Changes the specified effect by the specified increment
- `set_effect(effect, value)`: Sets the specified effect to the specified value.
- `clear_effects()`: Clears all effects applied to the sprite.
- `clear_effect(effect)`: Clears the specified effect applied to the sprite.
- `go_to_layer(layer)`: Moves the sprite to the specified layer.
- `go_by_layers("forward" | "backward", steps)`: Moves the sprite by the specified number of layers in the specified direction.
    - `"forward"`: Moves the sprite forward by the specified number of layers.
    - `"backward"`: Moves the sprite backward by the specified number of layers.
- `costume()`: Returns the current costume
- `backdrop()`: Returns the current backdrop
- `size()`: Returns the current size of the sprite.
- `scale()`: Returns the current scale modifier of the sprite.
- `bounds()`: Returns the bounds of the sprite as a list `[x, y, width, height]`, where `(x, y)` is the top-left corner of the sprite.
- `layer()`: Returns the current layer of the sprite.
- `effect(effect)`: Returns the current value of the specified effect applied to the sprite. The effect can be one of the valid effects listed below.

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
- `"multiply"`: Multiplies the colors of the sprite by the specified value.
- `"multiply-r"`: Multiplies the red channel of the sprite by the specified value.
- `"multiply-g"`: Multiplies the green channel of the sprite by the specified value.
- `"multiply-b"`: Multiplies the blue channel of the sprite by the specified value.
- `"add"`: Adds the specified value to the colors of the sprite.
- `"add-r"`: Adds the specified value to the red channel of the sprite.
- `"add-g"`: Adds the specified value to the green channel of the sprite.
- `"add-b"`: Adds the specified value to the blue channel of the sprite.
