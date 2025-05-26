# Window Functions

The window functions are used to control the window in which the project is displayed. They can be used to set the size and state of the window. Most of the planned functions are not implemented yet due to [Macroquad's](https://macroquad.rs/) limitations, but the implemented ones are listed below:

- `set_window_width(width)`: Sets the width of the window to the specified value in pixels.
- `set_window_height(height)`: Sets the height of the window to the specified value in pixels.
- `set_window_size(width, height)`: Sets the size of the window to the specified width and height in pixels.
- `set_window_state(state)`: Sets the state of the window to the specified state. The state can be one of the following:
  - `"normal"`: The window is in normal state.
  - `"fullscreen"`: The window is in fullscreen mode.
- `window_width()`: Returns the current width of the window in pixels.
- `window_height()`: Returns the current height of the window in pixels.
