use crate::utils::{sprite::builtins::*, *};
use std::collections::HashMap;

macro_rules! builtin {
    ($map:ident, $name:literal, $func:expr) => {
        $map.insert(
            $name.to_string(),
            Callable::Builtin(BuiltinFunction { inner: $func }),
        );
    };
}

#[rustfmt::skip]
pub fn builtins() -> HashMap<String, Callable> {
    let mut builtins = HashMap::new();

    // MISC
    builtin!(builtins, "args", |st, _| misc::args(st));
    builtin!(builtins, "print", |st, ar| misc::print(st, ar, false));
    builtin!(builtins, "print_raw", |st, ar| misc::print(st, ar, true));
    builtin!(builtins, "input", |st, ar| misc::input(st, ar));
    builtin!(builtins, "time", |st, _| misc::time(st));
    builtin!(builtins, "abs", |_, ar| misc::math(ar, "abs"));
    builtin!(builtins, "sqrt", |_, ar| misc::math(ar, "sqrt"));
    builtin!(builtins, "sin", |_, ar| misc::math(ar, "sin"));
    builtin!(builtins, "cos", |_, ar| misc::math(ar, "cos"));
    builtin!(builtins, "tan", |_, ar| misc::math(ar, "tan"));
    builtin!(builtins, "asin", |_, ar| misc::math(ar, "asin"));
    builtin!(builtins, "acos", |_, ar| misc::math(ar, "acos"));
    builtin!(builtins, "atan", |_, ar| misc::math(ar, "atan"));
    builtin!(builtins, "lerp", |_, ar| misc::lerp(ar));
    builtin!(builtins, "property_of", |st, ar| misc::property_of(st, ar));
    builtin!(builtins, "to_rad", |_, ar| misc::to_rad(ar));
    builtin!(builtins, "to_deg", |_, ar| misc::to_deg(ar));
    builtin!(builtins, "clamp", |_, ar| misc::clamp(ar));
    builtin!(builtins, "len", |_, ar| misc::len(ar));
    builtin!(builtins, "keys", |_, ar| misc::key_value(ar, "keys"));
    builtin!(builtins, "values", |_, ar| misc::key_value(ar, "values"));
    builtin!(builtins, "random", |_, ar| misc::random(ar));
    builtin!(builtins, "distance", |st, ar| misc::distance(st, ar, false));
    builtin!(builtins, "distance_to", |st, ar| misc::distance(st, ar, true));
    builtin!(builtins, "write", |st, ar| misc::write(st, ar));
    builtin!(builtins, "read", |st, ar| misc::read(st, ar, false));
    builtin!(builtins, "read_binary", |st, ar| misc::read(st, ar, true));
    builtin!(builtins, "parse_image", |_, ar| misc::parse_image(ar));
    builtin!(builtins, "screenshot", |st, ar| misc::screenshot(st, ar));
    builtin!(builtins, "typeof", |_, ar| misc::r#typeof(ar));
    builtin!(builtins, "push", |_, ar| misc::push(ar));
    builtin!(builtins, "pop", |_, ar| misc::pop(ar));
    builtin!(builtins, "insert", |_, ar| misc::insert(ar));
    builtin!(builtins, "remove", |_, ar| misc::remove(ar));
    builtin!(builtins, "extend", |_, ar| misc::extend(ar));
    builtin!(builtins, "contains", |_, ar| misc::contains(ar));
    builtin!(builtins, "sort", misc::sort);
    builtin!(builtins, "filter", misc::filter);
    builtin!(builtins, "map", misc::map);
    builtin!(builtins, "split", |_, ar| misc::split(ar));
    builtin!(builtins, "join", |_, ar| misc::join(ar));
    builtin!(builtins, "starts_with", |_, ar| misc::starts_with(ar));
    builtin!(builtins, "ends_with", |_, ar| misc::ends_with(ar));
    builtin!(builtins, "trim", |_, ar| misc::trim(ar));
    builtin!(builtins, "range", |_, ar| misc::range(ar));
    builtin!(builtins, "to_string", |_, ar| misc::to(ar, "string"));
    builtin!(builtins, "to_number", |_, ar| misc::to(ar, "number"));
    builtin!(builtins, "to_boolean", |_, ar| misc::to(ar, "boolean"));
    builtin!(builtins, "to_list", |_, ar| misc::to(ar, "list"));
    builtin!(builtins, "to_object", |_, ar| misc::to(ar, "object"));
    builtin!(builtins, "whoami", |st, _| misc::whoami(st));
    builtin!(builtins, "cloneid", |st, _| misc::cloneid(st));
    builtin!(builtins, "frame", |st, _| misc::frame(st));
    builtin!(builtins, "delta_time", |_, _| misc::delta_time());

    // MOTION
    builtin!(builtins, "move", |st, ar| motion::r#move(st, ar));
    builtin!(builtins, "turn_cw", |st, ar| motion::turn_cw(st, ar));
    builtin!(builtins, "turn_ccw", |st, ar| motion::turn_ccw(st, ar));
    builtin!(builtins, "goto", |st, ar| motion::goto(st, ar));
    builtin!(builtins, "glide", |st, ar| motion::glide(st, ar));
    builtin!(builtins, "point", |st, ar| motion::point(st, ar));
    builtin!(builtins, "set_x", |st, ar| motion::set_pos(st, ar, "x"));
    builtin!(builtins, "change_x", |st, ar| motion::change_pos(st, ar, "x"));
    builtin!(builtins, "set_y", |st, ar| motion::set_pos(st, ar, "y"));
    builtin!(builtins, "change_y", |st, ar| motion::change_pos(st, ar, "y"));
    builtin!(builtins, "edge_bounce", |st, ar| motion::edge_bounce(st, ar));
    builtin!(builtins, "rotation_style", |st, ar| motion::rotation_style(st, ar));
    builtin!(builtins, "direction", |st, _| motion::direction(st));
    builtin!(builtins, "x", |st, _| motion::position(st, "x"));
    builtin!(builtins, "y", |st, _| motion::position(st, "y"));

    // LOOKS
    builtin!(builtins, "hide", |st, _| looks::hide(st));
    builtin!(builtins, "show", |st, _| looks::show(st));
    builtin!(builtins, "say", |st, ar| looks::say(st, ar));
    builtin!(builtins, "think", |st, ar| looks::think(st, ar));
    builtin!(builtins, "switch_costume", |st, ar| looks::switch_costume(st, ar));
    builtin!(builtins, "next_costume", |st, _| looks::next_costume(st));
    builtin!(builtins, "previous_costume", |st, _| looks::previous_costume(st));
    builtin!(builtins,"switch_backdrop",|st, ar| looks::switch_backdrop(st, ar));
    builtin!(builtins, "next_backdrop", |st, _| looks::next_backdrop(st));
    builtin!(builtins, "previous_backdrop", |st, _| looks::previous_backdrop(st));
    builtin!(builtins, "set_scale", |st, ar| looks::set_scale(st, ar));
    builtin!(builtins, "change_scale", |st, ar| looks::change_scale(st, ar));
    builtin!(builtins, "set_effect", |st, ar| looks::set_effect(st, ar));
    builtin!(builtins, "change_effect", |st, ar| looks::change_effect(st, ar));
    builtin!(builtins, "clear_effects", |st, _| looks::clear_effects(st));
    builtin!(builtins, "clear_effect", |st, ar| looks::clear_effect(st, ar));
    builtin!(builtins, "go_to_layer", |st, ar| looks::go_to_layer(st, ar));
    builtin!(builtins, "go_by_layers", |st, ar| looks::go_by_layers(st, ar));
    builtin!(builtins, "costume", |st, _| looks::costume(st));
    builtin!(builtins, "backdrop", |st, _| looks::backdrop(st));
    builtin!(builtins, "size", |st, _| looks::size(st));
    builtin!(builtins, "scale", |st, _| looks::scale(st));
    builtin!(builtins, "bounds", |st, _| looks::bounds(st));
    builtin!(builtins, "layer", |st, _| looks::layer(st));
    builtin!(builtins, "effect", |st, ar| looks::effect(st, ar));

    // SOUNDS
    // builtin!(builtins, "play_sound", |st, ar| sounds::play_sound(st, ar));
    // builtin!(builtins, "stop_all_sounds", |st, _| sounds::stop_all_sounds(st));
    // builtin!(builtins, "stop_sound", |st, ar| sounds::stop_sound(st, ar));
    // builtin!(builtins, "change_sound_effect", |st, ar| sounds::change_sound_effect(st, ar));
    // builtin!(builtins, "set_sound_effect", |st, ar| sounds::set_sound_effect(st, ar));
    // builtin!(builtins, "sound_effect", |st, ar| sounds::sound_effect(st, ar));

    // EVENTS
    builtin!(builtins, "key_down", |st, ar| events::key_down(st, ar));
    builtin!(builtins, "key_pressed", |st, ar| events::key_pressed(st, ar));
    builtin!(builtins, "key_released", |st, ar| events::key_released(st, ar));
    builtin!(builtins, "mouse_button_down", |_, ar| events::mouse_button_down(ar));
    builtin!(builtins, "mouse_button_pressed", |_, ar| events::mouse_button_pressed(ar));
    builtin!(builtins, "mouse_button_released", |_, ar| events::mouse_button_released(ar));
    builtin!(builtins, "mouse_x", |st, _| events::mouse_x(st));
    builtin!(builtins, "mouse_y", |st, _| events::mouse_y(st));
    builtin!(builtins, "sprite_clicked", |st, _| events::sprite_clicked(st));
    builtin!(builtins, "is_backdrop", |st, ar| events::is_backdrop(st, ar));
    builtin!(builtins, "broadcast_id_of", |st, ar| events::broadcast_id_of(st, ar));
    builtin!(builtins, "broadcast", |st, ar| events::broadcast(st, ar));

    // CONTROLS
    builtin!(builtins, "wait", |st, ar| controls::wait(st, ar));
    builtin!(builtins, "stop", |st, ar| controls::stop(st, ar));
    builtin!(builtins, "clone", |st, _| controls::clone(st));
    builtin!(builtins, "delete_clone", |st, ar| controls::delete_clone(st, ar));
    builtin!(builtins, "skip_further_execution_if", |st, ar| controls::skip_further_execution_if(st, ar));

    // DRAWING
    builtin!(builtins, "set_color", |st, ar| drawing::set_color(st, ar));
    builtin!(builtins, "change_r", |st, ar| drawing::change_r(st, ar));
    builtin!(builtins, "change_g", |st, ar| drawing::change_g(st, ar));
    builtin!(builtins, "change_b", |st, ar| drawing::change_b(st, ar));
    builtin!(builtins, "change_a", |st, ar| drawing::change_a(st, ar));
    builtin!(builtins, "line", |st, ar| drawing::line(st, ar));
    builtin!(builtins, "rect", |st, ar| drawing::rect(st, ar));
    builtin!(builtins, "hrect", |st, ar| drawing::hrect(st, ar));
    builtin!(builtins, "circle", |st, ar| drawing::circle(st, ar));
    builtin!(builtins, "hcircle", |st, ar| drawing::hcircle(st, ar));
    builtin!(builtins, "ellipse", |st, ar| drawing::ellipse(st, ar));
    builtin!(builtins, "hellipse", |st, ar| drawing::hellipse(st, ar));
    builtin!(builtins, "polygon", |st, ar| drawing::polygon(st, ar));
    builtin!(builtins, "hpolygon", |st, ar| drawing::hpolygon(st, ar));
    builtin!(builtins, "textured_quad", |_, ar| drawing::textured_quad(ar));
    builtin!(builtins, "stamp", |st, _| drawing::stamp(st));
    builtin!(builtins, "clear_all_stamps", |st, _| drawing::clear_all_stamps(st));
    builtin!(builtins, "r", |st, _| drawing::r(st));
    builtin!(builtins, "g", |st, _| drawing::g(st));
    builtin!(builtins, "b", |st, _| drawing::b(st));
    builtin!(builtins, "a", |st, _| drawing::a(st));

    // WINDOW
    builtin!(builtins, "set_window_width", |st, ar| window::set_window_width(st, ar));
    builtin!(builtins, "set_window_height", |st, ar| window::set_window_height(st, ar));
    builtin!(builtins, "set_window_size", |st, ar| window::set_window_size(st, ar));
    builtin!(builtins, "set_window_state", |st, ar| window::set_window_state(st, ar));
    builtin!(builtins, "set_window_x", |st, ar| window::set_window_x(st, ar));
    builtin!(builtins, "set_window_y", |st, ar| window::set_window_y(st, ar));
    builtin!(builtins, "set_window_position", |st, ar| window::set_window_position(st, ar));
    builtin!(builtins, "pointer_grab", |st, ar| window::pointer_grab(st, ar));
    builtin!(builtins, "window_width", |st, _| window::window_width(st));
    builtin!(builtins, "window_height", |st, _| window::window_height(st));

    builtins
}
