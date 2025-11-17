## `play_sound(sound, stop_other_sounds?)`
Plays the specified sound. The sound is specified by its name in the sounds map of the sprite.

**Properties:**

- `sound` (String): The name of the sound to play.
- `stop_other_sounds` (Boolean, optional): If `true`, stops all other sounds currently playing before playing the specified sound. Default is `false`.

**Returns:** `null`

## `stop_all_sounds()`
Stops all sounds currently playing.

**Properties:** none

**Returns:** `null`

## `stop_sound(sound)`
Stops the specified sound. The sound is specified by its name in the sounds map of the sprite

**Properties:**

- `sound` (String): The name of the sound to stop.

**Returns:** `null`

## `change_sound_filter(filter, increment)`
Changes the specified sound filter by the specified increment.

**Properties:**

- `filter` (String): The name of the sound filter to change. See the list of valid filters below.
- `increment` (Number): The amount to change the filter by (can be positive or negative).

**Returns:** `null`

## `set_sound_filter(filter, value)`
Sets the specified sound filter to the specified value.

**Properties:**

- `filter` (String): The name of the sound filter to set. See the list of valid filters below.
- `value` (Number): The value to set the filter to.

**Returns:** `null`

## `sound_filter(filter)`
Returns the current value of the specified sound filter. The filter can be one of the valid filters listed below.

**Properties:**

- `filter` (String): The name of the sound filter to get. See the list of valid filters below.

**Returns:** `Number` - The current value of the specified sound filter.
!!! example

## Filters 

All the valid sound filters that can be used are:

- `"volume"`: Changes the volume of the sound (0 - 100).
- `"pitch"`: Changes the pitch of the sound (0 - 100).
- `"pan"`: Changes the stereo panning of the sound (0 - 100, where 0 is left, 50 is center, and 100 is right).
