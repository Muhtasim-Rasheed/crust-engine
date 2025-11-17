## `property_of(sprite, property)`
Returns the value of the given property of the sprite.

**Properties:**

- `sprite` (String): The name of the sprite to get the property from.
- `property` (String): The property to get. Can be one of the following:
    - `name`: The name of the sprite.
    - `x`: The x-coordinate of the sprite.
    - `y`: The y-coordinate of the sprite.
    - `size`: The size of the sprite, which is a list [width, height].
    - `scale`: The scale of the sprite, which is a number where 1.0 is 100% scale.
    - `direction`: The direction of the sprite in degrees.
    - `completed_broadcasts`: A list of broadcast IDs that have been completed by the sprite.
    - `tags`: A list of tags that the sprite is in.

**Returns:** The value of the specified property for the given sprite. The return type depends on the property requested.

## `set_uv(u, v, w, x)`
Sets the UV coordinates for the sprite, where (u, v) is the bottom-left corner and (w, x) is the top-right corner. This can be used for atlases where there's no need for extra costumes.

**Properties:**

- `u` (Number): The u-coordinate of the bottom-left corner.
- `v` (Number): The v-coordinate of the bottom-left corner.
- `w` (Number): The u-coordinate of the top-right corner.
- `x` (Number): The v-coordinate of the top-right corner.

**Returns:** `null`

## `whoami()`
Returns the name of the current sprite. Works for clones as well. Clones will return `sprite-name (clone #)`.

**Properties:** none

**Returns:** `String` - The name of the current sprite.

## `clone_id()`
Returns the ID of the current clone. Returns 0 if the sprite is not a clone.

**Properties:** none

**Returns:** `Number` - The ID of the current clone, or 0 if not a clone.
