# Misc

- `print(string)`
- `time()`
- `concat(strings..)`
- `abs(num)`
- `sqrt(num)`
- `sin(num)`
- `cos(num)`
- `tan(num)`
- `asin(num)`
- `acos(num)`
- `atan(num)`
- `lerp(a, b, t)`

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
- [x] `key_down(key)`
- [x] `key_pressed(key)`
- [x] `key_release(key)`
- [x] `did_get_clicked()`
- [ ] `backdrop_switch backdrop {}`
- [ ] `when bool {}`
- [x] `is_broadcasted(broadcast)`
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

# Drawing

- [x] `set_color(r, g, b)`
- [x] `change_r(increment)`
- [x] `change_g(increment)`
- [x] `change_b(increment)`
- [x] `line(x1, y1, x2, y2, thickness)`
- [x] `rect(x1, y1, x2, y2)`
- [x] `hrect(x1, y1, x2, y2, thickness)`
- [x] `circle(x, y, radius)`
- [x] `hcircle(x, y, radius, thickness)`
- [ ] `ellipse(x, y, width, height)`
- [ ] `hellipse(x, y, width, height, thickness)`
- [x] `polygon(x1, y1..., xN, yN)`
- [x] `hpolygon(thickness, x1, y1..., xN, yN)`
- [x] `r()`
- [x] `g()`
- [x] `b()`

# Window

- [x] `set_window_width(width)`
- [x] `set_window_height(height)`
- [x] `set_window_size(width, height)`
- [x] `set_window_state(state)`
    - [x] `normal`
    - [ ] `windowed-fullscreen`
    - [x] `fullscreen`
    - [ ] `borderless-windowed`
    - [ ] `minimized`
- [ ] `set_window_x(x)`
- [ ] `set_window_y(y)`
- [ ] `set_window_position(x, y)`
- [x] `window_width()`
- [x] `window_height()`
- [ ] `screen_width()`
- [ ] `screen_height()`
