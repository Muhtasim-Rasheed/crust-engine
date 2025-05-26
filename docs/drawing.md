The drawing functions are used to draw shapes and lines on the screen unlike in Scratch where you have to move a pen. They can be used to create graphics, animations, and more. It also lets you stamp the sprite onto the stage. Here is a list of all the drawing functions in Crust:

- `set_color(r, g, b)`: Sets the drawing color to the specified RGB values. The values are in the range of 0 to 255.
- `change_r(increment)`: Changes the red component of the drawing color by the specified increment. The increment can be positive or negative.
- `change_g(increment)`: Changes the green component of the drawing color by the specified increment. The increment can be positive or negative.
- `change_b(increment)`: Changes the blue component of the drawing color by the specified increment. The increment can be positive or negative.
- `line(x1, y1, x2, y2, thickness)`: Draws a line from the point `(x1, y1)` to the point `(x2, y2)` with the specified thickness.
- `rect(x1, y1, x2, y2)`: Draws a rectangle with the top-left corner at `(x1, y1)` and the bottom-right corner at `(x2, y2)`. The rectangle is filled with the current drawing color.
- `hrect(x1, y1, x2, y2, thickness)`: Draws a hollow rectangle with the top-left corner at `(x1, y1)` and the bottom-right corner at `(x2, y2)` with the specified thickness. The rectangle is outlined with the current drawing color.
- `circle(x, y, radius)`: Draws a filled circle with the center at `(x, y)` and the specified radius. The circle is filled with the current drawing color.
- `hcircle(x, y, radius, thickness)`: Draws a hollow circle with the center at `(x, y)` and the specified radius with the specified thickness. The circle is outlined with the current drawing color.
- `ellipse(x, y, width, height)`: Draws a filled ellipse with the center at `(x, y)` and the specified width and height. The ellipse is filled with the current drawing color.
- `ellipse(x, y, width, height, rotation)`: Draws a filled ellipse with the center at `(x, y)`, the specified width and height, and the specified rotation in degrees. The ellipse is filled with the current drawing color.
- `hellipse(x, y, width, height, thickness)`: Draws a hollow ellipse with the center at `(x, y)`, the specified width and height, and the specified thickness. The ellipse is outlined with the current drawing color.
- `hellipse(x, y, width, height, rotation, thickness)`: Draws a hollow ellipse with the center at `(x, y)`, the specified width and height, the specified rotation in degrees, and the specified thickness. The ellipse is outlined with the current drawing color.
- `polygon(x1, y1, ..., xN, yN)`: Draws a filled polygon with the specified vertices. The polygon is filled with the current drawing color.
- `hpolygon(thickness, x1, y1, ..., xN, yN)`: Draws a hollow polygon with the specified vertices and thickness. The polygon is outlined with the current drawing color.
- `textured_quad(parse_image_result, x1, y1, x2, y2, x3, y3, x4, y4)`: Draws a textured quadrilateral using the specified image result from `parse_image(binary)`. The quadrilateral is defined by the four points `(x1, y1)`, `(x2, y2)`, `(x3, y3)`, and `(x4, y4)`.
- `stamp()`: Stamps the sprite onto the stage at its current position. The stamp is a copy of the sprite's current appearance, effects included.
- `clear_all_stamps()`: Clears all the stamps on the stage.
- `r()`: Returns the current red component of the drawing color.
- `g()`: Returns the current green component of the drawing color.
- `b()`: Returns the current blue component of the drawing color.

!!! note
    The drawing functions draw in world coordinates, not screen coordinates. The origin `(0, 0)` is at the center of the screen, with positive x values to the right and positive y values downwards.
