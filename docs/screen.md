## Screen Space

The screen is where the game is displayed. The origin (0, 0) is at the top-left corner of the screen. The screen size is determined by the `window_width()` and `window_height()` functions. By default, the screen is 1024x576 pixels, but you can change it using the `set_window_size(width, height)` function (see [Window](window.md) for more details).

## World Space

The world is where the objects exist. The origin (0, 0) is at the center of the screen. All functions in the [Crust Language](crust-language.md) that take coordinates as arguments use world coordinates, not screen coordinates. The world is infinite, meaning you can move objects anywhere in the world without worrying about boundaries, but, the visible area is only two times the screen size from -1024, -576 to 1024, 576 by default. Right is positive x, down is positive y, left is negative x, up is negative y.
