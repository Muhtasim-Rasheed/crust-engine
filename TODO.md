# Misc

- `print(string)`
- `concat(strings..)`
- `math_fn(op, x)`

# Motion

- [x] `move(steps)`
- [x] `turn_cw(angle)`
- [x] `turn_ccw(angle)`
- [x] `goto(x, y)`
- [x] `goto(mouse | sprite)`
- [x] `glide(x, y, time)`
- [x] `glide(x, y, time, linear | ease | ease-in | ease-out | ease-in-out)`
- [x] `point(angle)`
- [x] `point(mouse | sprite)`
- [x] `point(x, y)`
- [x] `change_x(steps)`
- [x] `set_x(x)`
- [x] `change_y(steps)`
- [x] `set_y(y)`
- [ ] `edge_bounce()`
- [x] `rotation_style()`
- [x] `direction()`
- [x] `x()`
- [x] `y()`

# Looks

- [ ] `say(string, time)`
- [ ] `say(string)`
- [ ] `think(string, time)`
- [ ] `think(string)`
- [x] `switch_costume(costume)`
- [x] `next_costume()`
- [x] `previous_costume()`
- [x] `switch_backdrop(backdrop)`
- [x] `next_backdrop()`
- [x] `previous_backdrop()`
- [x] `change_size(increment)`
- [x] `set_size(size)`
- [x] `change_effect(effect, increment)`
- [x] `set_effect(effect, value)`
- [x] `clear_effects()`
- [x] `clear_effect(effect)`
- [x] `go_to_layer(layer)`
- [x] `go_by_layers(forward | backward, steps)`
- [x] `costume()`
- [x] `backdrop()`
- [x] `size()`
- [x] `effect(effect)`

# Sound

- [x] `play_sound(sound, stop-other-sounds)`
- [ ] `play_sound_until_done(sound)`
- [x] `stop_all_sounds()`
- [x] `stop_sound(sound)`
- [x] `change_sound_effect(effect, increment)`
- [x] `set_sound_effect(effect, value)`
- [x] `sound_effect(effect)`

# Events

- [x] `setup {}`
- [x] `update {}`
- [ ] `key_press key {}`
- [ ] `sprite_clicked {}`
- [ ] `backdrop_switch backdrop  {}`
- [ ] `when bool {}`
- [x] `when_recv_broadcast broadcast {}`
- [x] `broadcast(broadcast)`
- [ ] `broadcast_and_wait(broadcast)`

# Control

- [x] `wait(sec)`
- [ ] `wait_until(bool)`
- [x] `repeat x {}`
- [x] `if bool {}`
- [x] `if bool {} else {}`
- [x] `if bool {} else if bool {}`
- [x] `while bool {}`
- [ ] `stop(all | self | script | other-scripts | other-sprites-and-scripts)`
- [ ] `when_start_as_clone {}`
- [ ] `clone()`
- [ ] `destroy_clone()`

# Window

- [ ] `set_window_width(width)`
- [ ] `set_window_height(height)`
- [ ] `set_window_size(width, height)`
- [ ] `set_window_state(state)`
    - [ ] `normal`
    - [ ] `windowed-fullscreen`
    - [ ] `fullscreen`
    - [ ] `borderless-windowed`
    - [ ] `minimized`
- [ ] `set_window_x(x)`
- [ ] `set_window_y(y)`
- [ ] `set_window_position(x, y)`
- [ ] `window_width()`
- [ ] `window_height()`
- [ ] `screen_width()`
- [ ] `screen_height()`
