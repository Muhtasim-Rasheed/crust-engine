!!! note
    The words "Stage" and "Backdrop" are used interchangeably in Crust.

!!! note
    As of version 0.3.5, Crust allows you to create projects by writing:
    ```sh
    crust-engine --new my_project
    ```
    This creates a new project with the name `my_project` in the current directory, and initializes it with a default `project.toml` with an empty stage and a single sprite named "default-sprite".

Projects are defined by a `project.toml` file in the root directory. This file contains metadata about the project, such as allowing debug features, the stages, the sprites and their costumes, etc. Here is an example of a `project.toml` file:

```toml
debug_options = [ "show_fps", "show_mouse_pos" ]

[stage]
backdrops = [ "backdrop_0.png" ]

[[sprites]]
name = "title"
code = "title/title.crst"
costumes = [ "title/title.png" ]
sounds = []
x = 0
y = 200
w = 1200
h = 300

[[sprites]]
name = "player"
code = "player/player.crst"
costumes = [ "player/player.png" ]
x = 0
y = 0
w = 150
h = 150

    [[sprites.sounds]]
    name = "jump"
    file = "player/jump.wav"
```

The `project.toml` file is written in TOML format, which is a simple and human-readable configuration file format. You can read more about TOML [here](https://toml.io/en/).

## TOML Structure

All paths mentioned are relative to the `project.toml` file.

- `debug_options`: A list of debug options to enable. Only available options are `show_fps` and `show_mouse_pos`
    - `show_fps`: Shows the current frames per second (FPS)
    - `show_mouse_pos`: Shows the current mouse position on the screen (World coordinates, not screen coordinates)
- `[stage]`: The stage configuration
    - `backdrops`: A list of backdrops for the stage. If the list is empty, the stage will have an empty backdrop
- `[sprites]`: A list of sprites in the project
    - `[[sprites]]`: A sprite
        - `name`: The name of the sprite. Can have spaces and special characters. Case-sensitive.
        - `code`: The path to the sprite's code file.
        - `costumes`: A list of costumes for the sprite. The costumes are images that the sprite can use. If the list is empty, the sprite will have no costumes.
        - `[sprites.sounds]`: A list of sounds for the sprite
            - `[[sprites.sounds]]`: A sound
                - `name`: The name of the sound. Can have spaces and special characters. Case-sensitive.
                - `file`: The path to the sound file. Can only be a `.wav` file.
        - `x`: The x-coordinate of the sprite. Center is 0, left is negative, right is positive.
        - `y`: The y-coordinate of the sprite. Center is 0, up is negative, down is positive.
        - `w`: The width of the sprite in pixels.
        - `h`: The height of the sprite in pixels.
        - `visible`: Whether the sprite is visible. Defaults to `true`. If set to `false`, the sprite will not be rendered on the stage.
        - `layer`: The layer of the sprite. Defaults to `0`. A higher value means the sprite will be rendered on top of sprites with a lower layer value. Also effects the order of the sprite in the sprite list after the first frame.
        - `direction`: The rotation of the sprite in degrees. Defaults to `0`. A positive value rotates the sprite clockwise, a negative value rotates it counter-clockwise.

## Recommended Project Structure

The recommended project structure is as follows:

```
project/
|-- project.toml
|-- stages/
|   |-- backdrop_0.png
|-- sprites/
|   |-- sprite/
|   |   |-- sprite.crst
|   |   |-- sprite.png
|   |   |-- sprite.wav
```
