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
    builtin!(builtins, "args", |_, pr, _, _, _, _, _| misc::args(pr));
    builtin!(builtins, "print", |sp, _, _, _, _, _, ar| misc::print(sp, ar, false));
    builtin!(builtins, "print_raw", |sp, _, _, _, _, _, ar| misc::print(sp, ar, true));
    builtin!(builtins, "input", |sp, _, _, _, _, _, ar| misc::input(sp, ar));
    builtin!(builtins, "time", |_, _, _, _, _, _, _| misc::time());
    builtin!(builtins, "abs", |_, _, _, _, _, _, ar| misc::math(ar, "abs"));
    builtin!(builtins, "sqrt", |_, _, _, _, _, _, ar| misc::math(ar, "sqrt"));
    builtin!(builtins, "sin", |_, _, _, _, _, _, ar| misc::math(ar, "sin"));
    builtin!(builtins, "cos", |_, _, _, _, _, _, ar| misc::math(ar, "cos"));
    builtin!(builtins, "tan", |_, _, _, _, _, _, ar| misc::math(ar, "tan"));
    builtin!(builtins, "asin", |_, _, _, _, _, _, ar| misc::math(ar, "asin"));
    builtin!(builtins, "acos", |_, _, _, _, _, _, ar| misc::math(ar, "acos"));
    builtin!(builtins, "atan", |_, _, _, _, _, _, ar| misc::math(ar, "atan"));
    builtin!(builtins, "lerp", |_, _, _, _, _, _, ar| misc::lerp(ar));
    builtin!(builtins, "property_of", |_, _, sn, _, _, _, ar| misc::property_of(sn, ar));
    builtin!(builtins, "to_rad", |_, _, _, _, _, _, ar| misc::to_rad(ar));
    builtin!(builtins, "to_deg", |_, _, _, _, _, _, ar| misc::to_deg(ar));
    builtin!(builtins, "clamp", |_, _, _, _, _, _, ar| misc::clamp(ar));
    builtin!(builtins, "len", |_, _, _, _, _, _, ar| misc::len(ar));
    builtin!(builtins, "keys", |_, _, _, _, _, _, ar| misc::key_value(ar, "keys"));
    builtin!(builtins, "values", |_, _, _, _, _, _, ar| misc::key_value(ar, "values"));
    builtin!(builtins, "random", |_, _, _, _, _, _, ar| misc::random(ar));
    builtin!(builtins, "distance", |sp, _, sn, _, _, _, ar| misc::distance(sp, sn, ar, false));
    builtin!(builtins, "distance_to", |sp, _, sn, _, _, _, ar| misc::distance(sp, sn, ar, true));
    builtin!(builtins, "write", |sp, pr, _, _, _, _, ar| misc::write(sp, pr, ar));
    builtin!(builtins, "read", |_, pr, _, _, _, _, ar| misc::read(pr, ar, false));
    builtin!(builtins, "read_binary", |_, pr, _, _, _, _, ar| misc::read(pr, ar, true));
    builtin!(builtins, "parse_image", |_, _, _, _, _, _, ar| misc::parse_image(ar));
    builtin!(builtins, "screenshot", |_, pr, _, _, _, _, ar| misc::screenshot(pr, ar));
    builtin!(builtins, "typeof", |_, _, _, _, _, _, ar| misc::r#typeof(ar));
    builtin!(builtins, "push", |_, _, _, _, _, _, ar| misc::push(ar));
    builtin!(builtins, "pop", |_, _, _, _, _, _, ar| misc::pop(ar));
    builtin!(builtins, "insert", |_, _, _, _, _, _, ar| misc::insert(ar));
    builtin!(builtins, "remove", |_, _, _, _, _, _, ar| misc::remove(ar));
    builtin!(builtins, "extend", |_, _, _, _, _, _, ar| misc::extend(ar));
    builtin!(builtins, "contains", |_, _, _, _, _, _, ar| misc::contains(ar));
    builtin!(builtins, "sort", misc::sort);
    builtin!(builtins, "filter", misc::filter);
    builtin!(builtins, "map", misc::map);
    builtin!(builtins, "split", |_, _, _, _, _, _, ar| misc::split(ar));
    builtin!(builtins, "join", |_, _, _, _, _, _, ar| misc::join(ar));
    builtin!(builtins, "starts_with", |_, _, _, _, _, _, ar| misc::starts_with(ar));
    builtin!(builtins, "ends_with", |_, _, _, _, _, _, ar| misc::ends_with(ar));
    builtin!(builtins, "trim", |_, _, _, _, _, _, ar| misc::trim(ar));
    builtin!(builtins, "range", |_, _, _, _, _, _, ar| misc::range(ar));
    builtin!(builtins, "to_string", |_, _, _, _, _, _, ar| misc::to(ar, "string"));
    builtin!(builtins, "to_number", |_, _, _, _, _, _, ar| misc::to(ar, "number"));
    builtin!(builtins, "to_boolean", |_, _, _, _, _, _, ar| misc::to(ar, "boolean"));
    builtin!(builtins, "to_list", |_, _, _, _, _, _, ar| misc::to(ar, "list"));
    builtin!(builtins, "to_object", |_, _, _, _, _, _, ar| misc::to(ar, "object"));
    builtin!(builtins, "whoami", |sp, _, _, _, _, _, _| misc::whoami(sp));
    builtin!(builtins, "cloneid", |sp, _, _, _, _, _, _| misc::cloneid(sp));
    builtin!(builtins, "frame", |_, _, _, _, _, _, _| misc::frame());
    builtin!(builtins, "delta_time", |_, _, _, _, _, _, _| misc::delta_time());

    // MOTION
    builtin!(builtins, "move", |sp, _, _, _, _, _, ar| motion::r#move(sp, ar));
    builtin!(builtins, "turn_cw", |sp, _, _, _, _, _, ar| motion::turn_cw(sp, ar));
    builtin!(builtins, "turn_ccw", |sp, _, _, _, _, _, ar| motion::turn_ccw(sp, ar));
    builtin!(builtins, "goto", |sp, _, sn, _, _, _, ar| motion::goto(sp, sn, ar));
    builtin!(builtins, "glide", |sp, _, _, _, _, _, ar| motion::glide(sp, ar));
    builtin!(builtins, "point", |sp, _, sn, _, _, _, ar| motion::point(sp, sn, ar));
    builtin!(builtins, "set_x", |sp, _, _, _, _, _, ar| motion::set_pos(sp, ar, "x"));
    builtin!(builtins, "change_x", |sp, _, _, _, _, _, ar| motion::change_pos(sp, ar, "x"));
    builtin!(builtins, "set_y", |sp, _, _, _, _, _, ar| motion::set_pos(sp, ar, "y"));
    builtin!(builtins, "change_y", |sp, _, _, _, _, _, ar| motion::change_pos(sp, ar, "y"));
    builtin!(builtins, "edge_bounce", |sp, _, _, _, _, _, ar| motion::edge_bounce(sp, ar));
    builtin!(builtins, "rotation_style", |sp, _, _, _, _, _, ar| motion::rotation_style(sp, ar));
    builtin!(builtins, "direction", |sp, _, _, _, _, _, _| motion::direction(sp));
    builtin!(builtins, "x", |sp, _, _, _, _, _, _| motion::position(sp, "x"));
    builtin!(builtins, "y", |sp, _, _, _, _, _, _| motion::position(sp, "y"));

    // LOOKS
    builtin!(builtins, "hide", |sp, _, _, _, _, _, _| looks::hide(sp));
    builtin!(builtins, "show", |sp, _, _, _, _, _, _| looks::show(sp));
    builtin!(builtins, "say", |sp, _, _, _, _, _, ar| looks::say(sp, ar));
    builtin!(builtins, "think", |sp, _, _, _, _, _, ar| looks::think(sp, ar));
    builtin!(builtins, "switch_costume", |sp, _, _, _, _, _, ar| looks::switch_costume(sp, ar));
    builtin!(builtins, "next_costume", |sp, _, _, _, _, _, _| looks::next_costume(sp));
    builtin!(builtins, "previous_costume", |sp, _, _, _, _, _, _| looks::previous_costume(sp));
    builtin!(builtins, "switch_backdrop", |_, pr, _, _, _, _, ar| looks::switch_backdrop(pr, ar));
    builtin!(builtins, "next_backdrop", |_, pr, _, _, _, _, _| looks::next_backdrop(pr));
    builtin!(builtins, "previous_backdrop", |_, pr, _, _, _, _, _| looks::previous_backdrop(pr));
    builtin!(builtins, "set_scale", |sp, _, _, _, _, _, ar| looks::set_scale(sp, ar));
    builtin!(builtins, "change_scale", |sp, _, _, _, _, _, ar| looks::change_scale(sp, ar));
    builtin!(builtins, "set_effect", |sp, _, _, _, _, _, ar| looks::set_effect(sp, ar));
    builtin!(builtins, "change_effect", |sp, _, _, _, _, _, ar| looks::change_effect(sp, ar));
    builtin!(builtins, "clear_effects", |sp, _, _, _, _, _, _| looks::clear_effects(sp));
    builtin!(builtins, "clear_effect", |sp, _, _, _, _, _, ar| looks::clear_effect(sp, ar));
    builtin!(builtins, "go_to_layer", |sp, _, _, _, _, _, ar| looks::go_to_layer(sp, ar));
    builtin!(builtins, "go_by_layers", |sp, _, _, _, _, _, ar| looks::go_by_layers(sp, ar));
    builtin!(builtins, "costume", |sp, _, _, _, _, _, _| looks::costume(sp));
    builtin!(builtins, "backdrop", |_, pr, _, _, _, _, _| looks::backdrop(pr));
    builtin!(builtins, "size", |sp, _, _, _, _, _, _| looks::size(sp));
    builtin!(builtins, "scale", |sp, _, _, _, _, _, _| looks::scale(sp));
    builtin!(builtins, "bounds", |sp, _, _, _, _, _, _| looks::bounds(sp));
    builtin!(builtins, "layer", |sp, _, _, _, _, _, _| looks::layer(sp));
    builtin!(builtins, "effect", |sp, _, _, _, _, _, ar| looks::effect(sp, ar));

    // SOUNDS
    // builtin!(builtins, "play_sound", |sp, _, _, _, _, _, ar| sounds::play_sound(sp, ar));
    // builtin!(builtins, "stop_all_sounds", |sp, _, _, _, _, _, _| sounds::stop_all_sounds(sp));
    // builtin!(builtins, "stop_sound", |sp, _, _, _, _, _, ar| sounds::stop_sound(sp, ar));
    // builtin!(builtins, "change_sound_effect", |sp, _, _, _, _, _, ar| sounds::change_sound_effect(sp, ar));
    // builtin!(builtins, "set_sound_effect", |sp, _, _, _, _, _, ar| sounds::set_sound_effect(sp, ar));
    // builtin!(builtins, "sound_effect", |sp, _, _, _, _, _, ar| sounds::sound_effect(sp, ar));

    // EVENTS
    builtin!(builtins, "key_down", |_, _, _, _, _, _, ar| events::key_down(ar));
    builtin!(builtins, "key_pressed", |_, _, _, _, _, _, ar| events::key_pressed(ar));
    builtin!(builtins, "key_released", |_, _, _, _, _, _, ar| events::key_released(ar));
    builtin!(builtins, "mouse_button_down", |_, _, _, _, _, _, ar| events::mouse_button_down(ar));
    builtin!(builtins, "mouse_button_pressed", |_, _, _, _, _, _, ar| events::mouse_button_pressed(ar));
    builtin!(builtins, "mouse_button_released", |_, _, _, _, _, _, ar| events::mouse_button_released(ar));
    builtin!(builtins, "mouse_x", |_, _, _, _, _, _, _| events::mouse_x());
    builtin!(builtins, "mouse_y", |_, _, _, _, _, _, _| events::mouse_y());
    builtin!(builtins, "sprite_clicked", |sp, _, _, _, _, _, _| events::sprite_clicked(sp));
    builtin!(builtins, "is_backdrop", |_, pr, _, _, _, _, ar| events::is_backdrop(pr, ar));
    builtin!(builtins, "broadcast_id_of", |_, pr, _, _, _, _, ar| events::broadcast_id_of(pr, ar));
    builtin!(builtins, "broadcast", |_, pr, _, _, _, _, ar| events::broadcast(pr, ar));

    // CONTROLS
    builtin!(builtins, "wait", |sp, _, _, _, _, _, ar| controls::wait(sp, ar));
    builtin!(builtins, "stop", |sp, _, _, _, _, si, ar| controls::stop(sp, si, ar));
    builtin!(builtins, "clone", |sp, _, _, _, _, _, _| controls::clone(sp));
    builtin!(builtins, "delete_clone", |sp, _, _, _, _, _, ar| controls::delete_clone(sp, ar));
    builtin!(builtins, "skip_further_execution_if", |sp, _, _, _, _, _, ar| controls::skip_further_execution_if(sp, ar));

    // DRAWING
    builtin!(builtins, "set_color", |sp, _, _, _, _, _, ar| drawing::set_color(sp, ar));
    builtin!(builtins, "change_r", |sp, _, _, _, _, _, ar| drawing::change_r(sp, ar));
    builtin!(builtins, "change_g", |sp, _, _, _, _, _, ar| drawing::change_g(sp, ar));
    builtin!(builtins, "change_b", |sp, _, _, _, _, _, ar| drawing::change_b(sp, ar));
    builtin!(builtins, "change_a", |sp, _, _, _, _, _, ar| drawing::change_a(sp, ar));
    builtin!(builtins, "line", |sp, _, _, _, _, _, ar| drawing::line(sp, ar));
    builtin!(builtins, "rect", |sp, _, _, _, _, _, ar| drawing::rect(sp, ar));
    builtin!(builtins, "hrect", |sp, _, _, _, _, _, ar| drawing::hrect(sp, ar));
    builtin!(builtins, "circle", |sp, _, _, _, _, _, ar| drawing::circle(sp, ar));
    builtin!(builtins, "hcircle", |sp, _, _, _, _, _, ar| drawing::hcircle(sp, ar));
    builtin!(builtins, "ellipse", |sp, _, _, _, _, _, ar| drawing::ellipse(sp, ar));
    builtin!(builtins, "hellipse", |sp, _, _, _, _, _, ar| drawing::hellipse(sp, ar));
    builtin!(builtins, "polygon", |sp, _, _, _, _, _, ar| drawing::polygon(sp, ar));
    builtin!(builtins, "hpolygon", |sp, _, _, _, _, _, ar| drawing::hpolygon(sp, ar));
    builtin!(builtins, "textured_quad", |_, _, _, _, _, _, ar| drawing::textured_quad(ar));
    // builtin!(builtins, "stamp", |sp, pr, _, ca, _, _, _| drawing::stamp(sp, pr, ca));
    builtin!(builtins, "clear_all_stamps", |_, pr, _, _, _, _, _| drawing::clear_all_stamps(pr));
    builtin!(builtins, "r", |sp, _, _, _, _, _, _| drawing::r(sp));
    builtin!(builtins, "g", |sp, _, _, _, _, _, _| drawing::g(sp));
    builtin!(builtins, "b", |sp, _, _, _, _, _, _| drawing::b(sp));
    builtin!(builtins, "a", |sp, _, _, _, _, _, _| drawing::a(sp));

    // WINDOW
    // builtin!(builtins, "set_window_width", |_, _, _, _, _, ar| window::set_window_width(ar));
    // builtin!(builtins, "set_window_height", |_, _, _, _, _, ar| window::set_window_height(ar));
    // builtin!(builtins, "set_window_size", |_, _, _, _, _, ar| window::set_window_size(ar));
    // builtin!(builtins, "set_window_state", |_, _, _, _, _, ar| window::set_window_state(ar));
    // builtin!(builtins, "set_window_x", |_, _, _, _, _, ar| window::set_window_x(ar));
    // builtin!(builtins, "set_window_y", |_, _, _, _, _, ar| window::set_window_y(ar));
    // builtin!(builtins, "set_window_position", |_, _, _, _, _, ar| window::set_window_position(ar));
    // builtin!(builtins, "pointer_grab", |_, _, _, _, _, ar| window::pointer_grab(ar));
    // builtin!(builtins, "window_width", |_, _, _, _, _, _| window::window_width());
    // builtin!(builtins, "window_height", |_, _, _, _, _, _| window::window_height());

    builtins
}
