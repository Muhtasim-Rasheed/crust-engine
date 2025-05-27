The window functions are used to control the window's size, position, and state in the application. These functions allow you to customize the appearance of the window and how it interacts with the user. Here is a list of all the window functions in Crust:

- `set_window_width(width)`: Sets the width of the window to the specified value in pixels.
- `set_window_height(height)`: Sets the height of the window to the specified value in pixels.
- `set_window_size(width, height)`: Sets the size of the window to the specified width and height in pixels.
- `set_window_state(state)`: Sets the state of the window to the specified state. The state can be one of the following:
  - `"normal"`: The window is in normal state.
  - `"fullscreen"`: The window is in fullscreen mode.
- `set_window_x(x)`: Sets the x-coordinate of the window's position on the screen.
- `set_window_y(y)`: Sets the y-coordinate of the window's position on the screen.
- `set_window_position(x, y)`: Sets the position of the window on the screen to the specified x and y coordinates.
- `pointer_grab(bool)`: Sets whether the pointer (mouse cursor) is grabbed by the window. If `true`, the pointer is confined to the window; if `false`, it can move freely. This also affects the visibility of the pointer.
- `window_width()`: Returns the current width of the window in pixels.
- `window_height()`: Returns the current height of the window in pixels.
