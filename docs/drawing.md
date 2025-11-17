## `set_color(r, g, b, a)`
Sets the drawing color to the specified RGBA values. The values are in the range of 0 to 255.

**Properties:**

- `r` (Number): The red component of the color (0-255).
- `g` (Number): The green component of the color (0-255).
- `b` (Number): The blue component of the color (0-255).

**Returns:** `null`

## `change_r(increment)`
Changes the red component of the drawing color by the specified increment. The increment can be positive or negative.

**Properties:**

- `increment` (Number): The amount to change the red component by.

**Returns:** `null`

## `change_g(increment)`
Changes the green component of the drawing color by the specified increment. The increment can be positive or negative.

**Properties:**

- `increment` (Number): The amount to change the green component by.

**Returns:** `null`

## `change_b(increment)`
Changes the blue component of the drawing color by the specified increment. The increment can be positive or negative.

**Properties:**

- `increment` (Number): The amount to change the blue component by.

**Returns:** `null`

## `change_a(increment)`
Changes the alpha (transparency) component of the drawing color by the specified increment. The increment can be positive or negative.

**Properties:**

- `increment` (Number): The amount to change the alpha component by.

**Returns:** `null`

## `line(x1, y1, x2, y2, thickness)`
Draws a line from the point `(x1, y1)` to the point `(x2, y2)` with the specified thickness.

**Properties:**

- `x1` (Number): The x-coordinate of the starting point.
- `y1` (Number): The y-coordinate of the starting point.
- `x2` (Number): The x-coordinate of the ending point.
- `y2` (Number): The y-coordinate of the ending point.
- `thickness` (Number): The thickness of the line.

**Returns:** `null`

## `rect(x1, y1, x2, y2)`
Draws a rectangle with the top-left corner at `(x1, y1)` and the bottom-right corner at `(x2, y2)`.

**Properties:**

- `x1` (Number): The x-coordinate of the top-left corner.
- `y1` (Number): The y-coordinate of the top-left corner.
- `x2` (Number): The x-coordinate of the bottom-right corner.
- `y2` (Number): The y-coordinate of the bottom-right corner.

**Returns:** `null`

## `hrect(x1, y1, x2, y2, thickness)`
Draws a hollow rectangle with the top-left corner at `(x1, y1)` and the bottom-right corner at `(x2, y2)` with the specified thickness.

**Properties:**

- `x1` (Number): The x-coordinate of the top-left corner.
- `y1` (Number): The y-coordinate of the top-left corner.
- `x2` (Number): The x-coordinate of the bottom-right corner.
- `y2` (Number): The y-coordinate of the bottom-right corner.
- `thickness` (Number): The thickness of the rectangle outline.

**Returns:** `null`

## `circle(x, y, radius)`
Draws a filled circle with the center at `(x, y)` and the specified radius.

**Properties:**

- `x` (Number): The x-coordinate of the center of the circle.
- `y` (Number): The y-coordinate of the center of the circle.
- `radius` (Number): The radius of the circle.

**Returns:** `null`

## `hcircle(x, y, radius, thickness)`
Draws a hollow circle with the center at `(x, y)` and the specified radius with the specified thickness.

**Properties:**

- `x` (Number): The x-coordinate of the center of the circle.
- `y` (Number): The y-coordinate of the center of the circle.
- `radius` (Number): The radius of the circle.
- `thickness` (Number): The thickness of the circle outline.

**Returns:** `null`

## `ellipse(x, y, width, height, rotation?)`
Draws a filled ellipse with the center at `(x, y)` and the specified width and height. Optionally, a rotation in degrees can be specified.

**Properties:**

- `x` (Number): The x-coordinate of the center of the ellipse.
- `y` (Number): The y-coordinate of the center of the ellipse.
- `width` (Number): The width of the ellipse.
- `height` (Number): The height of the ellipse.
- `rotation` (Number, optional): The rotation of the ellipse in degrees.

**Returns:** `null`

## `hellipse(x, y, width, height, rotation?, thickness)`
Draws a hollow ellipse with the center at `(x, y)`, the specified width and height, and the specified thickness. Optionally, a rotation in degrees can be specified.

**Properties:**

- `x` (Number): The x-coordinate of the center of the ellipse.
- `y` (Number): The y-coordinate of the center of the ellipse.
- `width` (Number): The width of the ellipse.
- `height` (Number): The height of the ellipse.
- `rotation` (Number, optional): The rotation of the ellipse in degrees.
- `thickness` (Number): The thickness of the ellipse outline.

**Returns:** `null`

## `polygon(xs, ys)`
Draws a filled polygon with the specified vertices.
!!! warning
    The polygon can only be convex (no inward dents) and should be wounded counter-clockwise.

**Properties:**

- `xs` (List of Numbers): The x-coordinates of the vertices.
- `ys` (List of Numbers): The y-coordinates of the vertices.

**Returns:** `null`

## `hpolygon(thickness, xs, ys)`
Draws a hollow polygon with the specified vertices and thickness.
!!! warning
    The polygon can only be convex (no inward dents) and should be wounded counter-clockwise.

**Properties:**

- `thickness` (Number): The thickness of the polygon outline.
- `xs` (List of Numbers): The x-coordinates of the vertices.
- `ys` (List of Numbers): The y-coordinates of the vertices.

**Returns:** `null`

## `text(x, y, text, font_size)`
Draws the specified text at the position `(x, y)` with the specified font size.

**Properties:**

- `x` (Number): The x-coordinate of the position to draw the text.
- `y` (Number): The y-coordinate of the position to draw the text.
- `text` (String): The text to draw.
- `font_size` (Number): The font size of the text.

**Returns:** `null`

## `textured_tri(parse_image_result, xs, ys, us, vs)`
Draws a textured triangle using the specified vertices and texture coordinates.

**Properties:**

- `parse_image_result` (List): The result from the `parse_image()` function. Alternatively, you can construct this manually by creating a list like:
    ```
    [
        image_width,
        image_height,
        [R, G, B, A, R, G, B, A, ...], // Pixel data as a flat list
    ]
    ```
- `xs` (List of Numbers): The x-coordinates of the triangle vertices.
- `ys` (List of Numbers): The y-coordinates of the triangle vertices.
- `us` (List of Numbers): The u texture coordinates of the triangle vertices.
- `vs` (List of Numbers): The v texture coordinates of the triangle vertices.

**Returns:** `null`

## `stamp()`
Stamps the sprite onto the stage at its current position. The stamp is a copy of the sprite's current appearance, effects included.

**Properties:** none

**Returns:** `null`

## `clear_all_stamps()`
Clears all the stamps on the stage.

**Properties:** none

**Returns:** `null`

## `r()`
Returns the current red component of the drawing color.

**Properties:** none

**Returns:** `Number` - The red component of the drawing color (0-255).

## `g()`
Returns the current green component of the drawing color.

**Properties:** none

**Returns:** `Number` - The green component of the drawing color (0-255).

## `b()`
Returns the current blue component of the drawing color.

**Properties:** none

**Returns:** `Number` - The blue component of the drawing color (0-255).

## `a()`
Returns the current alpha (transparency) component of the drawing color.

**Properties:** none

**Returns:** `Number` - The alpha component of the drawing color (0-255).

!!! note
    The drawing functions draw in world coordinates, not screen coordinates. The origin `(0, 0)` is at the center of the screen, with positive x values to the right and positive y values upwards.
