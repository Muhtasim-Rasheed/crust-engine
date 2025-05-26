## What is Crust (The programming language)?

Crust is Crust's programming language, and it's syntax is similar to Rust. Crust is also coded in Rust, which makes it blazing fast. Crust is a text-based programming language, unlike Scratch, which uses blocks. Crust is designed to be easy to learn and use, while also being powerful enough to create complex games. Crust programs are written in `.crst` files, which are plain text files that can be edited with any text editor.

## Crust Datatypes

There are 6 main datatypes in Crust:

- `Null`: A null value, useful if you want to set a variable to nothing. Example: `null`
- `Number`: A number, can be an integer or a float. Example: `42`, `3.14`
- `String`: A string of characters, enclosed in double quotes. Example: `"Hello, world!"`
- `Boolean`: A boolean value, can be either `true` or `false`. Example: `true`, `false`
- `List`: A list of values, enclosed in square brackets. Example: `[1, 2, 3]`, `["apple", "banana", "cherry"]`
- `Object`: An object, which is a collection of key-value pairs. Example: `{ name: "John", age: 30 }`

## Crust Statements

Crust statements are basically Scratch blocks, but in text form. Crust statements are mainly inline or block statements.

### Inline Statements

Inline statements are single-line statements that perform a specific action. They can be used to assign values, call functions, or import files. There are 4 inline statements in Crust:

- `variable = value`: Assigns a value to a variable. Example: `x = 42`. Variables can change their datatype at any time.
- `global variable = value`: Assigns a value to a global variable. Global variables can be accessed from any sprite. Example: `global score = 0`
- `function_name(arguments)`: Calls a function with the given arguments. Example: `goto("mouse")`
- `import "file.crst"`: Imports a Crust file. The file can contain functions, variables, and other statements. Example: `import "utils.crst"`

To assign a list's or object's value, you can use the following syntax:

- `list[index] = value`: Assigns a value to a specific index in a list. Example: `my_list[0] = "apple"`
- `object["key"] = value`: Assigns a value to a specific key in an object. Example: `my_object["name"] = "John"`

### Block Statements

Block statements are multi-line statements that perform a many actions in one go. They are used to define functions, loops, and conditionals. Block statements are enclosed in curly braces `{}`. There are 8 block statements in Crust:

- `setup { ... }`: The setup block is executed once at the start of the program. It is used to initialize variables, import files, and set up the game environment.
  In library files, variable assignments are appended to the importing file from the setup block. Everything else is ignored.
- `update { ... }`: The update block is executed every frame, and is used to update the game state, handle input, and draw graphics. Libraries ignore the update block.
- `if condition { ... }`: Executes the block if the condition is true. Example: `if x > 0 { print("x is positive") }`
- `if condition { ... } else { ... }`: Executes the first block if the condition is true, otherwise executes the second block. Example: `if x > 0 { print("x is positive") } else { print("x is negative or zero") }`
- `if condition { ... } else if condition { ... } else { ... }`: Executes the first block if the first condition is true, otherwise checks the second condition, and so on. Example: 
```
if x > 0 {
  print("x is positive")
} else if x < 0 {
  print("x is negative")
} else {
  print("x is zero")
}
```
- `while condition { ... }`: Executes the block repeatedly while the condition is true. Example: `while x < 10 { x += 1 }`
- `repeat n { ... }`: Executes the block a specific number of times. Example: `repeat 5 { print("Hello") }`
- `clone_setup { ... }`: The clone setup block is executed once for each clone of a sprite. It is used to initialize variables and set up the clone's state.
- `clone_update { ... }`: The clone update block is executed every frame for each clone of a sprite. It is used to update the clone's state and draw graphics.
- `fn function_name(arguments) return { ... }`: Defines a function with the given name and arguments. The function can be called later using `function_name(arguments)`. Example: 
```
fn add(a, b) result {
    result = a + b
}
```

Note: No `return` statement exists, and the return is embedded in the function header. The function can return a value by assigning it to the `result` variable.

## Expressions

Expressions evaluate to a value and can be used in inline statements or block statements. There are 6 types of expressions in Crust:

- `Value`: A value can be a number, string, boolean, list, or object. Example: `42`, `"Hello"`, `true`, `[1, 2, 3]`, `{ name: "John" }`
- `Identifier`: An identifier is a variable's name. It can be used to access the value of a variable. Example: `x`, `my_list`, `my_object`
- `List Member Access`: Accesses a specific index in a list. Example: `my_list[0]`, `my_list[1]`. Can also be an object key access, which accesses a specific key in an object. Example: `my_object["name"]`, `my_object["age"]`
- `Binary`: A binary expression is an expression that combines two values using an operator. Example: `x + y`, `a < b`, `list1 == list2`, `object1 != object2`
- `Unary`: A unary expression is an expression that applies an operator to a single value. Example: `-x`, `!true`
- `Function Call`: A function call is an expression that calls a function with the given arguments. Action functions are different from Expression functions, as they do not return a value.
    Examples of action functions are `move(10)`, `turn_cw(90)`, and `set_color(255, 0, 0)`. Examples of expression functions are `x()`, `y()`, and `direction()`.
