## `set_cam()` / `set_cam(x, y)` / `set_cam(x, y, zoom_x, zoom_y)` / `set_cam(x, y, zoom_x, zoom_y, rotation)`
Sets the camera's position, zoom level, and rotation.

=== "`set_cam()`"
    Sets the camera's 0, 0 position to the sprite's current position.

    **Properties:** none

    **Returns:** `null`

=== "`set_cam(x, y)`"
    Sets the camera's 0, 0 position to the given coordinates `(x, y)`.

    **Properties:**

    - `x` (Number): The x-coordinate to set the camera's position to.
    - `y` (Number): The y-coordinate to set the camera's position to.

    **Returns:** `null`

=== "`set_cam(x, y, zoom_x, zoom_y)`"
    Sets the camera's 0, 0 position to the given coordinates `(x, y)` and sets the zoom level to `(zoom_x, zoom_y)`.

    **Properties:**

    - `x` (Number): The x-coordinate to set the camera's position to.
    - `y` (Number): The y-coordinate to set the camera's position to.
    - `zoom_x` (Number): The zoom level in the x-direction. 100% is represented by 100.
    - `zoom_y` (Number): The zoom level in the y-direction. 100% is represented by 100.

    **Returns:** `null`

=== "`set_cam(x, y, zoom_x, zoom_y, rotation)`"
    Sets the camera's 0, 0 position to the given coordinates `(x, y)`, sets the zoom level to `(zoom_x, zoom_y)`, and sets the rotation of the camera in degrees.

    **Properties:**

    - `x` (Number): The x-coordinate to set the camera's position to.
    - `y` (Number): The y-coordinate to set the camera's position to.
    - `zoom_x` (Number): The zoom level in the x-direction. 100% is represented by 100.
    - `zoom_y` (Number): The zoom level in the y-direction. 100% is represented by 100.
    - `rotation` (Number): The rotation of the camera in degrees.

    **Returns:** `null`
