# Sound Functions

The sound functions are used to play and control sounds in the project. They can be used to play sound effects, music, and more. Here is a list of all the sound functions in Crust:

- `play_sound(sound, stop_other_sounds)`: Plays the specified sound. The sound is specified by its name in the sounds map of the sprite.
- `stop_all_sounds()`: Stops all sounds currently playing.
- `stop_sound(sound)`: Stops the specified sound. The sound is specified by its name in the sounds map of the sprite.
- `change_sound_effect(effect, increment)`: Changes the specified sound effect by the specified increment.
- `set_sound_effect(effect, value)`: Sets the specified sound effect to the specified value.
- `sound_effect(effect)`: Returns the current value of the specified sound effect. The effect can be one of the valid effects listed below.

## Effects

All the valid sound effects that can be used are:

- `"volume"`: Changes the volume of the sound.
