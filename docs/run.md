# Running your Crust Project

To run your Crust project, you need to have Crust installed on your system (obviously). Once you have Crust installed, you can run your project using the following command in your terminal:

=== "Linux / macOS / Windows (WSL)"
    ```bash
    crust path/to/your_project.toml
    ```
=== "Windows"
    ```batch
    crust path\to\your_project.toml
    ```

Replace the path with the path to your project's `project.toml` file. This will start the Crust engine and run your project.
If you don't mention the path to the `project.toml` file, Crust will look for it in the current directory.
