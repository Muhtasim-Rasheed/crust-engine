## `len(value)`
Returns the length of the given value.

**Properties:**

- `value` (List | String | Object): The value to get the length of. The value is coerced to a list at first.

**Returns:** `Number` - The length of the given value.
!!! example
    ```
    assert len([1, 2, 3, 4, 5]) == 5
    assert len("Hello, World!") == 13
    obj = { "name": "Alice", "age": 30, "city": "New York" }
    assert len(obj) == 3
    ```

## `keys(object)`
Returns a list of keys in the given object.

**Properties:**

- `object` (Object): The object to get the keys from.

**Returns:** `List` - A list of keys in the given object.
!!! example
    ```
    obj = { "name": "Alice", "age": 30, "city": "New York" }
    assert keys(obj) == ["name", "age", "city"]
    ```

## `values(object)`
Returns a list of values in the given object.

**Properties:**

- `object` (Object): The object to get the values from.

**Returns:** `List` - A list of values in the given object.
!!! example
    ```
    obj = { "name": "Alice", "age": 30, "city": "New York" }
    assert values(obj) == ["Alice", 30, "New York"]
    ```

## `typeof(value)`
Returns the type of the given value as a string.

**Properties:**

- `value` (Any): The value to get the type of.

**Returns:** `String` - The type of the given value.
!!! example
    ```
    assert typeof(42) == "number"
    assert typeof("Hello") == "string"
    assert typeof(true) == "boolean"
    ```

## `list_with_capacity(capacity)`
Returns a list with capacity for values.

**Properties:**

- `capacity` (Number): The initial capacity of the list.

**Returns:** `List` - A list with capacity for values.

## `push(list, value)`
Pushes the given value to the end of the list in place.

**Properties:**

- `list` (List): The list to push the value to.
- `value` (Any): The value to push to the list.

**Returns:** `null`
!!! example
    ```
    my_list = [1, 2, 3]
    push(my_list, 4)
    assert my_list == [1, 2, 3, 4]
    ```

## `pop(list)`
Pops the last value from the list in place and returns the popped value.

**Properties:**

- `list` (List): The list to pop the value from.

**Returns:** `Any` - The popped value.
!!! example
    ```
    my_list = [1, 2, 3, 4]
    popped_value = pop(my_list)
    assert my_list == [1, 2, 3]
    assert popped_value == 4
    ```

## `insert(list, index, value)` / `insert(object, key, value)`
Inserts a value into a container in place.

=== "`insert(list, index, value)`"
    Inserts the given value at the specified index in the list in place.

    **Properties:**

    - `list` (List): The list to insert the value into.
    - `index` (Number): The index to insert the value at.
    - `value` (Any): The value to insert into the list.

    **Returns:** `null`
    !!! example
        ```
        my_list = [1, 2, 4]
        insert(my_list, 2, 3)
        assert my_list == [1, 2, 3, 4]
        ```

=== "`insert(object, key, value)`"
    Inserts the given key-value pair into the object in place.

    **Properties:**

    - `object` (Object): The object to insert the key-value pair into.
    - `key` (String): The key to insert into the object.
    - `value` (Any): The value to insert into the object.

    **Returns:** `null`
    !!! example
        ```
        my_object = { "name": "Alice", "age": 30 }
        insert(my_object, "city", "New York")
        assert my_object == { "name": "Alice", "age": 30, "city": "New York" }
        ```

## `remove(list, index)` / `remove(object, key)`
Removes a value from a container in place.

=== "`remove(list, index)`"
    Removes the value at the specified index from the list in place and returns the removed value.

    **Properties:**

    - `list` (List): The list to remove the value from.
    - `index` (Number): The index of the value to remove.

    **Returns:** `Any` - The removed value.
    !!! example
        ```
        my_list = [1, 2, 3, 4]
        removed_value = remove(my_list, 2)
        assert new_list == [1, 2, 4]
        assert removed_value == 3
        ```

=== "`remove(object, key)`"
    Removes the value with the specified key from the object in place and returns the removed value.

    **Properties:**

    - `object` (Object): The object to remove the value from.
    - `key` (String): The key of the value to remove.

    **Returns:** `Any` - The removed value.
    !!! example
        ```
        my_object = { "name": "Alice", "age": 30, "city": "New York" }
        removed_value = remove(my_object, "age")
        assert new_object == { "name": "Alice", "city": "New York" }
        assert removed_value == 30
        ```

## `extend(list1, list2)`
Extends the first list with the second list in place.

**Properties:**

- `list1` (List): The first list to extend.
- `list2` (List): The second list to extend the first list with.

**Returns:** `null`
!!! example
    ```
    list1 = [1, 2, 3]
    list2 = [4, 5, 6]
    extend(list1, list2)
    assert list1 == [1, 2, 3, 4, 5, 6]
    ```

## `contains(list, value)` / `contains(object, key)`
Checks if a container contains a value or key.

=== "`contains(list, value)`"
    Returns true if the list contains the given value, false otherwise.

    **Properties:**

    - `list` (List): The list to check.
    - `value` (Any): The value to check for.

    **Returns:** `Boolean` - `true` if the list contains the given value, `false` otherwise.
    !!! example
        ```
        my_list = [1, 2, 3, 4, 5]
        assert contains(my_list, 3) == true
        assert contains(my_list, 6) == false
        ```

=== "`contains(object, key)`"
    Returns true if the object contains the given key, false otherwise.

    **Properties:**

    - `object` (Object): The object to check.
    - `key` (String): The key to check for.

    **Returns:** `Boolean` - `true` if the object contains the given key, `false` otherwise.
    !!! example
        ```
        my_object = { "name": "Alice", "age": 30 }
        assert contains(my_object, "name") == true
        assert contains(my_object, "city") == false
        ```

## `sort(list, closure)`
Sorts the list in place using the given closure as the comparison function. The closure should take two arguments and return a boolean.

**Properties:**

- `list` (List): The list to sort.
- `closure` (Closure): The comparison function to use for sorting.

**Returns:** `null`
!!! example
    ```
    my_list = [5, 2, 4, 1, 3]
    sort(my_list, fn (a, b) result {
        result = a < b
    })
    assert my_list == [1, 2, 3, 4, 5]
    ```

## `filter(list, closure)`
Filters the list in place using the given closure. The closure should take one argument and return a boolean.
If the closure returns `true`, the element is included in the filtered list; otherwise, it is excluded.

**Properties:**

- `list` (List): The list to filter.
- `closure` (Closure): The filtering function to use.

**Returns:** `null`
!!! example
    ```
    my_list = range(1, 6)
    filter(my_list, fn (x) result {
        result = x % 2 == 0
    })
    assert my_list == [2, 4]
    ```

## `map(list, closure)`
Iterates over the list and applies the given closure to each element, and sets the list to the new list. The closure should take one argument and return a value.

**Properties:**

- `list` (List): The list to map over.
- `closure` (Closure): The mapping function to apply to each element.

**Returns:** `null`
!!! example
    ```
    my_list = range(1, 6)
    map(my_list, fn (x) result {
        result = x * x
    })
    assert my_list == [1, 4, 9, 16, 25]
    ```

## `split(string, delimiter)`
Splits the given string by the specified delimiter and returns a list of substrings.

**Properties:**

- `string` (String): The string to split.
- `delimiter` (String): The delimiter to split the string by.

**Returns:** `List` - A list of substrings.
!!! example
    ```
    my_string = "apple,banana,cherry"
    my_list = split(my_string, ",")
    assert my_list == ["apple", "banana", "cherry"]
    ```

## `join(list, delimiter)`
Joins the elements of the list into a single string, separated by the specified delimiter.

**Properties:**

- `list` (List): The list of strings to join.
- `delimiter` (String): The delimiter to separate the strings.

**Returns:** `String` - The joined string.
!!! example
    ```
    my_list = ["apple", "banana", "cherry"]
    my_string = join(my_list, ", ")
    assert my_string == "apple, banana, cherry"
    ```

## `starts_with(string, prefix)`
Returns true if the given string starts with the specified prefix, false otherwise.

**Properties:**

- `string` (String): The string to check.
- `prefix` (String): The prefix to check for.

**Returns:** `Boolean` - `true` if the string starts with the prefix, `false` otherwise.
!!! example
    ```
    my_string = "Hello, World!"
    assert starts_with(my_string, "Hello") == true
    assert starts_with(my_string, "World") == false
    ```

## `ends_with(string, suffix)`
Returns true if the given string ends with the specified suffix, false otherwise.

**Properties:**

- `string` (String): The string to check.
- `suffix` (String): The suffix to check for.

**Returns:** `Boolean` - `true` if the string ends with the suffix, `false` otherwise.
!!! example
    ```
    my_string = "Hello, World!"
    assert ends_with(my_string, "World!") == true
    assert ends_with(my_string, "Hello") == false
    ```

## `trim(string)`
Trims the whitespace from the beginning and end of the given string and returns the trimmed string.

**Properties:**

- `string` (String): The string to trim.

**Returns:** `String` - The trimmed string.
!!! example
    ```
    my_string = "   Hello, World!   "
    trimmed_string = trim(my_string)
    assert trimmed_string == "Hello, World!"
    ```

## `range(end)` / `range(start, end)` / `range(start, end, step)`
Returns a list of numbers in a specified range.

=== "`range(end)`"
    Returns a list of numbers from 0 to `end - 1`.

    **Properties:**

    - `end` (Number): The end of the range (exclusive).

    **Returns:** `List` - A list of numbers from 0 to `end - 1`.
    !!! example
        ```
        my_list = range(5)
        assert my_list == [0, 1, 2, 3, 4]
        ```

=== "`range(start, end)`"
    Returns a list of numbers from `start` to `end - 1`.

    **Properties:**

    - `start` (Number): The start of the range (inclusive).
    - `end` (Number): The end of the range (exclusive).

    **Returns:** `List` - A list of numbers from `start` to `end - 1`.
    !!! example
        ```
        my_list = range(3, 8)
        assert my_list == [3, 4, 5, 6, 7]
        ```

=== "`range(start, end, step)`"
    Returns a list of numbers from `start` to `end - step`, incrementing by `step`.

    **Properties:**

    - `start` (Number): The start of the range (inclusive).
    - `end` (Number): The end of the range (exclusive).
    - `step` (Number): The increment between each number in the range.

    **Returns:** `List` - A list of numbers from `start` to `end - 1`, incrementing by `step`.
    !!! example
        ```
        my_list = range(2, 10, 2)
        assert my_list == [2, 4, 6, 8]
        ```

## `clone_value(value)`
Creates a deep copy of the given value.

!!! warning
    This function performs a deep copy, meaning that all nested structures are also cloned. Be cautious when cloning large or complex structures, as it may impact performance.

**Properties:**

- `value` (Any): The value to clone.

**Returns:** `Any` - A deep copy of the given value.

## `to_string(value)` / `to_string(number, base)`
Converts a value to a string.

=== "`to_string(value)`"
    Converts the given value to a string.

    **Properties:**

    - `value` (Any): The value to convert to a string.

    **Returns:** `String` - The string representation of the given value.

=== "`to_string(number, base)`"
    Converts the given number to a string in the specified base (2 to 36).

    **Properties:**

    - `number` (Number): The decimal number to convert.
    - `base` (Number): The base to convert the number to (between 2 and 36).

    **Returns:** `String` - The string representation of the number in the specified base.
    !!! example
        ```
        assert to_string(255, 16) == "FF"
        assert to_string(10, 2) == "1010"
        ```

## `to_number(value)`
Converts the given value to a number.

**Properties:**

- `value` (Any): The value to convert to a number.

**Returns:** `Number` - The numeric representation of the given value.

## `to_boolean(value)`
Converts the given value to a boolean.

**Properties:**

- `value` (Any): The value to convert to a boolean.

**Returns:** `Boolean` - The boolean representation of the given value.

## `to_list(value)`
Converts the given value to a list.

**Properties:**

- `value` (Any): The value to convert to a list.

**Returns:** `List` - The list representation of the given value.

## `to_object(value)`
Converts the given value to an object.

**Properties:**

- `value` (Any): The value to convert to an object.

**Returns:** `Object` - The object representation of the given value.
