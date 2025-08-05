The sound functions are used to play and control sounds in the project. They can be used to play sound effects, music, and more. Here is a list of all the sound functions in Crust:

- `play_sound(sound, stop_other_sounds)`: Plays the specified sound. The sound is specified by its name in the sounds map of the sprite.
- `stop_all_sounds()`: Stops all sounds currently playing.
- `stop_sound(sound)`: Stops the specified sound. The sound is specified by its name in the sounds map of the sprite.
- `change_sound_filter(filter, increment)`: Changes the specified sound filter by the specified increment.
- `set_sound_filter(filter, value)`: Sets the specified sound filter to the specified value.
- `sound_filter(filter)`: Returns the current value of the specified sound filter. The filter can be one of the valid filters listed below.

## Filters 

All the valid sound filters that can be used are:

- `"volume"`: Changes the volume of the sound (0 - 100).
- `"pitch"`: Changes the pitch of the sound (0 - 100).
- `"pan"`: Changes the stereo panning of the sound (0 - 100, where 0 is left, 50 is center, and 100 is right).
