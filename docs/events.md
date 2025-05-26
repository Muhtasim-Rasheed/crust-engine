# Event Functions

The event functions are used to handle events in the project. They can be used to respond to user input, collisions, and other events. Here is a list of all the event functions in Crust:

- `key_down(key)`: Returns `true` if the specified key is currently pressed down.
- `key_pressed(key)`: Returns `true` if the specified key was pressed down in the current frame.
- `key_released(key)`: Returns `true` if the specified key was released in the current frame.
- `did_get_clicked()`: Returns `true` if the sprite was clicked in the current frame.
- `is_backdrop(backdrop)`: Returns `true` if the current backdrop is the specified backdrop. The backdrop is specified by its index in the stage's backdrop list, starting from 0.
- `is_broadcasted(broadcast)`: Returns `true` if the specified broadcast message was received in the current frame. The broadcast message is specified by its name.
- `broadcast(broadcast)`: Broadcasts the specified message to all sprites. The broadcast message is specified by its name.

!!! note
    If the reciver sprite is placed before the sender sprite in the `project.toml` file, the receiver will not receive the broadcast message. This is because Crust processes sprites in the order they are defined in the `project.toml` file. To ensure that all sprites can receive broadcast messages, place the sender sprite before all receiver sprites in the `project.toml` file.

## Keys

The keys that can be used in the `key_down`, `key_pressed`, and `key_released` functions are:

- `"a" - "z"`: The letters A to Z.
- `"0" - "9"`: The numbers 0 to 9.
- ``"`"``: The backtick / grave accent key.
- `"-"`: The minus key.
- `"="`: The equals key.
- `"("`: The left parenthesis key.
- `")"`: The right parenthesis key.
- `"\"`: The backslash key.
- `"/"`: The forward slash key.
- `";"`: The semicolon key.
- `"'"`: The single quote key.
- `","`: The comma key.
- `"."`: The period key.
- `"lctrl"`: The left control key.
- `"rctrl"`: The right control key.
- `"lshift"`: The left shift key.
- `"rshift"`: The right shift key.
- `"lalt"`: The left alt key.
- `"ralt"`: The right alt key.
- `"lsuper"`: The left super key (Windows key / Command key).
- `"rsuper"`: The right super key (Windows key / Command key) (who even uses this?).
- `"tab"`: The tab key.
- `"del"`: The delete key.
- `"back"`: The backspace key.
- `"left"`: The left arrow key.
- `"right"`: The right arrow key.
- `"up"`: The up arrow key.
- `"down"`: The down arrow key.
- `"space"`: The space key.
- `"enter"`: The enter key.
- `"esc"`: The escape key.

Thats a lot of keys!
