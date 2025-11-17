## `move(steps)`
Moves the sprite forward by the specified number of steps in the current direction.

**Properties:**

- `steps` (Number): The number of steps to move forward. Can be positive or negative.

**Returns:** `null`

## `turn_cw(angle)`
Turns the sprite's direction clockwise by the specified angle in degrees.

**Properties:**

- `angle` (Number): The angle in degrees to turn clockwise. Can be positive or negative.

**Returns:** `null`

## `turn_ccw(angle)`
Turns the sprite's direction counter-clockwise by the specified angle in degrees.

**Properties:**

- `angle` (Number): The angle in degrees to turn counter-clockwise. Can be positive or negative.

**Returns:** `null`

## `goto(x, y)` / `goto(object)`
Moves the sprite to a specific location.

=== "`goto(x, y)`"
    Moves the sprite to the specified coordinates `(x, y)` in World coordinates.

    **Properties:**

    - `x` (Number): The x-coordinate to move to.
    - `y` (Number): The y-coordinate to move to.

    **Returns:** `null`

=== "`goto(object)`"
    Moves the sprite to the position of an object.

    **Properties:**

    - `object` (String): The name of the object to move to.
        - `"mouse"`: Moves to the current mouse position.
        - `sprite` (String): The name of the sprite to move to.

    **Returns:** `null`

## `glide(x, y, time, easing?)`
Glides the sprite to the specified coordinates `(x, y)` over the specified time in seconds.

**Properties:**

- `x` (Number): The x-coordinate to glide to.
- `y` (Number): The y-coordinate to glide to.
- `time` (Number): The time in seconds to complete the glide.
- `easing` (String, optional): The easing function to use for the glide. Available options:
    - `"linear"`: Constant speed.
    - `"ease"`: Starts slow, speeds up, then slows down.
    - `"ease-in"`: Starts slow and speeds up.
    - `"ease-out"`: Starts fast and slows down.
    - `"ease-in-out"`: Starts slow, speeds up, then slows down.

**Returns:** `null`

## `point(angle)` / `point(x, y)` / `point(object)`
Points the sprite in a specific direction.

=== "`point(angle)`"
    Points the sprite in the specified direction in degrees.

    **Properties:**

    - `angle` (Number): The angle in degrees to point the sprite. The angle is relative to the up side of the sprite.

    **Returns:** `null`

=== "`point(x, y)`"
    Points the sprite towards the specified coordinates `(x, y)`.

    **Properties:**

    - `x` (Number): The x-coordinate to point towards.
    - `y` (Number): The y-coordinate to point towards.

    **Returns:** `null`

=== "`point(object)`"
    Points the sprite towards the mouse or another sprite.

    **Properties:**

    - `object` (String): The object to point towards.
        - `"mouse"`: Points towards the mouse cursor.
        - `sprite` (String): The name of the sprite to point towards.

    **Returns:** `null`

## `change_x(steps)`
Changes the sprite's x-coordinate by the specified number of steps. Positive values move the sprite to the right, negative values move it to the left.

**Properties:**

- `steps` (Number): The number of steps to change the x-coordinate by.

**Returns:** `null`

## `set_x(x)`
Sets the sprite's x-coordinate to the specified value. The value is in World coordinates, not screen coordinates.

**Properties:**

- `x` (Number): The x-coordinate to set the sprite to.

**Returns:** `null`

## `change_y(steps)`
Changes the sprite's y-coordinate by the specified number of steps. Positive values move the sprite up, negative values move it down.

**Properties:**

- `steps` (Number): The number of steps to change the y-coordinate by.

**Returns:** `null`

## `set_y(y)`
Sets the sprite's y-coordinate to the specified value. The value is in World coordinates, not screen coordinates.

**Properties:**

- `y` (Number): The y-coordinate to set the sprite to.

**Returns:** `null`

## `edge_bounce(enabled)`
Makes the sprite bounce off the edges of the screen when used with `move(steps)`, `change_x(steps)`, or `change_y(steps)`.

**Properties:**

- `enabled` (Boolean): If `true`, the sprite will bounce off the edges. If `false`, the sprite will not bounce off the edges.

**Returns:** `null`

## `rotation_style(style)`
Sets the sprite's rotation style. The rotation style determines how the sprite rotates when it moves. Only visual rotation is affected, not the direction of movement.

**Properties:**

- `style` (String): The rotation style to set. Available options:
    - `"all-around"`: The sprite can rotate in any direction.
    - `"left-right"`: The sprite can only rotate left and right.
    - `"dont-rotate"`: The sprite does not rotate at all.

**Returns:** `null`
!!! example
    ```
    point(45)
    rotation_style("left-right")
    // Still looks like the sprite is pointing right (0 degrees)
    ```

## `direction()`
Returns the sprite's current direction in degrees. The direction is relative to the up side of the sprite.

**Properties:** none

**Returns:** `Number` - The current direction of the sprite in degrees.

## `x()`
Returns the sprite's current x-coordinate in World coordinates.

**Properties:** none

**Returns:** `Number` - The current x-coordinate of the sprite.

## `y()`
Returns the sprite's current y-coordinate in World coordinates.

**Properties:** none

**Returns:** `Number` - The current y-coordinate of the sprite.
