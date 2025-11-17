# Input and Output

## `args()`
Returns the command line arguments passed to the program.

**Properties:** none

**Returns:** `List` - A list of command line arguments as strings.

## `print(values...)`
Prints the given values to the console.

**Properties:**

- `values...` (Any): The values to print.

**Returns:** `null`
!!! example
    ```
    print(
        "Hello, World!",
        42,
        true,
        [1, 2, 3],
        {"key": "value"}
    )
    ```

## `print_raw(values...)`
Prints the given values to the console without the automatic newline at the end and the name of the sprite.

**Properties:**

- `values...` (Any): The values to print.

**Returns:** `null`

## `input(prompt)`
Prompts the user for input and returns the input as a string. The prompt is displayed in the console.

!!! warning
    This function will pause the entire runtime until the user provides input.

**Properties:**

- `prompt` (String): The prompt to display to the user.

**Returns:** `String` - The user's input as a string.

## `write(content)` / `write(content, path)`
Writes the given content to a file.

=== "`write(content)`"
    Creates a file in the `exports` directory in the project root.

    **Properties:**

    - `content` (String): The content to write to the file.

    **Returns:** `null`

=== "`write(content, path)`"
    Creates a file at the specified path. The path is relative to the project root.

    **Properties:**

    - `content` (String): The content to write to the file.
    - `path` (String): The path to the file, relative to the project root

    **Returns:** `null`

## `read(path)`
Reads a file at the specified path.

**Properties:**

- `path` (String): The path to the file, relative to the project root.

**Returns:** `String` - The content of the file as a string.

## `read_binary(path)`
Reads a binary file at the specified path.

**Properties:**

- `path` (String): The path to the binary file, relative to the project root.

**Returns:** `List` - The content of the binary file as a list of byte values (Numbers).

## `parse_image(binary)`
Parses the given binary data as an image and returns a list `[width, height, pixels]`.
The pixels are a list of RGBA values in the format `[r, g, b, a, r, g, b, a, ...]`.

**Properties:**

- `binary` (List): The binary data of the image as a list of byte values (Numbers).

**Returns:** `List` - A list containing the width (Number), height (Number), and pixels (List of Numbers) of the image.

## `screenshot()` / `screenshot(path)`
Takes a screenshot of the current screen and saves it to a file.

=== "`screenshot()`"
    Saves the screenshot to the `exports` directory in the project root.

    **Properties:** none

    **Returns:** `null`

=== "`screenshot(path)`"
    Saves the screenshot to the specified path. The path is relative to the project root.

    **Properties:**

    - `path` (String): The path to save the screenshot, relative to the project root.

    **Returns:** `null`
