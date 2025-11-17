## `wait(time)`
Waits for a duration.

**Properties:**

- `time` (Number): Time to wait in seconds.

**Returns:** `null`

## `stop(option)`
Stops the specified script(s).

**Properties:**

- `option` (String): What to stop. Available options:
    - `"all"`: Stops all scripts in all sprites.
    - `"this"`: Stops all scripts in the current sprite.
    - `"script"`: Stops the current script.
    - `"other-scripts"`: Stops all other scripts in the current sprite.
    - `"other-sprites-and-scripts"`: Stops all other scripts in all sprites.

**Returns:** `null` 
!!! example
    ```
    setup { // A setup script
        // Something
        stop("this") // Stop this script
        // Stuff here will not be executed
    }
    ```

## `clone()`
Clones the sprite. The clone will execute the `clone_setup` and `clone_update` scripts.
!!! warning
    Clones are also sprites. Excessive cloning can and will lead to performance issues.

**Properties:** none

**Returns:** `null` 
!!! example
    ```
    setup {
        clone()
    }
    update {}
    clone_setup {
        // This will not be printed by the original sprite, but by the clone
        print("Hello, I'm a clone!")
    }
    clone_update {}
    ```

## `delete_clone()` / `delete_clone(cloneid)`
Deletes a clone.

=== "`delete_clone()`"
    Deletes the current clone that is running the function. If called on a non-clone sprite, does nothing.

    **Properties:** none

    **Returns:** `null` 
    !!! example
        ```
        clone_update {
            // Some code
            if dont_want_to_exist_anymore {
                delete_clone() // Delete this clone
            }
        }
        ```

=== "`delete_clone(cloneid)`"
    Deletes the specified clone by its ID from its parent sprite.

    **Properties:**

    - `cloneid` (Number): The ID of the clone to delete. Clone IDs start from 1 and increment for each clone created.

    **Returns:** `null` 
    !!! example
        ```
        setup {
            clone() // Create a clone
            clone() // Create another clone
            delete_clone(1) // Delete the first clone created
        }
        ```

## `skip_further_execution_if(bool)`
Skips further execution of the current script if the condition is true.

**Properties:**

- `bool` (Boolean): Condition to check.

**Returns:** `null` 
!!! example
    ```
    update {
        skip_further_execution_if(score < 10)
        print("Score is 10 or more!")
    }
    ```
