## `set_window_width(width)`
Sets the width of the window to the specified value in pixels.

**Properties:**

- `width` (Number): The desired width of the window in pixels.

**Returns:** `null`

## `set_window_height(height)`
Sets the height of the window to the specified value in pixels.

**Properties:**

- `height` (Number): The desired height of the window in pixels.

**Returns:** `null`

## `set_window_size(width, height)`
Sets the size of the window to the specified width and height in pixels.

**Properties:**

- `width` (Number): The desired width of the window in pixels.
- `height` (Number): The desired height of the window in pixels.

**Returns:** `null`

## `set_window_state(state)`
Sets the state of the window to the specified state.

**Properties:**

- `state` (String): The desired state of the window. Can be one of the following:
  - `"normal"`: The window is in normal state.
  - `"fullscreen"`: The window is in fullscreen mode.

**Returns:** `null`

## `set_window_x(x)`
Sets the x-coordinate of the window's position on the screen.

**Properties:**

- `x` (Number): The desired x-coordinate of the window's position on the screen.

**Returns:** `null`

## `set_window_y(y)`
Sets the y-coordinate of the window's position on the screen.

**Properties:**

- `y` (Number): The desired y-coordinate of the window's position on the screen.

**Returns:** `null`

## `set_window_position(x, y)`
Sets the position of the window on the screen to the specified x and y coordinates.

**Properties:**

- `x` (Number): The desired x-coordinate of the window's position on the screen.
- `y` (Number): The desired y-coordinate of the window's position on the screen

**Returns:** `null`

## `pointer_grab(bool)`
Sets whether the pointer (mouse cursor) is grabbed by the window. If `true`, the pointer is confined to the window; if `false`, it can move freely. This also affects the visibility of the pointer.

**Properties:**

- `bool` (Boolean): Whether to grab the pointer (`true`) or release it (`false`).

**Returns:** `null`

## `window_width()`
Returns the current width of the window in pixels.

**Properties:** none

**Returns:** `Number` - The current width of the window in pixels.

## `window_height()`
Returns the current height of the window in pixels.

**Properties:** none

**Returns:** `Number` - The current height of the window in pixels.
