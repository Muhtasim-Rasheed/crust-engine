# Time and Math

## `time()`
Returns the current time in seconds since the program started.

**Properties:** none

**Returns:** `Number` - The current time in seconds since the program started.

## `frame()`
Returns the current frame number. Starts at 0.

**Properties:** none

**Returns:** `Number` - The current frame number.

## `delta_time()`
Returns the time in seconds since the last frame.

**Properties:** none

**Returns:** `Number` - The time in seconds since the last frame.

## `abs(num)`
Returns the absolute value of the given number.

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The absolute value of the given number.
!!! example
    ```
    assert abs(-5) == abs(5)
    ```

## `sqrt(num)`
Returns the square root of the given number.

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The square root of the given number.
!!! example
    ```
    assert sqrt(25) == 5
    ```

## `sin(num)`
Returns the sine of the given number (in radians).

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The sine of the given number.
!!! example
    ```
    assert sin(to_rad(90)) == 1
    ```

## `cos(num)`
Returns the cosine of the given number (in radians).

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The cosine of the given number.
!!! example
    ```
    assert cos(to_rad(0)) == 1
    ```

## `tan(num)`
Returns the tangent of the given number (in radians).

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The tangent of the given number.
!!! example
    ```
    assert tan(to_rad(45)) == 1
    ```

## `asin(num)`
Returns the arcsine of the given number (in radians).

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The arcsine of the given number.
!!! example
    ```
    assert to_deg(asin(1)) == 90
    ```

## `acos(num)`
Returns the arccosine of the given number (in radians).

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The arccosine of the given number.
!!! example
    ```
    assert to_deg(acos(0)) == 90
    ```

## `atan(num)`
Returns the arctangent of the given number (in radians).

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The arctangent of the given number.
!!! example
    ```
    assert to_deg(atan(1)) == 45
    ```

## `lerp(a, b, t)`
Returns the linear interpolation between `a` and `b` at `t`, where `t` is a value between 0 and 1.

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The linear interpolation between `a` and `b` at `t`.
!!! example
    ```
    assert lerp(0, 10, 0.5) == 5
    ```

## `clamp(value, min, max)`
Clamps the given value between the minimum and maximum values.

**Properties:**

- `num` (Number): The number.

**Returns:** `Number` - The clamped value.
!!! example
    ```
    assert clamp(15, 0, 10) == 10
    ```

## `to_rad(deg)`
Converts the given angle in degrees to radians.

**Properties:**

- `deg` (Number): The angle in degrees.

**Returns:** `Number` - The angle in radians.
!!! example
    ```
    assert to_rad(180) == PI
    ```

## `to_deg(rad)`
Converts the given angle in radians to degrees.

**Properties:**

- `rad` (Number): The angle in radians.

**Returns:** `Number` - The angle in degrees.
!!! example
    ```
    assert to_deg(PI) == 180
    ```

## `random(min, max)`
Returns a random number between the given minimum and maximum values.

**Properties:**

- `min` (Number): The minimum value.
- `max` (Number): The maximum value.

**Returns:** `Number` - A random number between the given minimum and maximum values.
!!! example
    ```
    rand_num = random(1, 10)
    assert rand_num >= 1 and rand_num <= 10
    ```

## `distance(x1, y1, x2, y2)`
Returns the distance between the two points `(x1, y1)` and `(x2, y2)`.

**Properties:**

- `x1` (Number): The x-coordinate of the first point.
- `y1` (Number): The y-coordinate of the first point.
- `x2` (Number): The x-coordinate of the second point.
- `y2` (Number): The y-coordinate of the second point.

**Returns:** `Number` - The distance between the two points.
!!! example
    ```
    assert distance(0, 0, 3, 4) == 5
    ```

## `distance_to(x, y)` / `distance_to(object)`
Returns the distance from the sprite a point

=== "`distance_to(x, y)`"
    Returns the distance from the sprite to the point `(x, y)`.

    **Properties:**

    - `x` (Number): The x-coordinate of the point.
    - `y` (Number): The y-coordinate of the point.

    **Returns:** `Number` - The distance from the sprite to the point `(x, y)`.

=== "`distance_to(object)`"
    Returns the distance from the sprite to the other sprite or the mouse.

    **Properties:**

    - `object` (String): The name of the object to measure distance to.
        - "mouse"`: Measures distance to the mouse cursor.
        - `sprite` (String): The name of the sprite to measure distance to.

    **Returns:** `Number` - The distance from the sprite to the specified object.
